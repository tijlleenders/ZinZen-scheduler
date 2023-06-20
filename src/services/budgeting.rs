use std::collections::BTreeMap;

use chrono::NaiveDateTime;

use crate::models::{
    budget::{TaskBudget, TaskBudgets},
    goal::{Goal, Tag},
    slot::Slot,
    slots_iterator::TimeSlotsIterator,
    task::{NewTask, Task, TaskStatus},
    timeline::Timeline,
};

impl TaskBudgets {
    pub fn configure_budgets(&mut self, goals: &mut BTreeMap<String, Goal>) {
        // Todo: create a shadow tasks per budget period that have a tag so the won't be handled by initial call to schedule
        // Once all Tasks are scheduled, if a minimum budget per period is not reached,
        // give the task a duration to get to the minimum per period, remove don't schedule tag, mark ready to schedule and schedule
        // ! How to avoid overlapping budgets? Go from inner to outer budgets (/day first => then /week)
        // This way of shadowing is required so that the min budget scheduling at the end also takes into account the relevant filters and what slots have been taken already
        // It is also necessary to make the tasks being scheduled earlier (Regular and Filler) aware of the slots the budget_min is 'vying for' so they can try to 'keep away'
        if goals.is_empty() {
            panic!("expected goals for making TaskBudgets");
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
        for budget in &self.budget_id_to_budget {
            let mut parents_to_go: Vec<String> = vec![budget.0.clone()]; //start with the goal that initiates the budget
            self.goal_id_to_budget_ids
                .insert(budget.0.clone(), vec![budget.0.clone()]); //add itself for when budget filler min-max need to be checked with budget
            while !parents_to_go.is_empty() {
                let children = &goals.get(&parents_to_go[0]).unwrap().children;
                if children.is_some() {
                    for child_id in children.as_ref().unwrap() {
                        let temp_to_update = self.goal_id_to_budget_ids.get_mut(child_id);
                        if let Some(temp) = temp_to_update {
                            temp.push(budget.0.clone());
                        } else {
                            self.goal_id_to_budget_ids
                                .insert(child_id.clone(), vec![budget.0.clone()]);
                        }
                        parents_to_go.push(child_id.clone());
                    }
                }
                parents_to_go.remove(0);
            }
        }
        for budget in self.budget_id_to_budget.values_mut() {
            budget.initialize(self.calendar_start, self.calendar_end);
        }
    }

    pub(crate) fn is_allowed_by_budget(&mut self, slot: &Slot, goal_id: &String) -> bool {
        let mut result: bool = false;
        let budget_ids = self.goal_id_to_budget_ids.get(goal_id);
        //decrement all budgets or none => check first - then do
        if budget_ids.is_none() {
            return true;
        }
        let mut decrement_all = true;
        for budget_id in budget_ids.unwrap().iter() {
            let budget = self.budget_id_to_budget.get_mut(budget_id).unwrap();
            if !budget.test_decrement(slot) {
                decrement_all = false;
                break;
            }
        }
        if decrement_all {
            for budget_id in budget_ids.unwrap().iter() {
                let budget = self.budget_id_to_budget.get_mut(budget_id).unwrap();
                budget.decrement(slot);
            }
            result = true;
        }
        result
    }

    pub fn generate_budget_min_and_max_tasks(
        &mut self,
        goals: &mut BTreeMap<String, Goal>,
        counter: &mut usize,
    ) -> Vec<Task> {
        let mut tasks_result: Vec<Task> = Vec::new();
        //for each budget create a min task (and optional max task) per corresponding time period

        for task_budget in &self.budget_id_to_budget {
            let goal = goals.get(task_budget.0).unwrap();

            let start: NaiveDateTime = goal.start.unwrap();
            let deadline: NaiveDateTime = goal.deadline.unwrap();

            let time_slots_iterator =
                TimeSlotsIterator::new(start, deadline, goal.repeat, goal.filters.clone());

            for timeline in time_slots_iterator {
                let task_id = *counter;
                *counter += 1;
                if !timeline.slots.is_empty() {
                    // TODO 2023-05-31  | Create a function to split tasks
                    //and return them
                    let duration = task_budget.1.min.unwrap();

                    let new_task = NewTask {
                        task_id,
                        title: goal.title.clone(),
                        duration,
                        goal: goal.clone(),
                        timeline,
                        status: TaskStatus::BudgetMinWaitingForAdjustment,
                        timeframe: None,
                    };

                    let task = Task::new(new_task);

                    tasks_result.push(task);
                } else {
                    panic!("time_slots expected")
                }
            }
        }
        dbg!(&tasks_result);
        tasks_result
    }
}
