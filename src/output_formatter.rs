use crate::Slot;
//new module for outputting the result of task_placer in
//whichever format required by front-end
use crate::goal::Tag;
use crate::options_generator::options_generator;
use crate::task::{ScheduleOption, Task};
use crate::{errors::Error, task::TaskStatus};
use chrono::{Datelike, Days, NaiveDate, NaiveDateTime, NaiveTime, Timelike};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::ptr::eq;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Output {
    taskid: usize,
    goalid: String,
    title: String,
    duration: usize,
    start: NaiveDateTime,
    deadline: NaiveDateTime,
    #[serde(skip)]
    after_time: usize,
    #[serde(skip)]
    before_time: usize,
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
pub struct ScheduledOutput {
    pub day: NaiveDate,
    pub outputs: Vec<Output>,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct FinalOutput {
    pub scheduled: Vec<ScheduledOutput>,
    pub impossible: Vec<Output>,
}

pub fn output_formatter(scheduled: Vec<Task>, impossible: Vec<Task>) -> Result<FinalOutput, Error> {
    let mut scheduled_outputs: Vec<Output> = Vec::new();
    let mut impossible_outputs: Vec<Output> = Vec::new();
    let mut scheduled = scheduled;
    let last_index = scheduled.len() - 1;
    if scheduled[0].confirmed_start.is_none() {
        return Err(Error::NoConfirmedDate(
            scheduled[0].title.clone(),
            scheduled[0].id,
        ));
    }
    if scheduled[last_index].confirmed_deadline.is_none() {
        return Err(Error::NoConfirmedDate(
            scheduled[last_index].title.clone(),
            scheduled[last_index].id,
        ));
    }
    let calender_start = scheduled[0].calender_start;
    let calender_end = scheduled[0].calender_end;
    let reserved_start = scheduled[0].confirmed_start;
    let reserved_end = scheduled[last_index].confirmed_deadline;
    let first_free_slot = Slot {
        start: calender_start,
        end: reserved_start
            .unwrap()
            .with_hour(scheduled[0].before_time as u32)
            .unwrap(),
    };
    let last_free_slot = Slot {
        start: reserved_end.unwrap(),
        end: calender_end,
    };
    let first_free_task = Task {
        id: 0,
        goal_id: "free".to_string(),
        title: "free".to_string(),
        duration: first_free_slot.num_hours(),
        status: TaskStatus::Scheduled,
        flexibility: 0,
        start: calender_start,
        deadline: reserved_start
            .unwrap()
            .with_hour(scheduled[0].before_time as u32)
            .unwrap(),
        after_time: calender_start.hour() as usize,
        before_time: reserved_start.unwrap().hour() as usize,
        slots: vec![first_free_slot],
        confirmed_start: Some(calender_start),
        confirmed_deadline: reserved_start
            .unwrap()
            .with_hour(scheduled[0].before_time as u32),
        conflicts: vec![],
        tags: vec![],
        options: None,
        after_goals: None,
        calender_start,
        calender_end,
    };
    let last_free_task = Task {
        id: last_index + 1,
        goal_id: "free".to_string(),
        title: "free".to_string(),
        duration: last_free_slot.num_hours(),
        status: TaskStatus::Scheduled,
        flexibility: 0,
        start: reserved_end.unwrap(),
        deadline: calender_end,
        after_time: reserved_end.unwrap().hour() as usize,
        before_time: calender_end.hour() as usize,
        slots: vec![last_free_slot],
        confirmed_start: reserved_end,
        confirmed_deadline: Some(calender_end),
        conflicts: vec![],
        tags: vec![],
        options: None,
        after_goals: None,
        calender_start,
        calender_end,
    };

    scheduled = options_generator(scheduled);
    if first_free_slot.num_hours() > 0 {
        for task in scheduled.iter_mut() {
            task.id += 1;
        }
        scheduled_outputs.push(get_output_from_task(&first_free_task));
    }

    //convert scheduled tasks to output objects and add to scheduled_outputs vec
    for task in scheduled {
        if task.confirmed_start.is_none() || task.confirmed_deadline.is_none() {
            return Err(Error::NoConfirmedDate(task.title.clone(), task.id));
        }
        scheduled_outputs.push(get_output_from_task(&task));
    }
    if last_free_slot.num_hours() > 0 {
        scheduled_outputs.push(get_output_from_task(&last_free_task));
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
    split_cross_day_task(&mut scheduled_outputs);
    //generate_free_tasks(&mut scheduled_outputs);
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
        scheduled: get_output_with_date(scheduled_outputs, calender_start, calender_end),
        impossible: impossible_outputs,
    };

    Ok(final_ouput)
}

fn get_calender_days(start: NaiveDateTime, end: NaiveDateTime) -> Vec<NaiveDate> {
    let mut date = start.date();
    let days_num = Slot { start, end }.num_hours() / 24;
    let mut days = vec![];
    for _i in 0..days_num {
        days.push(date);
        date = date.checked_add_days(Days::new(1)).unwrap();
    }
    days
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
        after_time: task.after_time,
        before_time: task.before_time,
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

//If a task starts in one day and ends in the next day, it should be splitted into two tasks.
//e.g. A Task 'Sleep' from 22:00-6:00 should be split into two output tasks in output formatter: 22:00-0:00 and 0:00-6:00
fn split_cross_day_task(outputs: &mut Vec<Output>) {
    let mut new_outputs = vec![];
    for task in outputs.iter_mut() {
        if is_cross_day(task) {
            let mut task2 = task.clone();
            task.deadline = task.deadline.with_hour(0).unwrap();
            task2.start = task.deadline.with_hour(0).unwrap();
            task.duration = Slot {
                start: task.start,
                end: task.deadline,
            }
            .num_hours();
            task2.duration -= task.duration;
            new_outputs.push(task.clone());
            if task2.duration > 0 {
                new_outputs.push(task2);
            }
        } else {
            new_outputs.push(task.clone());
        }
    }
    outputs.clear();
    outputs.extend(new_outputs);
}
fn get_output_with_date(
    scheduled: Vec<Output>,
    start: NaiveDateTime,
    end: NaiveDateTime,
) -> Vec<ScheduledOutput> {
    let scheduled = scheduled;

    let mut scheduled_output = vec![];
    for day in get_calender_days(start, end).iter() {
        let mut outputs = scheduled
            .iter()
            .filter(|&e| day.eq(&e.start.date()))
            .cloned()
            .collect::<Vec<Output>>();
        let filled_slots = outputs
            .iter()
            .map(|e| Slot {
                start: e.start,
                end: e.deadline,
            })
            .collect::<Vec<_>>();
        let mut day_slot = day_hour_slots(day);
        for slot in filled_slots.iter() {
            day_slot.retain(|ds| !slot.contains_hour_slot(ds));
        }
        let free_outputs = day_slot
            .iter()
            .map(|s| Output {
                taskid: 0,
                goalid: "free".to_string(),
                title: "free".to_string(),
                duration: s.num_hours(),
                start: s.start,
                deadline: s.end,
                after_time: 0,
                before_time: 23,
                first_conflict_with: None,
                tags: vec![],
                impossible: false,
                options: None,
            })
            .collect::<Vec<_>>();
        outputs.extend(free_outputs);
        outputs.sort_by(|a, b| a.start.cmp(&b.start));

        combine(&mut outputs);
        let mut i = 0;
        for task in &mut outputs {
            task.taskid = i;
            i += 1;
        }
        scheduled_output.push(ScheduledOutput {
            day: day.to_owned(),
            outputs,
        })
    }

    scheduled_output
}
fn generate_free_tasks(outputs: &mut Vec<Output>) {
    let mut new_outputs = vec![];
    for task in outputs.iter_mut() {
        let day_slot = Slot {
            start: task.start.with_hour(0).unwrap(),
            end: task.start.with_hour(23).unwrap(),
        };

        let task_slot = Slot {
            start: task.start,
            end: task.deadline,
        };

        // if !task.goalid.eq("free") {
        //     let mut free_slot = day_slot - task_slot;
        //     let mut free_tasks: VecDeque<Output> = VecDeque::new();
        //     for slot in free_slot.iter_mut() {
        //         let mut free_task = task.clone();
        //         free_task.title = "free".to_string();
        //         free_task.goalid = "free".to_string();
        //         if ((slot.start.hour() as usize) < task.before_time)
        //             && ((slot.end.hour() as usize) > task.before_time)
        //         {
        //             slot.start = slot.start.with_hour(task.before_time as u32).unwrap()
        //         }
        //         if ((slot.start.hour() as usize) < task.after_time)
        //             && ((slot.end.hour() as usize) > task.after_time)
        //         {
        //             slot.end = slot.start.with_hour(task.after_time as u32).unwrap()
        //         }
        //         free_task.start = slot.start;
        //         free_task.deadline = slot.end;

        //         free_task.duration = slot.num_hours();
        //         free_tasks.push_back(free_task);
        //     }
        //     while !free_tasks.is_empty() {
        //         new_outputs.push(free_tasks.pop_front().unwrap());
        //     }
        // }

        new_outputs.push(task.to_owned());
    }
    new_outputs.sort_by(|a, b| a.start.cmp(&b.start));
    outputs.clear();
    outputs.extend(new_outputs);
}
fn is_cross_day(task: &Output) -> bool {
    task.start.day() < task.deadline.day()
}

fn day_hour_slots(day: &NaiveDate) -> Vec<Slot> {
    let mut result = vec![];
    let start = day.and_hms_opt(0, 0, 0).unwrap();
    let end_of_day = start.checked_add_days(Days::new(1)).unwrap();
    for hour in 0..24 {
        result.push(Slot {
            start: start.with_hour(hour).unwrap_or_default(),
            end: start.with_hour(hour + 1).unwrap_or(end_of_day),
        })
    }
    result
}
