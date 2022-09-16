//! The Task Placer receives a list of tasks from the Task Generator and attempts to assign each
//! task a confirmed start and deadline.
//! The scheduler optimizes for the minimum amount of IMPOSSIBLE tasks.
//For a visual step-by-step breakdown of the scheduler algorithm see https://docs.google.com/presentation/d/1Tj0Bg6v_NVkS8mpa-aRtbDQXM-WFkb3MloWuouhTnAM/edit?usp=sharing

use crate::errors::Error;
use crate::task::Task;
use crate::task::TaskStatus::{SCHEDULED, UNSCHEDULED};
use crate::time_slice_iterator::{Repetition, TimeSliceIterator};
use chrono::{NaiveDateTime, Timelike};

pub trait Conflicts {
    fn conflicts_with(&self, time1: NaiveDateTime, time2: NaiveDateTime) -> bool;
}

impl Conflicts for (NaiveDateTime, NaiveDateTime) {
    fn conflicts_with(&self, after_time: NaiveDateTime, before_time: NaiveDateTime) -> bool {
        //we can be either completely before or completely after. otherwise it's a conflict.
        !((self.0 < after_time && self.1 <= before_time)
            || (self.0 >= before_time && self.1 > before_time))
    }
}

pub fn task_placer(
    mut tasks: Vec<Task>,
    calendar_start: NaiveDateTime,
    calendar_end: NaiveDateTime,
) -> Vec<Task> {
    //slide 1 (generate all time slots based on calendar dates)
    let time_slice_iterator = TimeSliceIterator {
        start: calendar_start,
        end: calendar_end,
        repetition: Repetition::HOURLY,
    };
    let time_slots: Vec<(NaiveDateTime, NaiveDateTime)> = time_slice_iterator.collect();

    //slides 2 - 7 (assign slots to tasks)
    for task in &mut tasks {
        let mut i = 0;
        while i < time_slots.len() {
            //1) is the time_slot within the start and deadline dates of the task?
            if !((time_slots[i].0 >= task.start) && (time_slots[i].1 < task.deadline)) {
                i += 1;
                continue;
            }
            //2) is the time_slot after the after_time of the task?
            if !(time_slots[i].0.hour() >= task.after_time as u32) {
                i += 1;
                continue;
            }
            assign_slots(task, &time_slots, &mut i);
            //if too few slots were assigned (the remaining slots on calendar were not enough),
            //truncate the task's duration
            if task.slots.len() < task.duration {
                task.duration = task.slots.len();
            }
            i += 1;
        }
        task.calculate_flexibility();
    }

    //slide 9 (schedule task(s) with flexibilityof 1)
    for index in 0..tasks.len() {
        if tasks[index].flexibility == 1 {
            let my_slots = tasks[index].get_slots();
            tasks[index].set_confirmed_start(my_slots[0].0);
            tasks[index].set_confirmed_deadline(my_slots[my_slots.len() - 1].1);
            tasks[index].status = SCHEDULED;
            //slide 10 (remove the assigned slot from other tasks' slot lists)
            remove_slots_from_tasks(&mut tasks, my_slots[0].0, my_slots[my_slots.len() - 1].1);
        }
    }

    //slides 12-20 (attempt to schedule the other tasks without conflicting with other tasks'
    //slots)
    let mut counter = tasks[tasks.len() - 1].id + 1;
    loop {
        schedule_tasks(&mut tasks, &mut counter);
        if !tasks.iter().any(|t| t.status == UNSCHEDULED) {
            break;
        }
    }

    tasks
}

fn assign_slots(task: &mut Task, time_slots: &Vec<(NaiveDateTime, NaiveDateTime)>, i: &mut usize) {
    for _ in 0..(task.num_slots()) as usize {
        if *i < time_slots.len() {
            task.slots.push(time_slots[*i]);
            *i += 1;
        }
    }
    //skip to midnight so as not to add more slots on the same day
    while time_slots[*i - 1].1.hour() != 0 {
        *i += 1;
        if *i == time_slots.len() {
            break;
        }
    }
}

fn schedule_tasks(tasks: &mut Vec<Task>, counter: &mut usize) {
    tasks.sort();
    tasks.reverse();

    let tasks_length = tasks.len();
    'task_loop: for i in 0..tasks_length {
        while let Some((desired_start, desired_deadline)) =
            tasks[i].next_start_deadline_combination()
        {
            let mut is_conflicting = false;
            for k in 0..tasks_length {
                if tasks[k].status == SCHEDULED || k == i {
                    continue;
                }
                let other_task_after_time = tasks[k].slots[0].0;
                let other_task_before_time = tasks[k].slots[tasks[k].slots.len() - 1].1;
                if (desired_start, desired_deadline)
                    .conflicts_with(other_task_after_time, other_task_before_time)
                {
                    if k < i {
                        //this is a task that already tried to be scheduled but failed
                        //so we can split it and take slots from it
                        match tasks[k].split(&(desired_start, desired_deadline), counter) {
                            Err(Error::CannotSplit) | Err(_) => {
                                is_conflicting = true;
                                break;
                            }
                            Ok((taska, taskb)) => {
                                tasks.push(taska);
                                tasks.push(taskb);
                                tasks[i].schedule(desired_start, desired_deadline);
                                remove_slots_from_tasks(tasks, desired_start, desired_deadline);
                                tasks.remove(k);
                                continue 'task_loop;
                            }
                        }
                    } else {
                        is_conflicting = true;
                        break;
                    }
                }
            }
            if is_conflicting {
                continue;
            }
            tasks[i].schedule(desired_start, desired_deadline);
            remove_slots_from_tasks(tasks, desired_start, desired_deadline);
            continue 'task_loop;
        }
    }
}

fn remove_slots_from_tasks(tasks: &mut Vec<Task>, start: NaiveDateTime, deadline: NaiveDateTime) {
    for task in tasks {
        let slots = task.get_slots();
        for slot in slots {
            if slot.0 >= start && slot.1 <= deadline {
                task.remove_slot(&slot);
            }
        }
    }
}
