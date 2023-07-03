use crate::models::{
    budget::StepBudgets,
    goal::{Goal, GoalsMap, Tag},
    input::{Input, StepsToPlace},
    repetition::Repetition,
    step::Step,
};
use chrono::NaiveDateTime;
use std::collections::BTreeMap;

// Todo 2023-05-05  | Move preprocessing Goals into separate module(s) - generating Steps then becomes simple
/// Preprocesses the hierarchy of goals, then for each Goal call Goal.generate_steps
/// Preprocessing involves a number of steps:
/// - add_start_and_end_where_none
/// - add_filler_goals
/// - add_optional_flex_duration_regular_goals
/// - add_optional_flex_number_and_duration_habits_goals
/// - create min and max budgets (step_budgets.create_step_budgets_config)
/// - step_budgets.generate_budget_min_and_max_steps
pub fn generate_steps_to_place(input: Input) -> StepsToPlace {
    let calendar_start = input.calendar_start;
    let calendar_end = input.calendar_end;

    let mut goals = manipulate_input_goals(input);

    let mut step_budgets = StepBudgets::new(&calendar_start, &calendar_end);
    step_budgets.configure_budgets(&mut goals);

    let mut counter: usize = 0;
    let mut steps: Vec<Step> = step_budgets.generate_steps(&mut goals, &mut counter);

    for goal in goals {
        //for regular, filler, optional flexduration regular, optional flexnumber and/or flexduration habit goals
        let steps_for_goal: Vec<Step> =
            goal.1
                .generate_steps(calendar_start, calendar_end, &mut counter);
        steps.extend(steps_for_goal);
    }

    StepsToPlace {
        calendar_start,
        calendar_end,
        steps,
        step_budgets,
    }
}

/// Manipulate input which contains goals with start and end dates.
/// Returns list of Goals after manipulation
fn manipulate_input_goals(input: Input) -> GoalsMap {
    let calendar_start = input.calendar_start;
    let calendar_end = input.calendar_end;
    let goals = input.goals;

    let mut goals: GoalsMap = populate_goal_dates(goals, calendar_start, calendar_end);

    add_filler_goals(&mut goals);

    // TODO 2023-06-17: removed empty function until need it and develop it add_optional_flex_duration_regular_goals(&mut goals);

    generate_flex_weekly_goals(&mut goals);

    goals
}

fn populate_goal_dates(
    mut goals: GoalsMap,
    calendar_start: NaiveDateTime,
    calendar_end: NaiveDateTime,
) -> GoalsMap {
    for goal in goals.iter_mut() {
        goal.1.start.get_or_insert(calendar_start);
        goal.1.deadline.get_or_insert(calendar_end);
    }
    goals
}

/// Generate new goals based on given goals' FlexWeekly repetition
/// - Note: this function generating goals for goals with FlexWeekly repetition only
fn generate_flex_weekly_goals(goals: &mut GoalsMap) {
    let mut generated_goals: GoalsMap = BTreeMap::new();
    for (goal_id, goal) in goals.iter_mut() {
        if let Some(Repetition::FlexWeekly(min, max)) = goal.repeat {
            //Flex repeat goals are handled as follows:
            //If given a goal with 3-5x/week, create 3 goals and 2 extra optional goals
            goal.repeat = Some(Repetition::Weekly(1));

            // Create repeated goals and optional repeated goals
            for number in 1..max {
                // 1.. because we're leaving the initial goal
                let mut template_goal = goal.clone();
                template_goal.id.push_str("-repeat-");

                if number < min {
                    // Repeated goal
                    template_goal.id.push_str(&number.to_string());
                    generated_goals.insert(template_goal.id.clone(), template_goal);
                } else {
                    // Optional repeated goal
                    template_goal.id.push_str("opt-");
                    template_goal.id.push_str(&number.to_string());
                    template_goal.tags.push(Tag::Optional);
                    generated_goals.insert(template_goal.id.clone(), template_goal);
                }
            }
            generated_goals.insert(goal_id.to_owned(), goal.to_owned());
        }
    }

    goals.extend(generated_goals);
}

