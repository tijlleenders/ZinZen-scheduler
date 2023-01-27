//For a visual step-by-step breakdown of the scheduler algorithm see https://docs.google.com/presentation/d/1Tj0Bg6v_NVkS8mpa-aRtbDQXM-WFkb3MloWuouhTnAM/edit?usp=sharing

use chrono::Duration;

use crate::errors::Error;
use crate::goal::Tag;
use crate::slot::Slot;
use crate::task::TaskStatus::{Scheduled, UNScheduled};
use crate::task::{Task, TaskStatus};

/// The Task Placer receives a list of tasks from the Task Generator and attempts to assign each
/// task a confirmed start and deadline.
/// The scheduler optimizes for the minimum amount of Impossible tasks.
pub fn task_placer(mut tasks: Vec<Task>) -> (Vec<Task>, Vec<Task>) {
    let mut scheduled_tasks: Vec<Task> = Vec::new();

    //first pass of scheduler while tasks are unsplit
    schedule(&mut tasks, &mut scheduled_tasks);

    //if tasks is not empty, it means some tasks were unable to be scheduled
    //so we split the tasks and do another schedule run
    if !tasks.is_empty() {
        //we have some unschedulable tasks. so split them, and attempt to
        //schedule again
        let mut counter = tasks[tasks.len() - 1].id + 1;
        split_unscheduled_tasks(&mut tasks, &mut counter);
        tasks.sort();
        //schedule again
        schedule(&mut tasks, &mut scheduled_tasks);
    }

    //if tasks is still not empty, these are impossible to schedule tasks
    for task in &mut tasks {
        task.status = TaskStatus::Impossible;
    }

    (scheduled_tasks, tasks)
}

fn schedule(tasks: &mut Vec<Task>, scheduled_tasks: &mut Vec<Task>) {
    let mut i = 0; //index that points to a task in the collection of tasks
    tasks.sort();
    while i < tasks.len() {
        //if this task's status is Impossible, skip
        if tasks[i].status == TaskStatus::Impossible {
            i += 1;
            continue;
        }
        //check if it is possible to schedule this task
        if let Some(desired_time) = can_schedule(i, tasks) {
            tasks[i].schedule(desired_time);
            //since the task was scheduled, remove this time from other tasks' slots (except for those already scheduled)
            for k in 0..tasks.len() {
                //if the other task is a weekly task and is of the same goal id, remove the entire day from the
                //other task's slots. else remove just the time that this task has been scheduled at.
                //this is to prevent weekly tasks from combining all on on one day.
                if tasks[k].tags.contains(&Tag::Weekly) && tasks[k].goal_id == tasks[i].goal_id {
                    let day = desired_time.start.date().and_hms_opt(0, 0, 0).unwrap();
                    let slot = Slot {
                        start: day,
                        end: day + Duration::days(1),
                    };
                    tasks[k].remove_slot(slot);
                } else {
                    tasks[k].remove_slot(desired_time);
                }
                //if the removal has rendered the other task Impossible, add this task to that task's conflicts
                if tasks[k].status == TaskStatus::Impossible {
                    let goal_id = tasks[i].goal_id.to_owned();
                    tasks[k].conflicts.push((desired_time, goal_id));
                }
            }
            //add the task to list of scheduled tasks
            scheduled_tasks.push(tasks.remove(i));
            i = 0;
            tasks.sort();
        } else {
            i += 1;
        }
    }
}

pub fn can_schedule(i: usize, tasks: &mut Vec<Task>) -> Option<Slot> {
    let start_deadline_iterator = tasks[i].start_deadline_iterator().unwrap();
    'outer: for desired_time in start_deadline_iterator {
        if tasks[i].flexibility == 1 {
            //if task has flex 1, no need to check for conflicts with other tasks
            return Some(desired_time);
        }
        'inner: for k in 0..tasks.len() {
            if tasks[k].status == Scheduled || tasks[k].goal_id == tasks[i].goal_id {
                continue 'inner;
            }
            for slot in &tasks[k].slots {
                if desired_time.conflicts_with(slot) && !tasks[i].can_coexist_with(&tasks[k]) {
                    //found a conflict so try another start/deadline combo
                    //but save the conflict first
                    let goal_id = tasks[k].goal_id.to_owned();
                    tasks[i].conflicts.push((desired_time, goal_id));
                    continue 'outer;
                }
            }
        }
        //if we're here it means no conflicts were found for this desired time (start/deadline) and we can schedule.
        return Some(desired_time);
    }
    //there were conflicts for all possible start/deadline combos for this task
    //so not possible to schedule
    None
}

//splits unscheduled tasks into 1hr tasks and modifies 'tasks' accordingly.
fn split_unscheduled_tasks(tasks: &mut Vec<Task>, counter: &mut usize) {
    let mut new_tasks = Vec::new();
    let mut ids_to_remove = Vec::new();
    for task in tasks.iter_mut() {
        if task.status == UNScheduled && !task.tags.contains(&Tag::Optional) {
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
