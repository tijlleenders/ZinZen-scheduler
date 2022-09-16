//! The Task Placer receives a list of tasks from the Task Generator and attempts to assign each
//! task a confirmed start and deadline.
//! The scheduler optimizes for the minimum amount of IMPOSSIBLE tasks.
//For a visual step-by-step breakdown of the scheduler algorithm see https://docs.google.com/presentation/d/1Tj0Bg6v_NVkS8mpa-aRtbDQXM-WFkb3MloWuouhTnAM/edit?usp=sharing

use crate::errors::Error;
use crate::task::Task;
use crate::task::TaskStatus::{SCHEDULED, UNSCHEDULED};
use crate::time_slice_iterator::{Repetition, TimeSliceIterator};
use chrono::{NaiveDateTime, Timelike};

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
            //check if the time_slot is:
            //1) within the start and deadline dates of the task
            if (time_slots[i].0 >= task.start) && (time_slots[i].1 < task.deadline) {
                //2) after the after_time of the task
                if time_slots[i].0.hour() >= task.after_time as u32 {
                    for _ in 0..(get_num_slots(task)) as usize {
                        if i < time_slots.len() {
                            task.slots.push(time_slots[i]);
                            i += 1;
                        }
                    }
                    //skip to midnight so as not to add more slots on the same day
                    while time_slots[i - 1].1.hour() != 0 {
                        i += 1;
                        if i == time_slots.len() {
                            break;
                        }
                    }
                    //if the remaining slots on calendar are less than this task's duration,
                    //truncate the task's duration
                    if task.slots.len() < task.duration {
                        task.duration = task.slots.len();
                    }
                    continue;
                }
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
            remove_slots_from_tasks(&mut tasks, &my_slots[..]);
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

fn get_num_slots(task: &Task) -> usize {
    if task.before_time > task.after_time {
        task.before_time - task.after_time
    } else {
        task.before_time + (24 - task.after_time)
    }
}

fn schedule_tasks(tasks: &mut Vec<Task>, counter: &mut usize) {
    tasks.sort();
    tasks.reverse();

    let length = tasks.len();
    'outer: for i in 0..length {
        println!(
            "Scheduling {} with slots {:?}",
            tasks[i].title, tasks[i].slots
        );
        let my_slots = tasks[i].get_slots();
        'inner: for (j, _) in my_slots.iter().enumerate() {
            if j + tasks[i].duration - 1 >= my_slots.len() {
                continue 'outer;
            }
            let desired_first_slot = my_slots.get(j).unwrap();
            let desired_last_slot = my_slots.get(j + tasks[i].duration - 1).unwrap();
            let mut is_conflicting = false;
            for k in 0..length {
                if tasks[k].status == SCHEDULED || k == i {
                    continue;
                }
                if !((desired_first_slot < &tasks[k].slots[0]
                    && desired_last_slot < &tasks[k].slots[0])
                    || (desired_first_slot > &tasks[k].slots[tasks[k].slots.len() - 1]
                        && desired_last_slot > &tasks[k].slots[tasks[k].slots.len() - 1]))
                {
                    println!("task {} conflicts with {}", tasks[k].title, tasks[i].title);
                    if k < i {
                        println!("k {k} is less than i {i}");
                        //can attempt to split since this is a failed-to-schedule task
                        match tasks[k].split(&(desired_first_slot.0, desired_last_slot.1), counter)
                        {
                            Err(_) => {
                                is_conflicting = true;
                                break;
                            }
                            Ok((taska, taskb)) => {
                                println!("Splitting.........");
                                tasks.push(taska);
                                tasks.push(taskb);
                                tasks[i].set_confirmed_start(desired_first_slot.0);
                                tasks[i].set_confirmed_deadline(desired_last_slot.1);
                                remove_slots_from_tasks(
                                    tasks,
                                    &my_slots[j..j + tasks[i].duration - 1],
                                );
                                tasks[i].status = SCHEDULED;
                                tasks.remove(k);

                                continue 'outer;
                            }
                        }
                    } else {
                        is_conflicting = true;
                        break;
                    }
                }
            }
            if is_conflicting {
                continue 'inner;
            }
            tasks[i].set_confirmed_start(desired_first_slot.0);
            tasks[i].set_confirmed_deadline(desired_last_slot.1);
            tasks[i].status = SCHEDULED;
            remove_slots_from_tasks(tasks, &my_slots[j..j + tasks[i].duration - 1]);
            println!("Task {} has been scheduled.", tasks[i].title);
            continue 'outer;
        }
    }
    //tasks
}

fn remove_slots_from_tasks(tasks: &mut Vec<Task>, my_slots: &[(NaiveDateTime, NaiveDateTime)]) {
    for task in tasks {
        for slot in my_slots {
            if task.slots.contains(slot) {
                println!("Removing slot {:?} from task {}", slot, task.title);
                task.remove_slot(slot);
            }
        }
    }
}
