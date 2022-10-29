//! The Task Placer receives a list of tasks from the Task Generator and attempts to assign each
//! task a confirmed start and deadline.
//! The scheduler optimizes for the minimum amount of IMPOSSIBLE tasks.
//For a visual step-by-step breakdown of the scheduler algorithm see https://docs.google.com/presentation/d/1Tj0Bg6v_NVkS8mpa-aRtbDQXM-WFkb3MloWuouhTnAM/edit?usp=sharing

use crate::errors::Error;
use crate::slot::Slot;
use crate::task::Task;
use crate::task::TaskStatus::{SCHEDULED, UNSCHEDULED};
use chrono::NaiveDateTime;

pub fn task_placer(mut tasks: Vec<Task>) -> Vec<Task> {
    //slide 9 (schedule task(s) with flexibilityof 1)
    //TODO: currently we're assuming tasks with flex 1 are not conflicting.
    for index in 0..tasks.len() {
        if tasks[index].flexibility == 1 {
            let my_slots = tasks[index].get_slots();
            tasks[index].set_confirmed_start(my_slots[0].start);
            tasks[index].set_confirmed_deadline(my_slots[0].end);
            tasks[index].status = SCHEDULED;
            //slide 10 (remove the assigned slot from other tasks' slot lists)
            remove_slots_from_tasks(&mut tasks, my_slots[0].start, my_slots[0].end);
        }
    }

    //slides 12-20 (attempt to schedule the other tasks without conflicting with other tasks'
    //slots)
    //first we attempt to schedule the tasks as they are, however if unscheduled tasks remain, they
    //will be split into one hour tasks and then we attempt to schedule again.
    //The scheduling of the 1hr slots is done in a loop because some may still fail depending on
    //the order they are attempted to be scheduled.
    let mut counter = tasks[tasks.len() - 1].id + 1;
    schedule_tasks(&mut tasks);
    if tasks.iter().any(|t| t.status == UNSCHEDULED) {
        split_unscheduled_tasks(&mut tasks, &mut counter);
        loop {
            if tasks.iter().any(|t| t.status == UNSCHEDULED) {
                schedule_tasks(&mut tasks);
            } else {
                break;
            }
        }
    }
    tasks
}

//when a task is scheduled, remove the time it occupies from other tasks' slots
fn remove_slots_from_tasks(tasks: &mut Vec<Task>, start: NaiveDateTime, deadline: NaiveDateTime) {
    for task in tasks {
        let s = Slot {
            start,
            end: deadline,
        };
        let mut new_slots = Vec::new();
        let slots = task.get_slots();
        for slot in slots {
            new_slots.extend(slot - s);
        }
        task.slots = new_slots;
        if !task.slots.is_empty() {
            task.internal_marker = task.slots[0].start;
        }
    }
}

//This trait allows us to check whether we can schedule a task at a particular time by calling
//(desired_start,desired_deadline).conflicts_with(other_task's_aftertime,other_task's_beforetime)
pub trait Conflicts {
    fn conflicts_with(&self, time1: NaiveDateTime, time2: NaiveDateTime) -> bool;
}

impl Conflicts for (NaiveDateTime, NaiveDateTime) {
    fn conflicts_with(&self, after_time: NaiveDateTime, before_time: NaiveDateTime) -> bool {
        //we can be either completely before or completely after. otherwise it's a conflict.
        !((self.0 < after_time && self.1 <= after_time)
            || (self.0 >= before_time && self.1 > before_time))
    }
}

//the main scheduling function.
//for each task, it tries to schedule each start-deadline combination, while checking for conflicts
//with all other tasks' after_times and before_times
fn schedule_tasks(tasks: &mut Vec<Task>) {
    tasks.sort();
    tasks.reverse();
    let tasks_length = tasks.len();
    'task_loop: for i in 0..tasks_length {
        if tasks[i].status == SCHEDULED {
            continue;
        }
        //Place internal marker at first possible hour of task
        tasks[i].internal_marker = tasks[i].slots[0].start;
        'slot_loop: while let Some((desired_start, desired_deadline)) =
            tasks[i].next_start_deadline_combination()
        {
            'inner: for k in 0..tasks_length {
                if tasks[k].status == SCHEDULED || tasks[k].goal_id == tasks[i].goal_id {
                    continue 'inner;
                }
                let other_task_after_time = tasks[k].slots[0].start;
                let other_task_before_time = tasks[k].slots[tasks[k].slots.len() - 1].end;
                if (desired_start, desired_deadline)
                    .conflicts_with(other_task_after_time, other_task_before_time)
                    && !(tasks[i].goal_id == tasks[k].goal_id)
                    && !tasks[i].can_coexist_with(&tasks[k])
                {
                    continue 'slot_loop;
                }
            }
            //if we've reached here it means no conflicts were found for this start/deadline
            //combination and we can schedule.
            tasks[i].schedule(desired_start, desired_deadline);
            remove_slots_from_tasks(tasks, desired_start, desired_deadline);
            continue 'task_loop;
        }
    }
}

//if on the first pass of the scheduler, some tasks are unscheduled, split them into 1hr tasks and
//remove the original task.
fn split_unscheduled_tasks(tasks: &mut Vec<Task>, counter: &mut usize) {
    let mut new_tasks = Vec::new();
    let mut ids_to_remove = Vec::new();
    for i in 0..tasks.len() {
        if tasks[i].status == UNSCHEDULED {
            match tasks[i].split(counter) {
                Err(Error::CannotSplit) | Err(_) => {}
                Ok(mut one_hour_tasks) => {
                    ids_to_remove.push(tasks[i].id);
                    while !one_hour_tasks.is_empty() {
                        new_tasks.push(one_hour_tasks.pop().unwrap());
                    }
                }
            }
        }
    }
    for id in ids_to_remove {
        tasks.retain(|x| x.id != id);
    }
    tasks.extend_from_slice(&new_tasks[..]);
}
