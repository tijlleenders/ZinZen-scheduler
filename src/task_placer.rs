//! The core-scheduler focusses on scheduling. It is referred to as 'core' to avoid confusion
//! with the scheduler (which is pre-processor + core).
//! It takes a flat list of tasks with 0-* corresponding time-constrained slots.
//! For each tasks it decides which of the possible positions within the corresponding slots is best.
//! The scheduler optimizes for the minimum amount of IMPOSSIBLE tasks.
//! https://github.com/tijlleenders/ZinZen-scheduler/wiki/Core

use crate::task::Task;
use crate::task::TaskStatus::{IMPOSSIBLE, SCHEDULED};
use crate::date_range::DateRange;
use chrono::{Duration, NaiveDateTime};

//see the slides for an explanation of the algorithm https://docs.google.com/presentation/d/1Tj0Bg6v_NVkS8mpa-aRtbDQXM-WFkb3MloWuouhTnAM/edit?usp=sharing
pub fn task_placer<'a>(mut tasks: Vec<Task>, calendar_start: NaiveDateTime, calendar_end: NaiveDateTime) -> Vec<Task> {
	//slide 1
	let date_range = DateRange {
		start: calendar_start,
		end: calendar_end,
		interval: Some(Duration::hours(1)),
	};
	let time_slots: Vec<(NaiveDateTime, NaiveDateTime)> = date_range.collect();

	//slides 2 - 7
	for task in tasks.iter_mut() {
		'inner: for i in 0..time_slots.len() {
			if time_slots[i].0 >= task.start {
				for j in 0..((task.deadline-task.start).num_hours() as usize) {
					task.slots.push(time_slots[i+j]);
				}
				task.calculate_flexibility();
				break 'inner;
			}
		}
	}

	tasks.sort();
	tasks.reverse();
	let mut scheduled_tasks = Vec::new();

    //slide 9
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
		//slide 10:
		for task in &mut tasks {
			for i in 0..my_slots.len() {
				if task.slots.contains(&my_slots[i]) {
					task.remove_slot(&my_slots[i]);
				}
			}
		}
	}

	//slides 12-20
	while tasks.len() > 0 {
		let mut task = tasks.remove(0);
		'outer: for slot in task.get_slots() {
			for other_task in tasks.iter() {
				if other_task.status == SCHEDULED {
					continue;
				}
				if other_task.slots.contains(&slot) {
					continue 'outer;
				}
			}
			task.set_confirmed_start(slot.0);
			task.set_confirmed_deadline(slot.1);
			task.status = SCHEDULED;
			scheduled_tasks.push(task);
			break;
		}
	}
	scheduled_tasks
}
