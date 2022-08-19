use chrono::prelude::*;
use chrono::Duration;

use crate::input::Input;
use crate::task::{Slot, Task};
use crate::task_placer::TaskPlacer;
use crate::util::MyDurationRound;

/// A range of datetimes with an interval.
pub(crate) struct DateRange {
	pub(crate) start: NaiveDateTime,
	pub(crate) end: NaiveDateTime,
	pub(crate) interval: Duration,
}

impl Iterator for DateRange {
	type Item = (NaiveDateTime, NaiveDateTime);
	fn next(&mut self) -> Option<Self::Item> {
		if self.start < self.end {
			let start = self.start;
			let mut end = self.start + self.interval;
			if end > self.end {
				end = self.end;
			} else {
				end = end.duration_round(self.interval).ok()?;
			}
			self.start = end;
			Some((start, end))
		} else {
			None
		}
	}
}

pub fn task_generator(Input { start, end, goals }: Input) -> TaskPlacer {
	let mut tasks = vec![];
	let mut slots = vec![];

	let mut id = 0;
	for goal in goals.into_iter() {
		// Default dates for goals if start/deadline is not set
		let start = start.and_time(NaiveTime::from_hms(0, 0, 0));
		let end = end.and_time(NaiveTime::from_hms(0, 0, 0));
		let goal_start = goal.start.unwrap_or(start);
		let goal_end = goal.deadline.unwrap_or(end);

		// If there's repetition, create multiple tasks
		if let Some(repetition) = goal.repetition {
			let range = DateRange {
				start: goal_start,
				end: goal_end,
				interval: repetition.into(),
			};

			for (task_start, task_end) in range {
				tasks.push(Task::new(id, goal.id, goal.duration));
				slots.push(Slot::new(
					id,
					(task_start - start).num_hours() as usize,
					(task_end - start).num_hours() as usize,
				));
				id += 1;
			}
			continue;
		}

		tasks.push(Task::new(id, goal.id, goal.duration));
		slots.push(Slot::new(
			id,
			(goal_start - start).num_hours() as usize,
			(goal_end - start).num_hours() as usize,
		));
		id += 1;
	}

	TaskPlacer::new(tasks, slots)
}
