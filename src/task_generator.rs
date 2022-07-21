use std::cmp::min;

use chrono::prelude::*;
use chrono::{Duration, DurationRound};

use crate::input::Input;
use crate::task::{Slot, Task};
use crate::task_placer::TaskPlacer;

/// A range of datetimes with an interval.
struct DateRange {
	start: NaiveDateTime,
	end: NaiveDateTime,
	interval: Duration,
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

struct DateRangeIter {
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
				.unwrap(),
		);
		let res = (self.current_start, self.current_end);

		self.current_start = min(self.end, self.current_start + self.interval)
			.duration_round(self.interval)
			.unwrap();

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

#[cfg(test)]
mod tests {
	use crate::goal::{Goal, Repetition};
	use crate::input::Input;
	use crate::task_generator::task_generator;

	use super::*;

	#[test]
	fn date_range_iter_simple() {
		let r = DateRange {
			start: NaiveDate::from_ymd(2022, 1, 1).and_hms(0, 0, 0),
			end: NaiveDate::from_ymd(2022, 1, 2).and_hms(0, 0, 0),
			interval: Duration::hours(8),
		};

		assert_eq!(
			r.into_iter().collect::<Vec<_>>(),
			vec![
				(
					NaiveDate::from_ymd(2022, 1, 1).and_hms(0, 0, 0),
					NaiveDate::from_ymd(2022, 1, 1).and_hms(8, 0, 0)
				),
				(
					NaiveDate::from_ymd(2022, 1, 1).and_hms(8, 0, 0),
					NaiveDate::from_ymd(2022, 1, 1).and_hms(16, 0, 0),
				),
				(
					NaiveDate::from_ymd(2022, 1, 1).and_hms(16, 0, 0),
					NaiveDate::from_ymd(2022, 1, 2).and_hms(0, 0, 0),
				),
			]
		)
	}

	#[test]
	fn date_range_single() {
		let r = DateRange {
			start: NaiveDate::from_ymd(2022, 1, 1).and_hms(0, 0, 0),
			end: NaiveDate::from_ymd(2022, 1, 1).and_hms(8, 0, 0),
			interval: Duration::hours(8),
		};

		assert_eq!(
			r.into_iter().collect::<Vec<_>>(),
			vec![(
				NaiveDate::from_ymd(2022, 1, 1).and_hms(0, 0, 0),
				NaiveDate::from_ymd(2022, 1, 1).and_hms(8, 0, 0)
			),]
		)
	}

	#[test]
	fn date_range_single_not_round() {
		let r = DateRange {
			start: NaiveDate::from_ymd(2022, 1, 1).and_hms(1, 0, 0),
			end: NaiveDate::from_ymd(2022, 1, 1).and_hms(7, 0, 0),
			interval: Duration::hours(8),
		};

		assert_eq!(
			r.into_iter().collect::<Vec<_>>(),
			vec![(
				NaiveDate::from_ymd(2022, 1, 1).and_hms(1, 0, 0),
				NaiveDate::from_ymd(2022, 1, 1).and_hms(7, 0, 0)
			),]
		)
	}

	#[test]
	fn date_range_iter_not_round_end() {
		let r = DateRange {
			start: NaiveDate::from_ymd(2022, 1, 1).and_hms(0, 0, 0),
			end: NaiveDate::from_ymd(2022, 1, 1).and_hms(23, 0, 1),
			interval: Duration::hours(8),
		};

		assert_eq!(
			r.into_iter().collect::<Vec<_>>(),
			vec![
				(
					NaiveDate::from_ymd(2022, 1, 1).and_hms(0, 0, 0),
					NaiveDate::from_ymd(2022, 1, 1).and_hms(8, 0, 0)
				),
				(
					NaiveDate::from_ymd(2022, 1, 1).and_hms(8, 0, 0),
					NaiveDate::from_ymd(2022, 1, 1).and_hms(16, 0, 0),
				),
				(
					NaiveDate::from_ymd(2022, 1, 1).and_hms(16, 0, 0),
					NaiveDate::from_ymd(2022, 1, 1).and_hms(23, 0, 1),
				),
			]
		)
	}

	#[test]
	fn date_range_iter_not_round_start() {
		let r = DateRange {
			start: NaiveDate::from_ymd(2022, 1, 1).and_hms(1, 0, 1),
			end: NaiveDate::from_ymd(2022, 1, 2).and_hms(0, 0, 0),
			interval: Duration::hours(8),
		};

		assert_eq!(
			r.into_iter().collect::<Vec<_>>(),
			vec![
				(
					NaiveDate::from_ymd(2022, 1, 1).and_hms(1, 0, 1),
					NaiveDate::from_ymd(2022, 1, 1).and_hms(8, 0, 0),
				),
				(
					NaiveDate::from_ymd(2022, 1, 1).and_hms(8, 0, 0),
					NaiveDate::from_ymd(2022, 1, 1).and_hms(16, 0, 0),
				),
				(
					NaiveDate::from_ymd(2022, 1, 1).and_hms(16, 0, 0),
					NaiveDate::from_ymd(2022, 1, 2).and_hms(0, 0, 0),
				),
			]
		)
	}

