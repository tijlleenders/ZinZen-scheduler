//! The Task Placer receives a list of tasks from the Task Generator and attempts to assign each
//! task a confirmed start and deadline.
//! The scheduler optimizes for the minimum amount of IMPOSSIBLE tasks.
//For a visual step-by-step breakdown of the scheduler algorithm see https://docs.google.com/presentation/d/1Tj0Bg6v_NVkS8mpa-aRtbDQXM-WFkb3MloWuouhTnAM/edit?usp=sharing

use crate::slot::Slot;
use crate::task::Task;
use crate::time_slot_iterator::{Repetition, TimeSlotIterator};
use chrono::{NaiveDateTime, Timelike};

pub fn slot_assigner(
    mut tasks: Vec<Task>,
    calendar_start: NaiveDateTime,
    calendar_end: NaiveDateTime,
) -> Vec<Task> {
    //slide 1 (generate all time slots based on calendar dates)
    let time_slot_iterator = TimeSlotIterator {
        start: calendar_start,
        end: calendar_end,
        repetition: Repetition::HOURLY,
    };
    let time_slots: Vec<Slot> = time_slot_iterator.collect();

    //slides 2 - 7 (assign slots to tasks)
    for task in &mut tasks {
        let mut i = 0;
        while i < time_slots.len() {
            //1) is the time_slot within the start and deadline dates of the task?
            if !((time_slots[i].start >= task.start) && (time_slots[i].end < task.deadline)) {
                i += 1;
                continue;
            }
            //2) is the time_slot after the after_time of the task?
            if !(time_slots[i].start.hour() >= task.after_time as u32) {
                i += 1;
                continue;
            }
            assign_slots(task, &time_slots, &mut i);

            i += 1;
        }
        //if too few slots were assigned for the task (the remaining slots on calendar were not enough),
        //truncate the task's duration.
        //TODO: Need to confirm if this is the expected behaviour in all cases.
        let mut total_slot_hours = 0;
        for slot in &task.slots {
            total_slot_hours += slot.num_hours();
        }
        if total_slot_hours < task.duration {
            task.duration = total_slot_hours;
        }
        task.calculate_flexibility();
    }

    tasks
}

//assigns slots to a task based on its after_time and before_time.
//"i" is an index referring to a position in the time_slots vector.
fn assign_slots(task: &mut Task, time_slots: &Vec<Slot>, i: &mut usize) {
    let start = time_slots[*i];
    let mut end = time_slots[*i];
    for _ in 1..(task.size_of_slots_to_be_assigned()) as usize {
        if *i < time_slots.len() - 1 {
            *i += 1;
            end = time_slots[*i];
        }
    }
    let slot = Slot {
        start: start.start,
        end: end.end,
    };

    task.slots.push(slot);
    //move the time_slots index to midnight so as not to add more slots on the same day
    while time_slots[*i - 1].end.hour() != 0 {
        *i += 1;
        if *i == time_slots.len() {
            break;
        }
    }
}
