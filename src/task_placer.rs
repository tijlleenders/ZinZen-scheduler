//! The Task Placer receives a list of tasks from the Task Generator and attempts to assign each
//! task a confirmed start and deadline.
//! The scheduler optimizes for the minimum amount of IMPOSSIBLE tasks.
//For a visual step-by-step breakdown of the scheduler algorithm see https://docs.google.com/presentation/d/1Tj0Bg6v_NVkS8mpa-aRtbDQXM-WFkb3MloWuouhTnAM/edit?usp=sharing

use crate::errors::Error;
use crate::task::TaskStatus::{SCHEDULED, UNSCHEDULED};
use crate::task::{Task, TaskStatus};

pub fn task_placer(mut tasks: Vec<Task>) -> Vec<Task> {
    let mut scheduled_tasks: Vec<Task> = Vec::new();
    let mut i = 0; //index
    tasks.sort();
    loop {
        if i == tasks.len() {
            //if tasks is empty we're done; all tasks are scheduled
            if tasks.is_empty() {
                break;
            }
            //we have some unschedulable tasks. so split them, and restart the loop
            let mut counter = tasks[tasks.len() - 1].id + 1;
            split_unscheduled_tasks(&mut tasks, &mut counter);
            i = 0;
            continue;
        }
        if schedule_successful(i, &mut tasks) {
            scheduled_tasks.push(tasks.remove(i));
            i = 0;
            tasks.sort();
        } else {
            i += 1;
        }
    }
    scheduled_tasks
}

fn schedule_successful(i: usize, tasks: &mut Vec<Task>) -> bool {
    let start_deadline_iterator = tasks[i].start_deadline_iterator();
    'outer: for desired_time in start_deadline_iterator {
        'inner: for k in 0..tasks.len() {
            if tasks[k].status == SCHEDULED || tasks[k].goal_id == tasks[i].goal_id {
                continue 'inner;
            }
            for slot in &tasks[k].slots {
                if desired_time.conflicts_with(slot) && !tasks[i].can_coexist_with(&tasks[k]) {
                    //found a conflict so try another start/deadline combo
                    continue 'outer;
                }
            }
        }
        //if we're here it means no conflicts were found for this desired time (start/deadline) and we can schedule.
        tasks[i].schedule(desired_time);
        //remove this time from other tasks' slots (except for those already scheduled)
        for task in tasks {
            if task.status != TaskStatus::SCHEDULED {
                task.remove_slot(desired_time);
            }
        }
        return true;
    }
    //there were conflicts for all possible start/deadline combos for this task
    false
}

//splits unscheduled tasks into 1hr tasks and modifies 'tasks' accordingly.
fn split_unscheduled_tasks(tasks: &mut Vec<Task>, counter: &mut usize) {
    let mut new_tasks = Vec::new();
    let mut ids_to_remove = Vec::new();
    for task in tasks.iter_mut() {
        if task.status == UNSCHEDULED {
            match task.split(counter) {
                Err(Error::CannotSplit) | Err(_) => {}
                Ok(mut one_hour_tasks) => {
                    ids_to_remove.push(task.id);
                    while !one_hour_tasks.is_empty() {
                        new_tasks.push(one_hour_tasks.pop().unwrap());
                    }
                }
            }
        }
    }
    //need to remove the 'original' tasks
    for id in ids_to_remove {
        tasks.retain(|x| x.id != id);
    }
    tasks.extend_from_slice(&new_tasks[..]);
}
