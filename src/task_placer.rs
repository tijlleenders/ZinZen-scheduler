//! The core-scheduler focusses on scheduling. It is referred to as 'core' to avoid confusion
//! with the scheduler (which is pre-processor + core).
//! It takes a flat list of tasks with 0-* corresponding time-constrained slots.
//! For each tasks it decides which of the possible positions within the corresponding slots is best.
//! The scheduler optimizes for the minimum amount of IMPOSSIBLE tasks.
//! https://github.com/tijlleenders/ZinZen-scheduler/wiki/Core

use crate::task::TaskStatus::{IMPOSSIBLE, SCHEDULED};
use crate::task::Task;
use crate::task_generator::DateRange;
use chrono::{Duration, NaiveDateTime};

pub fn task_placer<'a>(tasks: &'a mut Vec<Task>, calendar_start: NaiveDateTime, calendar_end: NaiveDateTime) {
	let date_range = DateRange {
		start: calendar_start,
		end: calendar_end,
		interval: Some(Duration::hours(1)),
	};

	let mut time_slots: Vec<(NaiveDateTime, NaiveDateTime)> = date_range.collect();

	tasks.sort();

	for task in tasks {
		'inner: for i in 0..time_slots.len() {
			if time_slots[i].0 >= task.start {
				for _ in 0..task.duration {
					println!("Pushing slot in Task {:?}: {:?}", task.id, time_slots[i]);
					task.slots.push(time_slots.remove(i));
				}
                task.status = SCHEDULED;
				break 'inner;
			}
		}
	}
}
