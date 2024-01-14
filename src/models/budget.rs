use serde::Deserialize;

use super::{calendar::Calendar, goal::Goal};

//Can a budget have other budgets?

//Budget per day/week in one budget?
//   NO, separate list of budget per period (each day and each week) - per goal-id
//remove datetime completely? YES

// check with budget should be on the whole range of the block, taking into account the min_block

//activity has budgets it belongs to, placer checks each budget ...
//   NO, flex should do that and remove any that are not allowed
// ... is that enough ... or do we need an extra check on placing?
#[derive(Debug, Clone, Deserialize)]
pub struct Budget {
    pub id: String,
    pub participating_goals: Vec<String>,
    pub time_budgets: Vec<TimeBudget>,
}

#[derive(Deserialize, Debug, Clone)]
pub enum TimeBudgetType {
    Day,
    Week,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TimeBudget {
    pub time_budget_type: TimeBudgetType,
    pub calendar_start_index: usize,
    pub calendar_end_index: usize,
    pub scheduled: usize,
    pub min_scheduled: usize,
    pub max_scheduled: usize,
}

pub fn get_time_budgets_from(calendar: &Calendar, goal: &Goal) -> Vec<TimeBudget> {
    let mut time_budgets: Vec<TimeBudget> = vec![];
    let mut start_pointer: usize = 0;
    //get a time_budget for each day
    for hour_index in 0..calendar.hours.capacity() {
        if hour_index % 24 == 0 && hour_index > 0 {
            println!("Day boundary detected at hour_index {:?}", &hour_index);
            time_budgets.push(TimeBudget {
                time_budget_type: TimeBudgetType::Day,
                calendar_start_index: start_pointer,
                calendar_end_index: hour_index,
                scheduled: 0,
                min_scheduled: goal.budget_config.as_ref().unwrap().min_per_day,
                max_scheduled: goal.budget_config.as_ref().unwrap().max_per_day,
            });
            start_pointer = hour_index
        }
    }

    start_pointer = 0;
    //get a time_budget for each week
    for hour_index in 0..calendar.hours.capacity() {
        if hour_index % (24 * 7) == 0 && hour_index > 0 {
            println!("Week boundary detected at hour_index {:?}", &hour_index);
            time_budgets.push(TimeBudget {
                time_budget_type: TimeBudgetType::Week,
                calendar_start_index: start_pointer,
                calendar_end_index: hour_index,
                scheduled: 0,
                min_scheduled: goal.budget_config.as_ref().unwrap().min_per_week,
                max_scheduled: goal.budget_config.as_ref().unwrap().max_per_week,
            });
            start_pointer = hour_index
        }
    }
    dbg!(&time_budgets);
    time_budgets
}
