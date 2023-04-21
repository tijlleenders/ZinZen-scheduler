use chrono::NaiveDateTime;
use std::collections::BTreeMap;

use crate::models::goal::{Goal, Tag};
use crate::models::input::{Input, TasksToPlace};
use crate::models::repetition::Repetition;
use crate::models::task::Task;
use crate::models::budget::TaskBudgets;

/// # Task Generator
/// Takes an [Input](../input/index.html) and outputs a vector of TaskStatus::Blocked and TaskStatus::ReadyToSchedule [Tasks](../task/index.html).
pub fn task_generator(
    Input {
        calendar_start,
        calendar_end,
        goals,
    }: Input,
) -> TasksToPlace {
    let mut counter: usize = 0;
    let mut tasks: Vec<Task> = vec![];
    let mut goals: BTreeMap<String, Goal> =
        add_start_and_end_where_none(goals, calendar_start, calendar_end);

    add_filler_goals(&mut goals);
    add_optional_flex_duration_regular_goals(&mut goals); //TODO
    add_optional_flex_number_and_duration_habits_goals(&mut goals); //TODO

    let mut task_budgets = TaskBudgets::new(&calendar_start, &calendar_end);
    task_budgets.create_task_budgets_config(&mut goals);

    tasks.extend(task_budgets.generate_budget_min_and_max_tasks(&mut goals, &mut counter));

    for goal in goals {
        //for regular, filler, optional flexduration regular, optional flexnumber and/or flexduration habit goals
        let tasks_for_goal: Vec<Task> =
            goal.1
                .generate_tasks(calendar_start, calendar_end, &mut counter);
        tasks.extend(tasks_for_goal);
    }
    TasksToPlace {
        calendar_start,
        calendar_end,
        tasks,
        task_budgets,
    }
}

fn add_start_and_end_where_none(
    mut goals: BTreeMap<String, Goal>,
    calendar_start: NaiveDateTime,
    calendar_end: NaiveDateTime,
) -> BTreeMap<String, Goal> {
    for goal in goals.iter_mut() {
        if goal.1.start.is_none() {
            goal.1.start = Some(calendar_start);
        }
        if goal.1.deadline.is_none() {
            goal.1.deadline = Some(calendar_end);
        }
    }
    goals
}

fn add_optional_flex_duration_regular_goals(_goals: &mut BTreeMap<String, Goal>) {
    // TODO todo!();
}

fn add_optional_flex_number_and_duration_habits_goals(goals: &mut BTreeMap<String, Goal>) {
    let mut generated_goals: BTreeMap<String, Goal> = BTreeMap::new();
    let goal_ids_to_remove: Vec<String> = Vec::new();
    for goal in goals.iter_mut() {
        if let Some(Repetition::FlexWeekly(min, max)) = goal.1.repeat {
            //Flex repeat goals are handled as follows:
            //If given a goal with 3-5x/week, create 3 goals and 2 extra optional goals
            goal.1.repeat = Some(Repetition::Weekly(1));
            for number in 1..min {
                // 1.. because we're leaving the initial goal
                let mut template_goal = goal.1.clone();
                template_goal.id.push_str("-repeat-");
                template_goal.id.push_str(&number.to_string());
                generated_goals.insert(template_goal.id.clone(), template_goal);
            }
            for number in min..max - 1 {
                let mut template_goal = goal.1.clone();
                template_goal.id.push_str("-repeat-opt-");
                template_goal.id.push_str(&number.to_string());
                template_goal.tags.push(Tag::Optional);
                generated_goals.insert(template_goal.id.clone(), template_goal);
            }
            generated_goals.insert(goal.0.to_owned(), goal.1.to_owned());
        }
    }

    for id in goal_ids_to_remove {
        goals.remove(&id);
    }
    goals.extend(generated_goals);
}

pub fn add_filler_goals(goals: &mut BTreeMap<String, Goal>) {
    let mut results: BTreeMap<String, Goal> = BTreeMap::new();
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
            .push(Tag::IgnoreForTaskGeneration);
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
