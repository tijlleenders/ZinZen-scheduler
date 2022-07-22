use std::cmp::min;

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

impl IntoIterator for DateRange {
	type Item = (NaiveDateTime, NaiveDateTime);
	type IntoIter = DateRangeIter;

	/// Generate an iterator for this DateRange,
	/// returning each pair of valid dates (start-end) in this interval.
	/// If start and end dates are not rounded up, they will be rounded
	/// by the returned values.
	fn into_iter(self) -> Self::IntoIter {
		Self::IntoIter {
			end: self.end,
			interval: self.interval,
			current_start: self.start,
			current_end: self.start,
		}
	}
}

pub(crate) struct DateRangeIter {
	end: NaiveDateTime,
	current_start: NaiveDateTime,
	current_end: NaiveDateTime,
	interval: Duration,
}

impl Iterator for DateRangeIter {
	type Item = (NaiveDateTime, NaiveDateTime);

	fn next(&mut self) -> Option<Self::Item> {
		if self.current_end == self.end {
			return None;
		}

		self.current_end = min(
			self.end,
			(self.current_start + self.interval)
				.duration_round(self.interval)
				.ok()?,
		);
		let res = (self.current_start, self.current_end);

		self.current_start = min(self.end, self.current_start + self.interval)
			.duration_round(self.interval)
			.ok()?;

		Some(res)
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
