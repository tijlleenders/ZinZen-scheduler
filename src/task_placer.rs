//For a visual step-by-step breakdown of the scheduler algorithm see https://docs.google.com/presentation/d/1Tj0Bg6v_NVkS8mpa-aRtbDQXM-WFkb3MloWuouhTnAM/edit?usp=sharing

use chrono::Duration;

use crate::errors::Error;
use crate::goal::Tag;
use crate::input::{PlacedTasks, TasksToPlace};
use crate::slot::Slot;
use crate::task::{Task, TaskStatus};

/// The Task Placer receives a list of tasks from the Task Generator and attempts to assign each
/// task a confirmed start and deadline.
/// The scheduler optimizes for the minimum amount of Impossible tasks.
pub fn task_placer(tasks_to_place: TasksToPlace) -> PlacedTasks {
    //first pass of scheduler while tasks are unsplit
    schedule(&mut tasks_to_place);

    //second pass of scheduler with anything remaining split
    // split_remaining_tasks(&mut tasks, &mut counter);
    schedule(&mut tasks_to_place);

    //if tasks is not empty, it means some tasks were unable to be scheduled
    // split_remaining_tasks(&mut tasks, &mut counter);
    schedule(&mut tasks_to_place);

    // set any remaining TaskStatus::ReadyToSchedule to TaskStatus::Impossible;
    !todo!();

    PlacedTasks {
        calendar_start: tasks_to_place.calendar_start,
        calendar_end: tasks_to_place.calendar_end,
        tasks: tasks_to_place.tasks,
    }
}

fn schedule(tasks_to_place: &mut TasksToPlace) {
    'find_ready_to_schedule: loop {
        tasks_to_place.sort_on_flexibility();
        for task in tasks_to_place.tasks.iter_mut() {
            match task.status {
                TaskStatus::Blocked => {
                    continue;
                }
                TaskStatus::ReadyToSchedule => {
                    let slot_finder = SlotFinder::new(*task, tasks_to_place);
                    match slot_finder.find_best_slot() {
                        Some(slot) => {
                            do_the_scheduling(tasks_to_place, 1, slot);
                        }
                        None => {
                            task.status == TaskStatus::Impossible;
                        }
                    }
                }
                TaskStatus::Uninitialized => panic!("no uninitialized tasks should be present"),
                TaskStatus::Impossible | TaskStatus::Scheduled => {
                    break 'find_ready_to_schedule;
                }
            }
        }
    }
}

fn do_the_scheduling(tasks_to_place: &mut TasksToPlace, task_index: usize, desired_slot: Slot) {
    tasks_to_place.tasks[task_index].schedule(desired_slot);
    for task in tasks_to_place.tasks.iter_mut() {
        task.remove_slot(desired_slot);
        task.remove_from_blocked_by(task.goal_id);
    }
}

//splits remaining tasks into 1hr tasks and modifies 'tasks' accordingly.
fn split_remaining_tasks(tasks: &mut Vec<Task>, counter: &mut usize) {
    let mut new_tasks = Vec::new();
    let mut ids_to_remove = Vec::new();
    for task in tasks.iter_mut() {
        if (task.status == TaskStatus::ReadyToSchedule) && !task.tags.contains(&Tag::Optional) {
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

pub struct SlotFinder {
    slot_finder: Vec<(Slot, u32)>,
    tasks_to_place: TasksToPlace,
}

impl SlotFinder {
    fn new(task: Task, &mut tasks_to_place: &mut TasksToPlace) -> SlotFinder {
        SlotFinder {
            slot_finder: vec![],
            tasks_to_place: tasks_to_place,
        }
    }

    fn find_best_slot(&self) -> Option<Slot> {
        todo!()
    }
}
