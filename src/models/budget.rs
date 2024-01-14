use std::fmt::{Debug, Formatter};

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
    pub originating_goal_id: String,
    pub participating_goals: Vec<String>,
    pub time_budgets: Vec<TimeBudget>,
}
impl Budget {
    pub fn reduce_for_(&mut self, goal: &str, duration_offset: usize) -> () {
        if self.participating_goals.contains(&goal.clone().to_string()) {
            let mut time_budgets_updated = self.time_budgets.clone();
            for time_budget_index in 0..self.time_budgets.len() {
                if duration_offset >= self.time_budgets[time_budget_index].calendar_start_index
                    && duration_offset < self.time_budgets[time_budget_index].calendar_end_index
                {
                    time_budgets_updated[time_budget_index].scheduled += 1
                }
            }
            self.time_budgets = time_budgets_updated;
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub enum TimeBudgetType {
    Day,
    Week,
}

#[derive(Clone, Deserialize)]
pub struct TimeBudget {
    pub time_budget_type: TimeBudgetType,
    pub calendar_start_index: usize,
    pub calendar_end_index: usize,
    pub scheduled: usize,
    pub min_scheduled: usize,
    pub max_scheduled: usize,
}
impl TimeBudget {
    pub(crate) fn reduce_by(&self, hours: usize) -> () {
        println!("reducing by 1!");
        ()
    }
}

impl Debug for TimeBudget {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "\n{:?} budget from index {:?}-{:?}: Scheduled {:?} / {:?}-{:?}\n",
            &self.time_budget_type,
            &self.calendar_start_index,
            &self.calendar_end_index,
            &self.scheduled,
            &self.min_scheduled,
            &self.max_scheduled
        )
        .unwrap();
        Ok(())
    }
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

    start_pointer = 24;
    //get a time_budget for each week
    for hour_index in 24..calendar.hours.capacity() {
        if (hour_index - 24) % (24 * 7) == 0 && hour_index > 24 {
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
