//Responsible for generating slots that satisfy specific after_time/before_time time bounds,
//between a certain time period.
//For a visual step-by-step breakdown of the scheduler algorithm see https://docs.google.com/presentation/d/1Tj0Bg6v_NVkS8mpa-aRtbDQXM-WFkb3MloWuouhTnAM/edit?usp=sharing

use crate::repetition::Repetition;
use crate::slot::Slot;
use crate::task::Task;
use crate::time_slot_iterator::TimeSlotIterator;
use chrono::{Duration, NaiveDateTime, Timelike};

pub fn slot_generator(
    mut task: Task,
    time_period: &Slot,
    hard_deadline: Option<NaiveDateTime>,
) -> Task {
    if task.after_time == 0 && task.before_time == 24 {
        let slots = vec![Slot {
            start: time_period.start,
            end: time_period.end,
        }];
        task.slots = slots;
        return task;
    }

    //slides 2 - 7 (assign slots to tasks)
    //iterate through each hour in the time_period.
    //when we find an hour that is within the task's after_time and before_time,
    //assign a slot starting from there up to the before_time of the task.
    //e.g. if the time_period is a day and aftertime is 10 and before time is 14,
    //we'll get to 10 and add a slot starting from 10 up until 14.
    let hour_iterator = TimeSlotIterator::new(
        time_period.start,
        time_period.end,
        Some(Repetition::HOURLY),
        // Todo! 0-24
        vec![],
    );

    let mut slots: Vec<Slot> = Vec::new();
    let hours: Vec<Slot> = hour_iterator.collect();
    let mut i = 0; //index of hours
    while i < hours.len() {
        if !task.bounds_contain(hours[i].start.hour() as usize) {
            i += 1;
            continue;
        }
        let num_of_slots = size_of_slots_to_be_assigned(task.after_time, task.before_time);
        let slot = assign_slots(
            num_of_slots,
            hours.as_slice(),
            &mut i,
            hard_deadline,
            task.before_time,
        );
        slots.push(slot);
        i += 1;
    }
    task.slots = slots;
    task
}

fn assign_slots(
    num_of_slots: usize,
    hours: &[Slot],
    i: &mut usize,
    hard_deadline: Option<NaiveDateTime>,
    before_time: usize,
) -> Slot {
    let start = hours[*i];
    let mut end = start.start + Duration::hours(num_of_slots as i64);

    if let Some(hard_deadline) = hard_deadline {
        if end > hard_deadline {
            end = hard_deadline;
        }
    };

    //make sure assigned slots do not go past the task's beforetime
    if end.hour() > before_time as u32 {
        end = end.with_hour(before_time as u32).unwrap();
    }
    *i += num_of_slots;
    Slot {
        start: start.start,
        end,
    }
}

fn size_of_slots_to_be_assigned(after_time: usize, before_time: usize) -> usize {
    if before_time > after_time {
        before_time - after_time
    } else {
        before_time + (24 - after_time)
    }
}
