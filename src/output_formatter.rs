//new module for outputting the result of task_placer in
//whichever format required by front-end
use crate::errors::Error;
use crate::task::Task;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
//use serde_json::Result;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Output {
    taskid: usize,
    goalid: usize,
    title: String,
    duration: usize,
    start: NaiveDateTime,
    deadline: NaiveDateTime,
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

pub fn output_formatter(tasks: Vec<Task>) -> Result<Vec<Output>, Error> {
    let mut outputs = Vec::new();
    for task in tasks {
        if task.confirmed_start.is_none() || task.confirmed_deadline.is_none() {
            return Err(Error::NoConfirmedDate(task.id));
        }

        let output = Output {
            taskid: task.id,
            goalid: task.goal_id,
            title: task.title,
            duration: task.duration,
            start: task
                .confirmed_start
                .expect("Checked for None done above so should always be Some."),
            deadline: task
                .confirmed_deadline
                .expect("Checked for None done above so should always be Some."),
        };
        outputs.push(output);
    }
    outputs.sort();
    Ok(outputs)
}
