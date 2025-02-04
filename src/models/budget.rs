use std::{
    fmt::{Debug, Formatter},
    ops::{Add, Sub},
};

use chrono::{Datelike, Duration};
use serde::Deserialize;

use super::{
    calendar::Calendar,
    goal::{Filter, Goal},
};

#[derive(Debug, Clone, Deserialize)]
pub struct Budget {
    pub originating_goal_id: String,
    pub participating_goals: Vec<String>,
    pub time_budgets: Vec<TimeBudget>,
    pub time_filters: Filter,
}
impl Budget {
    pub fn reduce_for_(&mut self, goal: &str, cal_index: usize, cal_index_end: usize) {
        if self.participating_goals.contains(&goal.to_string()) {
            let iterator = self.time_budgets.iter_mut().enumerate();
            for (_, time_budget) in iterator {
                for offset in 0..(cal_index_end - cal_index) {
                    if cal_index + offset >= time_budget.calendar_start_index
                        && cal_index + offset < time_budget.calendar_end_index
                    {
                        time_budget.scheduled += 1;
                    }
                }
            }
        }
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
    }
}

pub fn get_time_budgets_from(calendar: &Calendar, goal: &Goal) -> Vec<TimeBudget> {
    println!("Getting time budgets from goal {}", goal.title);

    let mut time_budgets: Vec<TimeBudget> = vec![];
    //get a time_budget for each day
    for hour_index in 24..calendar.hours() - 24 {
        if (hour_index) % 24 == 0 {
            println!("Day boundary detected at hour_index {:?}", &hour_index);
            if let Some(config) = &goal.budget_config {
                let mut min = config.min_per_day;
                let mut max = config.max_per_day;
                if let Some(filters) = &goal.filters {
                    if filters.on_days.contains(
                        &calendar
                            .start_date_time
                            .sub(Duration::hours(24))
                            .add(Duration::hours(hour_index as i64))
                            .weekday(),
                    ) {
                        //OK
                    } else {
                        min = 0;
                        max = 0;
                    }
                }
                time_budgets.push(TimeBudget {
                    time_budget_type: TimeBudgetType::Day,
                    calendar_start_index: hour_index,
                    calendar_end_index: hour_index + 24,
                    scheduled: 0,
                    min_scheduled: min,
                    max_scheduled: max,
                });
            }
        }
    }

    let mut start_pointer = 24;
    //get a time_budget for each week
    for hour_index in 24..calendar.hours() {
        if (hour_index - 24) % (24 * 7) == 0 && hour_index > 24 {
            println!("Week boundary detected at hour_index {:?}", &hour_index);
            if let Some(config) = &goal.budget_config {
                time_budgets.push(TimeBudget {
                    time_budget_type: TimeBudgetType::Week,
                    calendar_start_index: start_pointer,
                    calendar_end_index: hour_index,
                    scheduled: 0,
                    min_scheduled: config.min_per_week,
                    max_scheduled: config.max_per_week,
                });
            }
            start_pointer = hour_index;
        }
    }
    dbg!(&time_budgets);
    time_budgets
}
