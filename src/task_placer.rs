//For a visual step-by-step breakdown of the scheduler algorithm see https://docs.google.com/presentation/d/1Tj0Bg6v_NVkS8mpa-aRtbDQXM-WFkb3MloWuouhTnAM/edit?usp=sharing

use std::cmp::Ordering;

use chrono::Duration;

use crate::errors::Error;
use crate::goal::Tag;
use crate::input::{PlacedTasks, TasksToPlace};
use crate::task::{Task, TaskStatus};
use crate::{slot::*, task};

/// The Task Placer receives a list of tasks from the Task Generator and attempts to assign each
/// task a confirmed start and deadline.
/// The scheduler optimizes for the minimum amount of Impossible tasks.
pub fn task_placer(mut tasks_to_place: TasksToPlace) -> PlacedTasks {
    //first pass of scheduler while tasks are unsplit
    schedule(&mut tasks_to_place);

    // set any remaining TaskStatus::ReadyToSchedule to TaskStatus::Impossible;
    // !todo!();

    PlacedTasks {
        calendar_start: tasks_to_place.calendar_start,
        calendar_end: tasks_to_place.calendar_end,
        tasks: tasks_to_place.tasks,
    }
}

fn schedule(mut tasks_to_place: &mut TasksToPlace) {
    loop {
        tasks_to_place.sort_on_flexibility();
        if tasks_to_place.tasks[0].status != TaskStatus::ReadyToSchedule {
            break;
        }
        match find_best_slots(&tasks_to_place.tasks) {
            Some(chosen_slots) => do_the_scheduling(&mut tasks_to_place.tasks, chosen_slots),
            None => break,
        }
    }
}

fn do_the_scheduling(tasks_to_place: &mut Vec<Task>, chosen_slots: Vec<Slot>) {
    let mut new_task = tasks_to_place[0].clone();
    new_task.status = TaskStatus::Scheduled;
    new_task.duration = 1;

    new_task.id = tasks_to_place.len();
    new_task.slots.clear();
    for slot in chosen_slots.iter() {
        new_task.id += 1;
        new_task.confirmed_start = Some(slot.start);
        new_task.confirmed_deadline = Some(slot.end);
        tasks_to_place.push(new_task.clone());
    }
    //check if all duration handle schedule
    for task in tasks_to_place.iter_mut() {
        for slot in chosen_slots.iter() {
            task.remove_slot(slot.to_owned());
        }
    }
    let total_slots_duaration = chosen_slots.len();
    let remaining_hours = tasks_to_place[0].duration - total_slots_duaration;
    if remaining_hours > 0 {
        tasks_to_place[0].duration = remaining_hours;
        tasks_to_place[0].status = TaskStatus::Impossible;
    } else {
        let task_scheduled_goal_id = tasks_to_place[0].goal_id.clone();
        tasks_to_place.remove(0);
        for task in tasks_to_place {
            task.remove_from_blocked_by(task_scheduled_goal_id.clone());
        }
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

fn find_best_slots(tasks_to_place: &Vec<Task>) -> Option<Vec<Slot>> {
    let mut slot_conflicts: Vec<(Slot, usize)> = vec![];
    let task = &tasks_to_place[0];

    for slot in task.slots.iter() {
        for hour_slot in slot.get_1h_slots() {
            let mut count: usize = 0;
            for t in tasks_to_place {
                if t.slots.contains(&hour_slot) {
                    count += 1;
                }
            }
            slot_conflicts.push((hour_slot, count));
        }
    }
    slot_conflicts.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let mut result = vec![];
    for d in 0..task.duration {
        match slot_conflicts.pop() {
            Some(s) => result.push(s.0),
            None => break,
        }
    }

    Some(result)
}

//return the slot with lowest number of conflicts in slot_conflicts

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
