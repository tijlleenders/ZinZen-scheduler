use crate::{
    goal::{BudgetType, Tag},
    task::{Task, TaskDTO, TaskStatus},
    time_slot_iterator::TimeSlotsIterator,
    Goal, Repetition, Slot,
};
use chrono::NaiveDateTime;
use serde::Deserialize;
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Deserialize)] //Todo deserialize not needed as this is not in input, only TaskBudget is
pub struct TaskBudgets {
    calendar_start: NaiveDateTime,
    calendar_end: NaiveDateTime,
    goal_id_to_budget_ids: HashMap<String, Vec<String>>,
    pub budget_id_to_budget: HashMap<String, TaskBudget>,
}

#[derive(Debug, Deserialize)]
pub struct TaskBudget {
    task_budget_type: BudgetType,
    pub slot_budgets: Vec<SlotBudget>,
    min: Option<usize>, //only needed once, can't remove as used for subsequent SlotBudget initialization?
    max: Option<usize>, //only needed once, can't remove as used for subsequent SlotBudget initialization?
}

#[derive(Debug, Deserialize)]
pub struct SlotBudget {
    pub slot: Slot,
    pub min: Option<usize>,
    pub max: Option<usize>,
    pub used: usize,
}

impl TaskBudget {
    fn decrement(&mut self, slot: &Slot) {
        for slot_budget in self.slot_budgets.iter_mut() {
            if slot.start.ge(&slot_budget.slot.start) && slot.end.le(&slot_budget.slot.end) {
                slot_budget.used += slot.num_hours();
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
                && slot_budget.used + slot.num_hours() > slot_budget.max.unwrap()
            {
                result = false;
            }
        }
        result
    }

    fn initialize(&mut self, budget_start: NaiveDateTime, budget_end: NaiveDateTime) {
        let mut repetition = Repetition::Weekly(1);
        match self.task_budget_type {
            BudgetType::Weekly => (),
            BudgetType::Daily => repetition = Repetition::DAILY(1),
        }
        let mut time_slot_iterator = TimeSlotsIterator::new(
            budget_start.clone(),
            budget_end.clone(),
            Some(repetition),
            None,
        );
        while let Some(slots) = time_slot_iterator.next() {
            for slot in slots {
                self.slot_budgets.push(SlotBudget {
                    slot: slot.clone(),
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
            calendar_start: calendar_start.clone(),
            calendar_end: calendar_end.clone(),
            goal_id_to_budget_ids: HashMap::new(),
            budget_id_to_budget: HashMap::new(),
        }
    }

    pub fn create_task_budgets_config(&mut self, goals: &mut BTreeMap<String, Goal>) {
        // Todo: create a shadow tasks per budget period that have a tag so the won't be handled by initial call to schedule
        // Once all Tasks are scheduled, if a minimum budget per period is not reached,
        // give the task a duration to get to the minimum per period, remove don't schedule tag, mark ready to schedule and schedule
        // ! How to avoid overlapping budgets? Go from inner to outer budgets (/day first => then /week)
        // This way of shadowing is required so that the min budget scheduling at the end also takes into account the relevant filters and what slots have been taken already
        // It is also necessary to make the tasks being scheduled earlier (Regular and Filler) aware of the slots the budget_min is 'vying for' so they can try to 'keep away'
        if goals.len() == 0 {
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
            while parents_to_go.len() > 0 {
                let children = &goals.get(&parents_to_go[0]).unwrap().children;
                if children.is_some() {
                    for child_id in children.as_ref().unwrap() {
                        let temp_to_update = self.goal_id_to_budget_ids.get_mut(child_id);
                        if temp_to_update.is_some() {
                            temp_to_update.unwrap().push(budget.0.clone());
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
        for (_budget_id, budget) in &mut self.budget_id_to_budget {
            budget.initialize(self.calendar_start, self.calendar_end);
        }
    }

    fn add(&mut self, goal: &Goal) {
        for budget in goal.budgets.clone().unwrap() {
            let budget = TaskBudget {
                task_budget_type: budget.budget_type.clone(),
                slot_budgets: Vec::new(),
                min: budget.min.clone(),
                max: budget.max.clone(),
            };
            self.budget_id_to_budget.insert(goal.id.clone(), budget);
        }
    }

    pub(crate) fn decrement_budgets(&mut self, slot: &Slot, goal_id: &String) -> bool {
        let mut result: bool = false;
        let budget_ids = self.goal_id_to_budget_ids.get(goal_id);
        //decrement all budgets or none => check first - then do
        if budget_ids.is_none() {
            return true;
        }
        let mut decrement_all = true;
        for budget_id in budget_ids.unwrap().iter() {
            let budget = self.budget_id_to_budget.get_mut(budget_id).unwrap();
            if budget.test_decrement(slot) == false {
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

            for time_slots in time_slots_iterator {
                let task_id = *counter;
                *counter += 1;
                if time_slots.len() > 0 {
                    let task = Task::new(TaskDTO {
                        id: task_id,
                        goal_id: goal.id.clone(),
                        title: goal.title.clone(),
                        duration: task_budget.1.min.unwrap(),
                        start: None,
                        deadline: None,
                        calender_start: goal.start.unwrap(),
                        calender_end: goal.deadline.unwrap(),
                        slots: time_slots,
                        status: TaskStatus::BudgetMinWaitingForAdjustment,
                        tags: goal.tags.clone(),
                        after_goals: goal.after_goals.clone(),
                    });
                    tasks_result.push(task);
                } else {
                    panic!("time_slots expected")
                }
            }
        }
        tasks_result
    }
}
