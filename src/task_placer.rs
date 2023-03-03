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
pub fn task_placer(mut tasks_to_place: TasksToPlace) -> PlacedTasks {
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

fn schedule(mut tasks_to_place: &mut TasksToPlace) {
    loop {
        tasks_to_place.sort_on_flexibility();

        match find_best_slot(&tasks_to_place.tasks) {
            Some(chosen_slot) => do_the_scheduling(&mut tasks_to_place.tasks, chosen_slot),
            None => break,
        }
    }
}

fn do_the_scheduling(tasks_to_place: &mut Vec<Task>, chosen_slot: Slot) {
    tasks_to_place[0].schedule(chosen_slot);
    //REFACTOR!!
    // //prevent deadline end from exceeding calender end and update duration
    // for task in scheduled.iter_mut() {
    //     if task.confirmed_start.is_none() || task.confirmed_deadline.is_none() {
    //         return Err(Error::NoConfirmedDate(task.title.clone(), task.id));
    //     }
    //     //prevent slot end from exceeding calender end
    //     if task.confirmed_deadline.unwrap() > calender_end {
    //         task.confirmed_deadline = Some(calender_end);
    //         task.duration = Slot {
    //             start: task.confirmed_start.unwrap(),
    //             end: task.confirmed_deadline.unwrap(),
    //         }
    //         .num_hours();
    //     }
    // }
    for task in tasks_to_place.iter_mut() {
        task.remove_slot(chosen_slot);
        task.remove_from_blocked_by(task.goal_id.clone());
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

fn find_best_slot(tasks_to_place: &Vec<Task>) -> Option<Slot> {
    let slot_conflicts: Vec<(Slot, u32)>;
    let tasks_to_place: TasksToPlace;
    todo!()
}