fn add_filler_goals(goals: &mut GoalsMap) {
    let mut results: GoalsMap = BTreeMap::new();
    let mut ignore: Vec<String> = Vec::new();
    let mut children_to_add: Vec<(String, String)> = Vec::new();
    for goal in goals.iter() {
        if goal.1.children.is_some() && goal.1.budgets.is_none() {
            let mut duration_of_children: usize = 0;
            for child in goal.1.children.clone().unwrap().iter() {
                let child_goal = goals.get(child).unwrap();
                duration_of_children += child_goal.min_duration.unwrap();
            }
            let difference = goal.1.min_duration.unwrap() - duration_of_children;
            if difference > 0 {
                let mut filler_goal = goal.1.clone();
                children_to_add.push((goal.1.id.clone(), filler_goal.id.clone()));
                filler_goal.title.push_str(" filler");
                filler_goal.min_duration = Some(difference);
                filler_goal.tags.push(Tag::Filler);
                results.insert(filler_goal.id.clone(), filler_goal);
                ignore.push(goal.1.id.clone());
            }
        }
    }
    for goal_id_to_ignore in ignore {
        goals
            .get_mut(&goal_id_to_ignore)
            .unwrap()
            .tags
            .push(Tag::IgnoreStepGeneration);
    }
    for parent_child in children_to_add {
        goals
            .get_mut(&parent_child.0)
            .unwrap()
            .children
            .as_mut()
            .unwrap()
            .push(parent_child.1.clone());
    }
    goals.extend(results);
}

#[allow(dead_code)]
fn get_1_hr_goals(goal: Goal) -> Vec<Goal> {
    let mut goals = vec![];
    let dur = goal.min_duration.unwrap();
    for _ in 0..dur {
        let mut g = goal.clone();
        g.min_duration = Some(1);
        goals.push(g);
    }
    goals
}

#[cfg(test)]
mod tests {
    mod generate_flex_weekly_goals {
        use std::collections::BTreeMap;

        use chrono::Duration;

        use crate::{
            models::{
                goal::{Goal, GoalsMap, Tag},
                repetition::Repetition,
                slot::Slot,
            },
            services::preprocess::generate_flex_weekly_goals,
        };

        /// Test generating flex weekly goals based on one step with 1-3/week
        /// ```markdown
        /// Input:
        ///     Goal:
        ///         title: side project
        ///         min_duration: 8
        ///         repeat: 1-3/week
        ///
        /// Output:
        ///     Goal:
        ///         id: 1-repeat-opt-1
        ///         title: side project
        ///         min_duration: 8
        ///         repeat: 1/week
        ///     Goal:
        ///         id: 1-repeat-opt-2
        ///         title: side project
        ///         min_duration: 8
        ///         repeat: 1/week
        ///
        /// ```
        #[test]
        fn test_single_goal() {
            let goal_dates = Slot::mock(Duration::days(31), 2022, 10, 1, 0, 0);

            let mut input_goal = Goal::mock("1", "side project", goal_dates);
            input_goal.min_duration = Some(8);
            input_goal.repeat = Some(Repetition::FlexWeekly(1, 3));

            let mut input_goals: GoalsMap = BTreeMap::new();
            input_goals.insert(input_goal.id.clone(), input_goal);

            generate_flex_weekly_goals(&mut input_goals);

            let mut expected_goal_1 = Goal::mock("1", "side project", goal_dates);
            expected_goal_1.min_duration = Some(8);
            expected_goal_1.repeat = Some(Repetition::Weekly(1));

            let mut expected_goal_2 = expected_goal_1.clone();
            expected_goal_2.id = "1-repeat-opt-1".to_string();
            expected_goal_2.tags.push(Tag::Optional);

            let expected_goal_3 = expected_goal_2.clone();
            expected_goal_2.id = "1-repeat-opt-2".to_string();

            let mut expected_goals = GoalsMap::new();
            expected_goals.insert(expected_goal_1.id.clone(), expected_goal_1);
            expected_goals.insert(expected_goal_2.id.clone(), expected_goal_2);
            expected_goals.insert(expected_goal_3.id.clone(), expected_goal_3);

            assert_eq!(expected_goals, input_goals);
        }
    }
}
