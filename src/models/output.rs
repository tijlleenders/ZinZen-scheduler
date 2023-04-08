//new module for outputting the result of task_placer in
//whichever format required by front-end
use crate::models::goal::Tag;
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub taskid: usize,
    pub goalid: String,
    pub title: String,
    pub duration: usize,
    pub start: NaiveDateTime,
    pub deadline: NaiveDateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(skip)]
    pub tags: Vec<Tag>,
    #[serde(skip)]
    pub impossible: bool,
}

impl Ord for Output {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start.cmp(&other.start)
    }
}

impl PartialOrd for Output {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct DayOutputFormat {
    pub day: NaiveDate,
    pub outputs: Vec<Output>,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct FinalOutput {
    pub scheduled: Vec<DayOutputFormat>,
    pub impossible: Vec<DayOutputFormat>,
}
