//new module for outputting the result of task_placer in
//whichever format required by front-end
use crate::errors::Error;
use crate::task::Task;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

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
    let mut outputs: Vec<Output> = Vec::new();
    for i in 0..tasks.len() {
        if tasks[i].confirmed_start.is_none() || tasks[i].confirmed_deadline.is_none() {
            return Err(Error::NoConfirmedDate(tasks[i].title.clone(), tasks[i].id));
        }
        outputs.push(get_output_from_task(&tasks[i]));
    }
    outputs.sort();
    combine(&mut outputs);
    for i in 0..outputs.len() {
        outputs[i].taskid = i;
    }
    Ok(outputs)
}

fn get_output_from_task(task: &Task) -> Output {
    Output {
        taskid: task.id,
        goalid: task.goal_id,
        title: task.title.clone(),
        duration: task.duration,
        start: task
            .confirmed_start
            .expect("Checked for None done above so should always be Some."),
        deadline: task
            .confirmed_deadline
            .expect("Checked for None done above so should always be Some."),
    }
}

//If tasks had been split, they need to be combined into contiguous tasks
//e.g. work(1-2), work(2-3), work(3-4) should be combined into work(1-4).
fn combine(outputs: &mut Vec<Output>) {
    let mut indexes_to_remove = Vec::new();
    let mut i = 0;
    'outer: while i < outputs.len() {
        for j in (i + 1)..outputs.len() {
            if outputs[j].goalid == outputs[i].goalid && outputs[j].start == outputs[i].deadline {
                outputs[i].deadline = outputs[j].deadline;
                outputs[i].duration += outputs[j].duration;
                indexes_to_remove.push(j);
            } else {
                i = j;
                continue 'outer;
            }
        }
        break;
    }

    while !indexes_to_remove.is_empty() {
        outputs.remove(indexes_to_remove.pop().unwrap());
    }
}
