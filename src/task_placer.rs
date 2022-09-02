//! The core-scheduler focusses on scheduling. It is referred to as 'core' to avoid confusion
//! with the scheduler (which is pre-processor + core).
//! It takes a flat list of tasks with 0-* corresponding time-constrained slots.
//! For each tasks it decides which of the possible positions within the corresponding slots is best.
//! The scheduler optimizes for the minimum amount of IMPOSSIBLE tasks.
//! https://github.com/tijlleenders/ZinZen-scheduler/wiki/Core

use crate::task::Task;
use crate::task::TaskStatus::{IMPOSSIBLE, SCHEDULED};
use crate::time_slice_iterator::{TimeSliceIterator, Repetition};
use chrono::{Duration, NaiveDateTime, Timelike};

//see the slides for an explanation of the algorithm https://docs.google.com/presentation/d/1Tj0Bg6v_NVkS8mpa-aRtbDQXM-WFkb3MloWuouhTnAM/edit?usp=sharing
pub fn task_placer<'a>(mut tasks: Vec<Task>, calendar_start: NaiveDateTime, calendar_end: NaiveDateTime) -> Vec<Task> {
	//slide 1 (generate all time slots based on calendar dates)
	let time_slice_iterator = TimeSliceIterator {
		start: calendar_start,
		end: calendar_end,
		repetition: Repetition::HOURLY,
	};
	let time_slots: Vec<(NaiveDateTime, NaiveDateTime)> = time_slice_iterator.collect();

	//slides 2 - 7 (assign slots to tasks)
    for task in tasks.iter_mut() {
        for i in 0..time_slots.len() {
           //check if the time_slot is 1) within the start and deadline dates of the task, 2)
           //within the after_time and before_time of the task, and 3) the task doesn't already
           //contain the slot
           if (time_slots[i].0 >= task.start) && (time_slots[i].1 < task.deadline) {
               if (time_slots[i].0.hour() >= task.after_time as u32) && (time_slots[i].1.hour() <= task.before_time as u32) {
                    if !task.slots.contains(&time_slots[i]){
                        for j in 0..task.duration as usize {
                            task.slots.push(time_slots[i+j]);
                        }
                    }
               }
           }
        }
        task.calculate_flexibility();
    }


	tasks.sort();
	tasks.reverse();
	let mut scheduled_tasks = Vec::new();

    //slide 9 (assign slot(s) to task with flexibilityof 1)
	//TODO: need to make this more concise
	let last_index = tasks.len() - 1;
	if tasks[last_index].flexibility == 1 {
		let mut task = tasks.remove(last_index);
		let my_slots = task.get_slots();
		task.set_confirmed_start(my_slots[0].0);
		let deadline = my_slots[my_slots.len() - 1].1;
		task.set_confirmed_deadline(deadline);
		task.status = SCHEDULED;
		scheduled_tasks.push(task);
		//slide 10 (remove the assigned slot from other tasks' slot lists)
		for task in &mut tasks {
			for i in 0..my_slots.len() {
				if task.slots.contains(&my_slots[i]) {
					task.remove_slot(&my_slots[i]);
				}
			}
		}
	}

	//slides 12-20 (attempt to schedule the other tasks without conflicting with other tasks'
    //slots)
	while tasks.len() > 0 {
		let mut task = tasks.remove(0);
		'outer: for (index,slot) in task.get_slots().iter().enumerate() {
            let my_slots = task.get_slots();
            let desired_first_slot = my_slots.get(index).unwrap();
            let desired_last_slot = my_slots.get(index + task.duration - 1).unwrap();
			for other_task in tasks.iter() {
				if other_task.status == SCHEDULED {
					continue;
				}
				if other_task.slots.contains(&desired_first_slot) || other_task.slots.contains(&desired_last_slot) {
					continue 'outer;
				}
			}
			task.set_confirmed_start(desired_first_slot.0);
			task.set_confirmed_deadline(desired_last_slot.1);
			task.status = SCHEDULED;
			scheduled_tasks.push(task);
			break;
		}
	}
	scheduled_tasks
}
