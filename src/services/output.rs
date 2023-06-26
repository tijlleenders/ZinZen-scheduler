//new module for outputting the result of step_placer in
//whichever format required by front-end
use crate::errors::Error;
use crate::models::goal::Tag;
use crate::models::input::PlacedSteps;
use crate::models::output::{DayOutputFormat, FinalOutput, Output};
use crate::models::slot::Slot;
use crate::models::step::{Step, StepStatus};
use chrono::{Datelike, Days, NaiveDate, NaiveDateTime, Timelike};

/// Formatting, sorting, and merging (contiguous) incoming steps into
/// a FinalOutput data-structure to be returned back.
/// This data-structure will be printed out in the `output.json`
pub fn output_formatter(mut placed_steps: PlacedSteps) -> Result<FinalOutput, Error> {
    let mut scheduled_outputs: Vec<Output> = Vec::new();
    let mut impossible_outputs: Vec<Output> = Vec::new();

    for step in placed_steps.steps.iter_mut() {
        match step.status {
            StepStatus::Scheduled => {
                //convert scheduled steps to output objects and add to scheduled_outputs vec
                if step.start.is_none() || step.deadline.is_none() {
                    return Err(Error::NoConfirmedDate(step.title.clone(), step.id));
                }
                scheduled_outputs.push(get_output_from_step(
                    step,
                    placed_steps.calendar_start,
                    placed_steps.calendar_end,
                ));
            }
            StepStatus::Impossible => {
                //convert impossible steps to output objects and add to impossible_outputs vec
                //don't report optional steps
                if step.tags.contains(&Tag::Optional) {
                    continue;
                }
                impossible_outputs.push(get_output_from_step(
                    step,
                    placed_steps.calendar_start,
                    placed_steps.calendar_end,
                ));
            }
            StepStatus::Uninitialized => {
                panic!("no uninitialized steps should be present in placed_steps")
            }
            StepStatus::Blocked => panic!("no Blocked steps should be present in placed_steps"),
            StepStatus::ReadyToSchedule => {
                panic!("no ReadyToSchedule steps should be present in placed_steps")
            }
            StepStatus::BudgetMinWaitingForAdjustment => {
                panic!("no BudgetMinWaitingForAdjustment steps should be present in placed_steps")
            }
        }
    }

    //sort and combine the scheduled outputs
    scheduled_outputs.sort();

    combine(&mut scheduled_outputs);
    split_cross_day_step(&mut scheduled_outputs);
    generate_free_tasks(
        &mut scheduled_outputs,
        placed_steps.calendar_start,
        placed_steps.calendar_end,
    );
    //assign task ids
    let mut i = 0;
    for task in &mut scheduled_outputs {
        task.taskid = i;
        i += 1;
    }
    //sort and combine the impossible outputs
    impossible_outputs.sort();
    //assign step ids (start from last scheduled id)
    combine(&mut impossible_outputs);
    for task in &mut impossible_outputs {
        task.taskid = i;
        i += 1;
    }
    //create final output object
    let final_ouput = FinalOutput {
        scheduled: get_output_with_date(
            scheduled_outputs,
            placed_steps.calendar_start,
            placed_steps.calendar_end,
        ),
        impossible: get_output_with_date(
            impossible_outputs,
            placed_steps.calendar_start,
            placed_steps.calendar_end,
        ),
    };

    print!("{}", serde_json::to_string_pretty(&final_ouput).unwrap());

    Ok(final_ouput)
}

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

fn get_output_from_step(
    step: &mut Step,
    calendar_start: NaiveDateTime,
    calendar_end: NaiveDateTime,
) -> Output {
    match step.status {
        StepStatus::Scheduled => Output {
            taskid: step.id,
            goalid: step.goal_id.clone(),
            title: step.title.clone(),
            duration: step.duration,
            start: step.start.unwrap(),
            deadline: step.deadline.unwrap(),
            tags: step.tags.clone(),
            impossible: false,
        },
        StepStatus::Impossible => Output {
            taskid: step.id,
            goalid: step.goal_id.clone(),
            title: step.title.clone(),
            duration: step.duration,
            start: calendar_start,
            deadline: calendar_end,
            tags: step.tags.clone(),
            impossible: true,
        },
        StepStatus::Uninitialized => todo!(),
        StepStatus::Blocked => todo!(),
        StepStatus::ReadyToSchedule => todo!(),
        StepStatus::BudgetMinWaitingForAdjustment => todo!(),
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

//If a step starts in one day and ends in the next day, it should be splitted into two steps.
//e.g. A Step 'Sleep' from 22:00-6:00 should be split into two output steps in output formatter: 22:00-0:00 and 0:00-6:00
fn split_cross_day_step(outputs: &mut Vec<Output>) {
    dbg!(&outputs);
    /*
    TODO 2023-06-04  | Debug note | case bug_215
    - For param "outputs", it contains wrong duration for steps "hurdle" and "sleep".
    - Attention to function "is_cross_day" which comparison need to be enhanced. Check output.title:"hurdle"
    - Attention to code line "step2.duration -= step.duration;" which seems is not accurate and also affected by function "is_cross_day"
    */

    let mut new_outputs = vec![];
    for step in outputs.iter_mut() {
        if is_cross_day(step) {
            let mut step2 = step.clone();
            step.deadline = step.deadline.with_hour(0).unwrap();
            step2.start = step.deadline.with_hour(0).unwrap();
            step.duration = Slot {
                start: step.start,
                end: step.deadline,
            }
            .duration_as_hours();

            dbg!(&step, &step2);

            step2.duration -= step.duration;
            new_outputs.push(step.clone());
            if step2.duration > 0 {
                new_outputs.push(step2);
            }
        } else {
            new_outputs.push(step.clone());
            dbg!(&new_outputs);
        }
    }

    dbg!(&new_outputs);
    outputs.clear();
    outputs.extend(new_outputs);
}

fn get_output_with_date(
    output: Vec<Output>,
    start: NaiveDateTime,
    end: NaiveDateTime,
) -> Vec<DayOutputFormat> {
    let mut result = vec![];
    for day in get_calendar_days(start, end).iter() {
        let mut outputs = output
            .iter()
            .filter(|&e| day.eq(&e.start.date()))
            .cloned()
            .collect::<Vec<Output>>();
        outputs.sort_by(|a, b| a.start.cmp(&b.start));

        combine(&mut outputs);
        result.push(DayOutputFormat {
            day: day.to_owned(),
            outputs,
        })
    }

    result
}

fn generate_free_tasks(outputs: &mut Vec<Output>, start: NaiveDateTime, end: NaiveDateTime) {
    let mut new_outputs = vec![];
    for day in get_calendar_days(start, end).iter() {
        let mut day_outputs = outputs
            .iter()
            .filter(|&e| day.eq(&e.start.date()))
            .cloned()
            .collect::<Vec<Output>>();
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
            .map(|s| Output {
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

fn is_cross_day(task: &Output) -> bool {
    dbg!(&task);
    dbg!(&task.start.day(), &task.deadline.day());
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
