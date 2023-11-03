// new module for outputting the result of step_placer in
// whichever format required by front-end
use crate::errors::Error;
use crate::models::goal::Tag;
use crate::models::input::PlacedSteps;
use crate::models::output::{DayTasks, FinalTasks, Task};
use crate::models::slot::Slot;
use crate::models::step::{Step, StepStatus};
use crate::services::splitters::split_crossed_tasks;
use chrono::{Days, NaiveDate, NaiveDateTime, Timelike};

/// Formatting, sorting, and merging (contiguous) incoming steps into
/// a FinalTasks data-structure to be returned back.
/// This data-structure will be printed out in the `output.json`
pub fn output_formatter(mut placed_steps: PlacedSteps) -> Result<FinalTasks, Error> {
    let mut scheduled_tasks: Vec<Task> = Vec::new();
    let mut impossible_tasks: Vec<Task> = Vec::new();

    for step in placed_steps.steps.iter_mut() {
        match step.status {
            StepStatus::Scheduled => {
                //convert scheduled steps to output objects and add to scheduled_outputs vec
                if step.start_date.is_none() || step.end_date.is_none() {
                    return Err(Error::NoConfirmedDate(step.title.clone(), step.id));
                }
                scheduled_tasks.push(get_task_from_step(
                    step,
                    placed_steps.start_date,
                    placed_steps.end_date,
                ));
            }
            StepStatus::Impossible => {
                //convert impossible steps to output objects and add to impossible_outputs vec
                //don't report optional steps
                if step.tags.contains(&Tag::Optional) {
                    continue;
                }
                impossible_tasks.push(get_task_from_step(
                    step,
                    placed_steps.start_date,
                    placed_steps.end_date,
                ));
            }
            StepStatus::Uninitialized => {
                panic!("no uninitialized steps should be present in placed_steps")
            }
            StepStatus::Blocked => panic!("no Blocked steps should be present in placed_steps"),
            StepStatus::ReadyToSchedule => {
                panic!("no ReadyToSchedule steps should be present in placed_steps")
            }
        }
    }

    //sort and combine the scheduled outputs
    scheduled_tasks.sort();

    combine(&mut scheduled_tasks);
    split_crossed_tasks(&mut scheduled_tasks);
    generate_free_tasks(
        &mut scheduled_tasks,
        placed_steps.start_date,
        placed_steps.end_date,
    );
    //assign task ids
    let mut i = 0;
    for task in &mut scheduled_tasks {
        task.taskid = i;
        i += 1;
    }
    //sort and combine the impossible outputs
    impossible_tasks.sort();
    //assign step ids (start from last scheduled id)
    combine(&mut impossible_tasks);
    for task in &mut impossible_tasks {
        task.taskid = i;
        i += 1;
    }
    //create FinalTasks object
    let final_tasks = FinalTasks {
        scheduled: get_tasks_with_date(
            scheduled_tasks,
            placed_steps.start_date,
            placed_steps.end_date,
        ),
        impossible: get_tasks_with_date(
            impossible_tasks,
            placed_steps.start_date,
            placed_steps.end_date,
        ),
    };

    print!("{}", serde_json::to_string_pretty(&final_tasks).unwrap());

    Ok(final_tasks)
}

/// Get list of days between given start and end dates
fn get_calendar_days(start: NaiveDateTime, end: NaiveDateTime) -> Vec<NaiveDate> {
    let mut date = start.date();
    let days_num = Slot { start, end }.duration_as_hours() / 24;
    let mut days = vec![];
    for _i in 0..days_num {
        days.push(date);
        date = date.checked_add_days(Days::new(1)).unwrap();
    }
    days
}

