//new module for outputting the result of task_placer in
//whichever format required by front-end
use crate::goal::Tag;
use crate::options_generator::options_generator;
use crate::task::{ScheduleOption, Task};
use crate::{errors::Error, task::TaskStatus};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Output {
    taskid: usize,
    goalid: String,
    title: String,
    duration: usize,
    start: NaiveDateTime,
    deadline: NaiveDateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    first_conflict_with: Option<String>,
    #[serde(skip)]
    tags: Vec<Tag>,
    #[serde(skip)]
    impossible: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<Vec<ScheduleOption>>,
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
pub struct FinalOutput {
    pub scheduled: Vec<Output>,
    pub impossible: Vec<Output>,
}

pub fn output_formatter(scheduled: Vec<Task>, impossible: Vec<Task>) -> Result<FinalOutput, Error> {
    let mut scheduled_outputs: Vec<Output> = Vec::new();
    let mut impossible_outputs: Vec<Output> = Vec::new();
    let mut scheduled = scheduled;
    if !scheduled[0].clone().tags.contains(&Tag::DoNotSort) {
        scheduled = options_generator(scheduled);
    }
    //convert scheduled tasks to output objects and add to scheduled_outputs vec
    for task in scheduled {
        if task.confirmed_start.is_none() || task.confirmed_deadline.is_none() {
            return Err(Error::NoConfirmedDate(task.title.clone(), task.id));
        }
        scheduled_outputs.push(get_output_from_task(&task));
    }
    //convert impossible tasks to output objects and add to impossible_outputs vec
    for task in impossible {
        //don't report optional tasks
        if task.tags.contains(&Tag::Optional) {
            continue;
        }
        impossible_outputs.push(get_output_from_task(&task));
    }
    //sort and combine the scheduled outputs
    scheduled_outputs.sort();
    combine(&mut scheduled_outputs);
    //assign task ids
    let mut i = 0;
    for task in &mut scheduled_outputs {
        task.taskid = i;
        i += 1;
    }
    //sort and combine the impossible outputs
    impossible_outputs.sort();
    //assign task ids (start from last scheduled id)
    combine(&mut impossible_outputs);
    for task in &mut impossible_outputs {
        task.taskid = i;
        i += 1;
    }
    //create final output object
    let final_ouput = FinalOutput {
        scheduled: scheduled_outputs,
        impossible: impossible_outputs,
    };

    Ok(final_ouput)
}

fn get_output_from_task(task: &Task) -> Output {
    let start = if task.status == TaskStatus::Scheduled {
        task.confirmed_start
            .expect("Checked for None above so should always be Some.")
    } else {
        task.conflicts[0].0.start
    };
    let deadline = if task.status == TaskStatus::Scheduled {
        task.confirmed_deadline
            .expect("Checked for None above so should always be Some.")
    } else {
        task.conflicts[0].0.end
    };
    Output {
        taskid: task.id,
        goalid: task.goal_id.clone(),
        title: task.title.clone(),
        duration: task.duration,
        start,
        deadline,
        first_conflict_with: if task.status == TaskStatus::Impossible {
            Some(task.conflicts[0].1.to_owned())
        } else {
            None
        },
        tags: task.tags.clone(),
        impossible: task.status == TaskStatus::Impossible,
        options: task.options.clone(),
    }
}

//If tasks had been split, they need to be combined into contiguous tasks
//e.g. work(1-2), work(2-3), work(3-4) should be combined into work(1-4).
fn combine(outputs: &mut Vec<Output>) {
    let mut indexes_to_remove = Vec::new();
    let mut i = 0;
    'outer: while i < outputs.len() {
        for j in (i + 1)..outputs.len() {
            if (outputs[j].goalid == outputs[i].goalid && outputs[j].start == outputs[i].deadline)
                || (outputs[j].goalid == outputs[i].goalid
                    && outputs[i].tags.contains(&Tag::FlexDur)
                    && outputs[i].impossible)
            {
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
