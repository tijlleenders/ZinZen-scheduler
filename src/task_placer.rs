//! The Task Placer receives a list of tasks from the Task Generator and attempts to assign each
//! task a confirmed start and deadline.
//! The scheduler optimizes for the minimum amount of IMPOSSIBLE tasks.
//! https://github.com/tijlleenders/ZinZen-scheduler/wiki/Core
//For a visual step-by-step breakdown of the scheduler algorithm see https://docs.google.com/presentation/d/1Tj0Bg6v_NVkS8mpa-aRtbDQXM-WFkb3MloWuouhTnAM/edit?usp=sharing

use crate::task::Task;
use crate::task::TaskStatus::{IMPOSSIBLE, SCHEDULED};
use crate::time_slice_iterator::{Repetition, TimeSliceIterator};
use chrono::{Duration, NaiveDateTime, Timelike};

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
							i = i + 1;
						}
					}
					//skip to midnight so as not to add more slots on the same day
					while time_slots[i - 1].1.hour() != 0 {
						i = i + 1;
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
			i = i + 1;
		}
		task.calculate_flexibility();
	}

	tasks.sort();
	tasks.reverse();
	let mut scheduled_tasks = Vec::new();

	//slide 9 (assign slot(s) to task with flexibilityof 1)
	for index in 0..tasks.len() {
		if tasks[index].flexibility == 1 {
			let my_slots = tasks[index].get_slots();
			tasks[index].set_confirmed_start(my_slots[0].0);
			let deadline = my_slots[my_slots.len() - 1].1;
			tasks[index].set_confirmed_deadline(deadline);
			tasks[index].status = SCHEDULED;
			scheduled_tasks.push(tasks[index].clone());
			//slide 10 (remove the assigned slot from other tasks' slot lists)
			for task in &mut tasks {
				for i in 0..my_slots.len() {
					if task.slots.contains(&my_slots[i]) {
						task.remove_slot(&my_slots[i]);
					}
				}
			}
		}
	}

	//slides 12-20 (attempt to schedule the other tasks without conflicting with other tasks'
	//slots)
	while tasks.len() > 0 {
		let mut task = tasks.remove(0);
		'outer: for (index, _) in task.get_slots().iter().enumerate() {
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

fn get_num_slots(task: &Task) -> usize {
	if task.before_time > task.after_time {
		task.before_time - task.after_time
	} else {
		task.before_time + (24 - task.after_time)
	}
}
