use std::collections::BTreeMap;

use chrono::NaiveDateTime;

use crate::models::{
    budget::StepBudgets,
    goal::{Goal, Tag},
    slot::Slot,
    slots_iterator::TimeSlotsIterator,
    step::{NewStep, Step, StepStatus},
};

impl StepBudgets {
    pub fn configure_budgets(&mut self, goals: &mut BTreeMap<String, Goal>) {
        // Todo: create a shadow steps per budget period that have a tag so the won't be handled by initial call to schedule
        // Once all Steps are scheduled, if a minimum budget per period is not reached,
        // give the step a duration to get to the minimum per period, remove don't schedule tag, mark ready to schedule and schedule
        // ! How to avoid overlapping budgets? Go from inner to outer budgets (/day first => then /week)
        // This way of shadowing is required so that the min budget scheduling at the end also takes into account the relevant filters and what slots have been taken already
        // It is also necessary to make the steps being scheduled earlier (Regular and Filler) aware of the slots the budget_min is 'vying for' so they can try to 'keep away'
        if goals.is_empty() {
            panic!("expected goals for making StepBudgets");
        }

        let mut goals_to_mark_as_budget: Vec<String> = Vec::new();
        for goal in goals.iter() {
            //Collect budgets per goal
            if goal.1.budgets.is_some() {
                self.add(goal.1);
                goals_to_mark_as_budget.push(goal.0.clone());
            }
        }
        for goal_id in goals_to_mark_as_budget {
            goals.get_mut(&goal_id).unwrap().tags.push(Tag::Budget);
        }
        //For each budget add all descendants
        for (goal_id, _) in &self.budget_map {
            let mut parents_to_go: Vec<String> = vec![goal_id.clone()]; //start with the goal that initiates the budget
            self.budget_ids_map
                .insert(goal_id.clone(), vec![goal_id.clone()]); //add itself for when budget filler min-max need to be checked with budget
            while !parents_to_go.is_empty() {
                let children = &goals.get(&parents_to_go[0]).unwrap().children;
                if children.is_some() {
                    for child_id in children.as_ref().unwrap() {
                        let temp_to_update = self.budget_ids_map.get_mut(child_id);
                        if let Some(temp) = temp_to_update {
                            temp.push(goal_id.clone());
                        } else {
                            self.budget_ids_map
                                .insert(child_id.clone(), vec![goal_id.clone()]);
                        }
                        parents_to_go.push(child_id.clone());
                    }
                }
                parents_to_go.remove(0);
            }
        }
        for budget in self.budget_map.values_mut() {
            budget.initialize(self.calendar_start, self.calendar_end);
        }
    }

    pub(crate) fn is_allowed_by_budget(&mut self, slot: &Slot, goal_id: &String) -> bool {
        let mut result: bool = false;
        let budget_ids = self.budget_ids_map.get(goal_id);
        //decrement all budgets or none => check first - then do
        if budget_ids.is_none() {
            return true;
        }
        let mut decrement_all = true;
        for budget_id in budget_ids.unwrap().iter() {
            let budget = self.budget_map.get_mut(budget_id).unwrap();
            if !budget.test_decrement(slot) {
                decrement_all = false;
                break;
            }
        }
        if decrement_all {
            for budget_id in budget_ids.unwrap().iter() {
                let budget = self.budget_map.get_mut(budget_id).unwrap();
                budget.decrement(slot);
            }
            result = true;
        }
        result
    }

    /// Generate Steps only for goals which have budgets
    pub fn generate_steps(
        &mut self,
        goals: &mut BTreeMap<String, Goal>,
        counter: &mut usize,
    ) -> Vec<Step> {
        let mut steps_result: Vec<Step> = Vec::new();

        //for each budget create a min step (and optional max step) per corresponding time period
        for (goal_id, step_budget) in &self.budget_map {
            let goal = goals.get(goal_id).unwrap();

            let start: NaiveDateTime = goal.start.unwrap();
            let deadline: NaiveDateTime = goal.deadline.unwrap();

            let time_slots_iterator =
                TimeSlotsIterator::new(start, deadline, goal.repeat, goal.filters.clone());

            for timeline in time_slots_iterator {
                let step_id = *counter;
                *counter += 1;
                if !timeline.slots.is_empty() {
                    let duration = step_budget.min.unwrap();

                    let new_step = NewStep {
                        step_id,
                        title: goal.title.clone(),
                        duration,
                        goal: goal.clone(),
                        timeline,
                        status: StepStatus::BudgetMinWaitingForAdjustment,
                        timeframe: None,
                    };

                    let mut step = Step::new(new_step);

                    let splitted_steps = step.split(counter).unwrap();

                    steps_result.extend(splitted_steps);
                } else {
                    panic!("No timeline slots found")
                }
            }
        }
        steps_result
    }
}

// test
#[cfg(test)]
mod tests {

    mod generate_steps {
        use crate::models::{
            budget::{Budget, BudgetType, SlotBudget, StepBudget, StepBudgets},
            goal::{Goal, GoalsMap, Tag},
            repetition::Repetition,
            slot::Slot,
            step::{Step, StepStatus},
        };
        use chrono::Duration;
        use std::collections::HashMap;

