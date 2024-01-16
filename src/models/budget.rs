use std::fmt::{Debug, Formatter};

use serde::Deserialize;

use super::{activity::ActivityType, calendar::Calendar, goal::Goal};

#[derive(Debug, Clone, Deserialize)]
pub struct Budget {
    pub originating_goal_id: String,
    pub participating_goals: Vec<String>,
    pub time_budgets: Vec<TimeBudget>,
}
impl Budget {
    pub fn reduce_for_(&mut self, goal: &str, duration_offset: usize) -> () {
        if self.participating_goals.contains(&goal.to_string()) {
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

    pub(crate) fn is_within_budget(
        &self,
        hour_index: usize,
        offset: usize,
        activity_type: ActivityType,
    ) -> bool {
        let mut budget_cut_off_number = 0;
        let mut is_allowed = true;
        for time_budget in &self.time_budgets {
            match activity_type {
                ActivityType::SimpleGoal => {
                    budget_cut_off_number = time_budget.min_scheduled;
                }
                ActivityType::Budget => {
                    budget_cut_off_number = time_budget.min_scheduled;
                }
            }
            //figure out how many of the hours in hour_index till hour_index + offset are in the time_budget window
            let mut hours_in_time_budget_window = 0;
            for local_offset in 0..offset {
                if (hour_index + local_offset) >= time_budget.calendar_start_index
                    && (hour_index + local_offset) < time_budget.calendar_end_index
                {
                    hours_in_time_budget_window += 1;
                }
            }
            if (hour_index + offset) >= time_budget.calendar_start_index
                && (hour_index + offset) < time_budget.calendar_end_index
                && time_budget.scheduled + hours_in_time_budget_window >= budget_cut_off_number
            {
                is_allowed = false;
            }
        }
        is_allowed
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
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
    let mut start_pointer: usize = 24;
    //get a time_budget for each day
    for hour_index in 24..calendar.hours.capacity() {
        if (hour_index - 24) % 24 == 0 && hour_index > 24 {
            println!("Day boundary detected at hour_index {:?}", &hour_index);
            //TODO: Budgets are being created even if the day is not_on
            //          (example 'work' on sat and sun in default_budgets test case)
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
