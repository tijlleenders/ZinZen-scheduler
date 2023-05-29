use std::collections::{BTreeMap, HashMap};

use super::{BudgetType, SlotBudget, TaskBudget, TaskBudgets};
use crate::models::{
    goal::{Goal, Tag},
    repetition::Repetition,
    slot::Slot,
    slots_iterator::TimeSlotsIterator,
    task::{NewTask, Task, TaskStatus},
};
use chrono::NaiveDateTime;

impl TaskBudget {
    fn decrement(&mut self, slot: &Slot) {
        for slot_budget in self.slot_budgets.iter_mut() {
            if slot.start.ge(&slot_budget.slot.start) && slot.end.le(&slot_budget.slot.end) {
                slot_budget.used += slot.duration_as_hours();
                if slot_budget.max.is_some() && slot_budget.used > slot_budget.max.unwrap() {
                    panic!("allocated more than max SlotBudget!");
                }
            }
        }
    }

    fn test_decrement(&self, slot: &Slot) -> bool {
        let mut result = true;
        for slot_budget in self.slot_budgets.iter() {
            if slot.start.ge(&slot_budget.slot.start)
                && slot.end.le(&slot_budget.slot.end)
                && slot_budget.max.is_some()
                && slot_budget.used + slot.duration_as_hours() > slot_budget.max.unwrap()
            {
                result = false;
            }
        }
        result
    }

    fn initialize(&mut self, budget_start: NaiveDateTime, budget_end: NaiveDateTime) {
        let mut repetition: Repetition = Repetition::Weekly(1);
        match self.task_budget_type {
            BudgetType::Weekly => (),
            BudgetType::Daily => repetition = Repetition::DAILY(1),
        }
        let time_slot_iterator =
            TimeSlotsIterator::new(budget_start, budget_end, Some(repetition), None);
        for timeline in time_slot_iterator {
            for slot in timeline.slots {
                self.slot_budgets.push(SlotBudget {
                    slot,
                    min: self.min,
                    max: self.max,
                    used: 0,
                });
            }
        }
    }
}

impl TaskBudgets {
    pub fn new(calendar_start: &NaiveDateTime, calendar_end: &NaiveDateTime) -> Self {
        Self {
            calendar_start: *calendar_start,
            calendar_end: *calendar_end,
            goal_id_to_budget_ids: HashMap::new(),
            budget_id_to_budget: HashMap::new(),
        }
    }

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

    fn add(&mut self, goal: &Goal) {
        for budget in goal.budgets.clone().unwrap() {
            let budget = TaskBudget {
                task_budget_type: budget.budget_type.clone(),
                slot_budgets: Vec::new(),
                min: budget.min,
                max: budget.max,
            };
            self.budget_id_to_budget.insert(goal.id.clone(), budget);
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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_initialize_weekly_for_a_month() {
        // Test that the weekly budget is initialized correctly
        //for a month with 5 weeks

        let mut task_budget = TaskBudget {
            task_budget_type: BudgetType::Weekly,
            max: Some(10),
            min: Some(1),
            slot_budgets: vec![],
        };
        let timeframe = Slot::mock(Duration::days(31), 2023, 5, 1, 0, 0);
        dbg!(&timeframe);
        let start_date = timeframe.start;
        let end_date = timeframe.end;

        dbg!(&task_budget);
        task_budget.initialize(start_date, end_date);
        dbg!(&task_budget);

        assert_eq!(task_budget.slot_budgets.len(), 5);
        for slot_budget in task_budget.slot_budgets.iter() {
            assert_eq!(slot_budget.used, 0);
            assert_eq!(slot_budget.min, Some(1));
            assert_eq!(slot_budget.max, Some(10));
        }
    }
}