        /// Test budgeted goal "4-work" simulating case "budget_with_no_children" but with little horus for simplicity
        #[test]
        fn test_work_in_case_budget_with_no_children() {
            let calendar = Slot::mock(Duration::days(7), 2018, 1, 1, 0, 0);
            let budget_min: usize = 5;

            let mut goal_id_to_budget_ids: HashMap<String, Vec<String>> = HashMap::new();
            goal_id_to_budget_ids.insert("4-work".to_string(), vec!["4-work".to_string()]);

            let mut budget_id_to_budget: HashMap<String, StepBudget> = HashMap::new();
            budget_id_to_budget.insert(
                "4-work".to_string(),
                StepBudget {
                    step_budget_type: BudgetType::Weekly,
                    slot_budgets: vec![SlotBudget {
                        slot: calendar,
                        min: Some(budget_min),
                        max: None,
                        used: 0,
                    }],
                    min: Some(budget_min),
                    max: None,
                },
            );

            let mut step_budgets = StepBudgets {
                calendar_start: calendar.start,
                calendar_end: calendar.end,
                budget_ids_map: goal_id_to_budget_ids,
                budget_map: budget_id_to_budget,
            };

            let work_goal = Goal {
                id: "4-work".to_string(),
                title: "Work".to_string(),
                min_duration: None,
                max_duration: None,
                budgets: Some(vec![Budget {
                    budget_type: BudgetType::Weekly,
                    min: Some(40),
                    max: None,
                }]),
                repeat: Some(Repetition::Weekly(1)),
                start: Some(calendar.start),
                deadline: Some(calendar.end),
                tags: vec![Tag::Budget],
                filters: None,
                children: None,
                after_goals: None,
            };

            let mut goals: GoalsMap = GoalsMap::new();
            goals.insert(work_goal.id.clone(), work_goal);
            let mut counter = 0;
            let result_steps = step_budgets.generate_steps(&mut goals, &mut counter);

            // Expected steps data
            let goal_id: String = "4-work".to_string();
            let title = "Work".to_string();
            let status = StepStatus::ReadyToSchedule;
            let expected_steps: Vec<Step> = vec![
                Step {
                    id: 1,
                    goal_id: goal_id.clone(),
                    title: title.clone(),
                    duration: 1,
                    status: status.clone(),
                    flexibility: 0,
                    start: None,
                    deadline: None,
                    slots: vec![calendar],
                    tags: vec![],
                    after_goals: None,
                },
                Step {
                    id: 2,
                    goal_id: goal_id.clone(),
                    title: title.clone(),
                    duration: 1,
                    status: status.clone(),
                    flexibility: 0,
                    start: None,
                    deadline: None,
                    slots: vec![calendar],
                    tags: vec![],
                    after_goals: None,
                },
                Step {
                    id: 3,
                    goal_id: goal_id.clone(),
                    title: title.clone(),
                    duration: 1,
                    status: status.clone(),
                    flexibility: 0,
                    start: None,
                    deadline: None,
                    slots: vec![calendar],
                    tags: vec![],
                    after_goals: None,
                },
                Step {
                    id: 4,
                    goal_id: goal_id.clone(),
                    title: title.clone(),
                    duration: 1,
                    status: status.clone(),
                    flexibility: 0,
                    start: None,
                    deadline: None,
                    slots: vec![calendar],
                    tags: vec![],
                    after_goals: None,
                },
                Step {
                    id: 5,
                    goal_id: goal_id.clone(),
                    title: title.clone(),
                    duration: 1,
                    status: status.clone(),
                    flexibility: 0,
                    start: None,
                    deadline: None,
                    slots: vec![calendar],
                    tags: vec![],
                    after_goals: None,
                },
            ];

            assert_eq!(result_steps, expected_steps);
            assert_eq!(result_steps[0].duration, expected_steps[0].duration);
            assert_eq!(result_steps[0].status, expected_steps[0].status);
            assert_eq!(result_steps[0].tags, expected_steps[0].tags);

            assert_eq!(result_steps[1].duration, expected_steps[1].duration);
            assert_eq!(result_steps[1].status, expected_steps[1].status);
            assert_eq!(result_steps[1].tags, expected_steps[1].tags);

            assert_eq!(result_steps[2].duration, expected_steps[2].duration);
            assert_eq!(result_steps[2].status, expected_steps[2].status);
            assert_eq!(result_steps[2].tags, expected_steps[2].tags);

            assert_eq!(result_steps[3].duration, expected_steps[3].duration);
            assert_eq!(result_steps[3].status, expected_steps[3].status);
            assert_eq!(result_steps[3].tags, expected_steps[3].tags);

            assert_eq!(result_steps[4].duration, expected_steps[4].duration);
            assert_eq!(result_steps[4].status, expected_steps[4].status);
            assert_eq!(result_steps[4].tags, expected_steps[4].tags);
        }
    }
}
