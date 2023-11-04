//new module for outputting the result of step_placer in
//whichever format required by front-end
use crate::models::goal::Tag;
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use crate::models::date::deserialize_normalized_date;
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub taskid: usize,
    pub goalid: String,
    pub title: String,
    pub duration: usize,
    #[serde(deserialize_with = "deserialize_normalized_date")]
    pub start: NaiveDateTime,
    #[serde(deserialize_with = "deserialize_normalized_date")]
    pub deadline: NaiveDateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(skip)]
    pub tags: Vec<Tag>,
    #[serde(skip)]
    pub impossible: bool,
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start.cmp(&other.start)
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct DayTasks {
    pub day: NaiveDate,
    pub tasks: Vec<Task>,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct FinalTasks {
    pub scheduled: Vec<DayTasks>,
    pub impossible: Vec<DayTasks>,
}
