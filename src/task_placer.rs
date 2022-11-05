//! The Task Placer receives a list of tasks from the Task Generator and attempts to assign each
//! task a confirmed start and deadline.
//! The scheduler optimizes for the minimum amount of IMPOSSIBLE tasks.
//For a visual step-by-step breakdown of the scheduler algorithm see https://docs.google.com/presentation/d/1Tj0Bg6v_NVkS8mpa-aRtbDQXM-WFkb3MloWuouhTnAM/edit?usp=sharing

use crate::errors::Error;
use crate::slot::Slot;
use crate::task::Task;
use crate::task::TaskStatus::{SCHEDULED, UNSCHEDULED};

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
            remove_slot_from_tasks(
                &mut tasks,
                Slot {
                    start: my_slots[0].start,
                    end: my_slots[0].end,
                },
            );
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
fn remove_slot_from_tasks(tasks: &mut Vec<Task>, slot: Slot) {
    for task in tasks {
        task.remove_slot(slot);
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
        let mut start_deadline_iterator = tasks[i].start_deadline_iterator();
        'slot_loop: while let Some(desired_time) = start_deadline_iterator.next() {
            'inner: for k in 0..tasks_length {
                if tasks[k].status == SCHEDULED || tasks[k].goal_id == tasks[i].goal_id {
                    continue 'inner;
                }
                for slot in &tasks[k].slots {
                    if desired_time.conflicts_with(slot) && !tasks[i].can_coexist_with(&tasks[k]) {
                        continue 'slot_loop;
                    }
                }
            }
            //if we've reached here it means no conflicts were found for this start/deadline
            //combination and we can schedule.
            tasks[i].schedule(desired_time);
            remove_slot_from_tasks(tasks, desired_time);
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
