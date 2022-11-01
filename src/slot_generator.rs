//Responsible for generating slots that satisfy specific after_time/before_time time bounds,
//between a certain time period.
//For a visual step-by-step breakdown of the scheduler algorithm see https://docs.google.com/presentation/d/1Tj0Bg6v_NVkS8mpa-aRtbDQXM-WFkb3MloWuouhTnAM/edit?usp=sharing

use crate::slot::Slot;
use crate::time_slot_iterator::{Repetition, TimeSlotIterator};
use chrono::{Duration, Timelike};

pub fn slot_generator(after_time: usize, before_time: usize, time_period: &Slot) -> Vec<Slot> {
    //slides 2 - 7 (assign slots to tasks)
    let hour_iterator = TimeSlotIterator {
        start: time_period.start,
        end: time_period.end,
        repetition: Some(Repetition::HOURLY),
    };

    let mut slots: Vec<Slot> = Vec::new();
    let hours: Vec<Slot> = hour_iterator.collect();
    let mut i = 0;
    while i < hours.len() {
        //check if the time_slot is after the after_time of the task
        if !(hours[i].start.hour() >= after_time as u32) {
            i += 1;
            continue;
        }
        let num_of_slots = size_of_slots_to_be_assigned(after_time, before_time);
        let mut slot = assign_slots(num_of_slots, &hours, &mut i);
        //handle cases where beforetime is on the next day e.g. sleep 22-6
        if before_time < after_time {
            slot.end += Duration::hours(before_time as i64);
        }
        //merge adjacent slots. this happens when aftertime is 0 and beforetime is 24 and the time
        //period spans multiple days.
        if slots.len() > 0 && slot.start == slots[slots.len() - 1].end {
            slots.push(slots[slots.len() - 1] + slot);
            slots.remove(slots.len() - 2);
        } else {
            slots.push(slot);
        }

        i += 1;
    }

    slots
}

fn assign_slots(num_of_slots: usize, hours: &Vec<Slot>, i: &mut usize) -> Slot {
    let start = dbg!(hours[*i]);
    let mut end = hours[*i];
    for _ in 1..num_of_slots as usize {
        if *i < dbg!(hours.len() - 1) {
            *i += 1;
            end = hours[*i];
        }
    }

    let slot = Slot {
        start: start.start,
        end: end.end,
    };

    //move the index to midnight so as not to add more slots on the same day
    while (hours[*i].end).hour() != 0 {
        *i += 1;
        if *i == hours.len() {
            break;
        }
    }

    slot
}

fn size_of_slots_to_be_assigned(after_time: usize, before_time: usize) -> usize {
    if before_time > after_time {
        before_time - after_time
    } else {
        before_time + (24 - after_time)
    }
}