/// Get a task from a given Step
fn get_task_from_step(
    step: &Step,
    start_date: NaiveDateTime,
    end_date: NaiveDateTime,
) -> Task {
    match step.status {
        StepStatus::Scheduled => Task {
            taskid: step.id,
            goalid: step.goal_id.clone(),
            title: step.title.clone(),
            duration: step.duration,
            start: step.start_date.unwrap(),
            deadline: step.end_date.unwrap(),
            tags: step.tags.clone(),
            impossible: false,
        },
        StepStatus::Impossible => Task {
            taskid: step.id,
            goalid: step.goal_id.clone(),
            title: step.title.clone(),
            duration: step.duration,
            start: start_date,
            deadline: end_date,
            tags: step.tags.clone(),
            impossible: true,
        },
        StepStatus::Uninitialized => todo!(),
        StepStatus::Blocked => todo!(),
        StepStatus::ReadyToSchedule => todo!(),
    }
}

//If tasks had been split, they need to be combined into contiguous tasks
//e.g. work(1-2), work(2-3), work(3-4) should be combined into work(1-4).
fn combine(tasks: &mut Vec<Task>) {
    let mut indexes_to_remove = Vec::new();
    let mut i = 0;
    'outer: while i < tasks.len() {
        for j in (i + 1)..tasks.len() {
            if (tasks[j].goalid == tasks[i].goalid && tasks[j].start == tasks[i].deadline)
                || (tasks[j].goalid == tasks[i].goalid
                    && tasks[i].tags.contains(&Tag::FlexDur)
                    && tasks[i].impossible)
            {
                tasks[i].deadline = tasks[j].deadline;
                tasks[i].duration += tasks[j].duration;
                indexes_to_remove.push(j);
            } else {
                i = j;
                continue 'outer;
            }
        }
        break;
    }

    while let Some(element) = indexes_to_remove.pop() {
        tasks.remove(element);
    }
}

/// Filter tasks with date and return sorted tasks
fn get_tasks_with_date(
    tasks: Vec<Task>,
    start: NaiveDateTime,
    end: NaiveDateTime,
) -> Vec<DayTasks> {
    let mut result = vec![];
    for day in get_calendar_days(start, end).iter() {
        let mut filtered_tasks = tasks
            .iter()
            .filter(|&e| day.eq(&e.start.date()))
            .cloned()
            .collect::<Vec<Task>>();
        filtered_tasks.sort_by(|a, b| a.start.cmp(&b.start));

        combine(&mut filtered_tasks);
        result.push(DayTasks {
            day: day.to_owned(),
            tasks: filtered_tasks,
        })
    }

    result
}

/// Generate tasks for free slots without any tasks
fn generate_free_tasks(outputs: &mut Vec<Task>, start: NaiveDateTime, end: NaiveDateTime) {
    let mut new_outputs = vec![];
    for day in get_calendar_days(start, end).iter() {
        let mut day_outputs = outputs
            .iter()
            .filter(|&e| day.eq(&e.start.date()))
            .cloned()
            .collect::<Vec<Task>>();
        let filled_slots = day_outputs
            .iter()
            .map(|e| Slot {
                start: e.start,
                end: e.deadline,
            })
            .collect::<Vec<_>>();
        let mut day_slot = day_hour_slots(day);
        for slot in filled_slots.iter() {
            day_slot.retain(|ds| !slot.is_contains_slot(ds));
        }
        let free_outputs = day_slot
            .iter()
            .map(|s| Task {
                taskid: 0,
                goalid: "free".to_string(),
                title: "free".to_string(),
                duration: s.duration_as_hours(),
                start: s.start,
                deadline: s.end,
                tags: vec![],
                impossible: false,
            })
            .collect::<Vec<_>>();
        day_outputs.extend(free_outputs);
        day_outputs.sort_by(|a, b| a.start.cmp(&b.start));

        combine(&mut day_outputs);

        new_outputs.extend(day_outputs);
    }
    new_outputs.sort_by(|a, b| a.start.cmp(&b.start));
    outputs.clear();
    outputs.extend(new_outputs);
}

/// Return list of hourly slots for a day starting given datetime
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
