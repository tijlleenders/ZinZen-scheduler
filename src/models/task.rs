///Tasks are only used for outputting
use chrono::{NaiveDate, NaiveDateTime};
use serde::Serialize;

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

#[derive(Serialize, Debug)]
pub struct DayTasks {
    pub day: NaiveDate,
    pub tasks: Vec<Task>,
}
#[derive(Serialize, Debug)]
pub struct FinalTasks {
    pub scheduled: Vec<DayTasks>,
    pub impossible: Vec<DayTasks>,
}