	#[test]
	fn date_range_iter_not_round_start_end() {
		let r = DateRange {
			start: NaiveDate::from_ymd(2022, 1, 1).and_hms(1, 0, 1),
			end: NaiveDate::from_ymd(2022, 1, 1).and_hms(23, 0, 1),
			interval: Duration::hours(8),
		};

		assert_eq!(
			r.into_iter().collect::<Vec<_>>(),
			vec![
				(
					NaiveDate::from_ymd(2022, 1, 1).and_hms(1, 0, 1),
					NaiveDate::from_ymd(2022, 1, 1).and_hms(8, 0, 0),
				),
				(
					NaiveDate::from_ymd(2022, 1, 1).and_hms(8, 0, 0),
					NaiveDate::from_ymd(2022, 1, 1).and_hms(16, 0, 0),
				),
				(
					NaiveDate::from_ymd(2022, 1, 1).and_hms(16, 0, 0),
					NaiveDate::from_ymd(2022, 1, 1).and_hms(23, 0, 1),
				),
			]
		)
	}

	#[test]
	fn repeat() {
		let input = Input::new(
			NaiveDate::from_ymd(2022, 1, 1),
			NaiveDate::from_ymd(2022, 1, 4),
			vec![Goal::new(1).duration(1).repetition(Repetition::DAILY)],
		);

		let scheduler = task_generator(input);
		assert_eq!(
			scheduler.tasks,
			vec![Task::new(0, 1, 1), Task::new(1, 1, 1), Task::new(2, 1, 1)]
		);
		assert_eq!(
			scheduler.slots,
			vec![Slot::new(0, 0, 24), Slot::new(1, 24, 48), Slot::new(2, 48, 72)]
		)
	}

	#[test]
	fn repeat_with_goal_start_not_midnight() {
		let input = Input::new(
			NaiveDate::from_ymd(2022, 1, 1),
			NaiveDate::from_ymd(2022, 1, 4),
			vec![Goal::new(1)
				.duration(1)
				.repetition(Repetition::DAILY)
				.start(NaiveDate::from_ymd(2022, 1, 1).and_hms(10, 0, 0))],
		);

		let scheduler = task_generator(input);
		assert_eq!(
			scheduler.tasks,
			vec![Task::new(0, 1, 1), Task::new(1, 1, 1), Task::new(2, 1, 1)]
		);
		assert_eq!(
			scheduler.slots,
			vec![Slot::new(0, 10, 24), Slot::new(1, 24, 48), Slot::new(2, 48, 72)]
		)
	}

	#[test]
	fn repeat_with_goal_end_not_midnight() {
		let input = Input::new(
			NaiveDate::from_ymd(2022, 1, 1),
			NaiveDate::from_ymd(2022, 1, 4),
			vec![Goal::new(1)
				.duration(1)
				.repetition(Repetition::DAILY)
				.deadline(NaiveDate::from_ymd(2022, 1, 3).and_hms(14, 0, 0))],
		);

		let scheduler = task_generator(input);
		assert_eq!(
			scheduler.tasks,
			vec![Task::new(0, 1, 1), Task::new(1, 1, 1), Task::new(2, 1, 1)]
		);
		assert_eq!(
			scheduler.slots,
			vec![Slot::new(0, 0, 24), Slot::new(1, 24, 48), Slot::new(2, 48, 62)]
		)
	}

	#[test]
	fn repeat_with_goal_start_and_end_not_midnight() {
		let input = Input::new(
			NaiveDate::from_ymd(2022, 1, 1),
			NaiveDate::from_ymd(2022, 1, 4),
			vec![Goal::new(1)
				.duration(1)
				.repetition(Repetition::DAILY)
				.start(NaiveDate::from_ymd(2022, 1, 1).and_hms(10, 0, 0))
				.deadline(NaiveDate::from_ymd(2022, 1, 3).and_hms(14, 0, 0))],
		);

		let scheduler = task_generator(input);
		assert_eq!(
			scheduler.tasks,
			vec![Task::new(0, 1, 1), Task::new(1, 1, 1), Task::new(2, 1, 1)]
		);
		assert_eq!(
			scheduler.slots,
			vec![Slot::new(0, 10, 24), Slot::new(1, 24, 48), Slot::new(2, 48, 62)]
		)
	}
}
