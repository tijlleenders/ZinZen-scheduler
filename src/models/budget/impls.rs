use std::collections::HashMap;

use super::{BudgetType, SlotBudget, StepBudget, StepBudgets};
use crate::models::{
    goal::Goal, repetition::Repetition, slot::Slot, slots_iterator::TimeSlotsIterator,
};
use chrono::NaiveDateTime;

impl StepBudget {
    pub fn decrement(&mut self, slot: &Slot) {
        for slot_budget in self.slot_budgets.iter_mut() {
            if slot.start.ge(&slot_budget.slot.start) && slot.end.le(&slot_budget.slot.end) {
                slot_budget.used += slot.duration_as_hours();
                if slot_budget.max.is_some() && slot_budget.used > slot_budget.max.unwrap() {
                    panic!("allocated more than max SlotBudget!");
                }
            }
        }
    }

    pub fn test_decrement(&self, slot: &Slot) -> bool {
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

    pub fn initialize(&mut self, budget_start: NaiveDateTime, budget_end: NaiveDateTime) {
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

impl StepBudgets {
    pub fn new(calendar_start: &NaiveDateTime, calendar_end: &NaiveDateTime) -> Self {
        Self {
            calendar_start: *calendar_start,
            calendar_end: *calendar_end,
            goal_id_to_budget_ids: HashMap::new(),
            budget_id_to_budget: HashMap::new(),
        }
    }

    pub fn add(&mut self, goal: &Goal) {
        for budget in goal.budgets.clone().unwrap() {
            let budget = StepBudget {
                task_budget_type: budget.budget_type.clone(),
                slot_budgets: Vec::new(),
                min: budget.min,
                max: budget.max,
            };
            self.budget_id_to_budget.insert(goal.id.clone(), budget);
        }
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

        let mut task_budget = StepBudget {
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
