use std::ops::{Add, Sub};

use chrono::{Datelike, Duration, NaiveDateTime, Weekday};
use serde::Deserialize;

use super::calendar::Calendar;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Goal {
    pub id: String,
    #[serde(default)]
    pub start: NaiveDateTime,
    #[serde(default)]
    pub deadline: Option<NaiveDateTime>,
    #[serde(rename = "budget")]
    pub budget_config: Option<BudgetConfig>,
    pub filters: Option<Filters>,
    pub min_duration: Option<usize>,
    pub title: String,
    pub children: Option<Vec<String>>,
    pub not_on: Option<Vec<Slot>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Slot {
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Filters {
    pub after_time: usize,
    pub before_time: usize,
    pub on_days: Vec<Weekday>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BudgetConfig {
    pub min_per_day: usize,
    pub max_per_day: usize,
    pub min_per_week: usize,
    pub max_per_week: usize,
}

impl Goal {
    pub fn get_adj_start_deadline(
        &self,
        calendar: &Calendar,
    ) -> (NaiveDateTime, Option<NaiveDateTime>) {
        let mut adjusted_goal_start = self.start;
        if self.start.year() == 1970 || self.start < calendar.start_date_time {
            adjusted_goal_start = calendar.start_date_time;
        }
        let mut adjusted_goal_deadline = self.deadline;
        if self.deadline.is_none() {
            adjusted_goal_deadline = None;
        }

        if self.filters.is_none() {
            return (adjusted_goal_start, adjusted_goal_deadline);
        }

        let filter_option = self.filters.clone().unwrap();
        if filter_option.after_time < filter_option.clone().before_time {
            //normal case
        } else {
            // special case where we know that compatible times cross the midnight boundary
            println!(
                "Special case adjusting start from {:?}",
                &adjusted_goal_start
            );
            adjusted_goal_start = adjusted_goal_start
                .sub(Duration::hours(24))
                .add(Duration::hours(filter_option.after_time as i64));
            println!("... to {:?}", &adjusted_goal_start);
            adjusted_goal_deadline = Some(
                adjusted_goal_start.add(Duration::days(
                    (adjusted_goal_deadline.unwrap_or(calendar.end_date_time)
                        - adjusted_goal_start)
                        .num_days()
                        + 1,
                )),
            );
        }
        (adjusted_goal_start, adjusted_goal_deadline)
    }

    /// Get parent goal of this goal based in provided list of goals
    pub fn get_parent_goal(&self, goals: &[Goal]) -> Option<Goal> {
        let parent_goal = goals.iter().find(|goal| {
            if let Some(childs) = &goal.children {
                childs.contains(&self.id)
            } else {
                false
            }
        });

        parent_goal.cloned()
    }
}
