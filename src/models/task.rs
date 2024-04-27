///Tasks are only used for outputting
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

use super::calendar::ImpossibleActivity;

#[derive(Serialize, Debug)]
pub struct FinalTasks {
    pub scheduled: Vec<DayTasks>,
    pub impossible: Vec<ImpossibleActivity>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub taskid: usize,
    pub goalid: String,
    pub title: String,
    pub duration: usize,
    pub start: NaiveDateTime,
    pub deadline: NaiveDateTime,
}

#[derive(Serialize, Debug, Clone)]
pub struct DayTasks {
    pub day: NaiveDate,
    pub tasks: Vec<Task>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TaskCompletedToday {
    pub goalid: String,
    pub start: NaiveDateTime,
    pub deadline: NaiveDateTime,
}
