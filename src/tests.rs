#![allow(unused_imports)]
use std::num::NonZeroUsize;

use crate::{console, error::{ErrorCode, Explode}, goal::Goal, preprocessor::PreProcessor};
use time::Duration;

#[test]
pub(crate) fn test_preprocessor() {
	let goals = &mut [
		Goal {
			id: NonZeroUsize::new(7).unwrap(),
			interval: None,
			task_duration: Duration::hours(12),
			..Default::default()
		},
		Goal {
			id: NonZeroUsize::new(1).unwrap(),
			task_duration: Duration::hours(12),
			interval: Some(Duration::DAY),
			..Default::default()
		},
		Goal {
			id: NonZeroUsize::new(2).unwrap(),
			task_duration: Duration::hours(12),
			interval: Some(Duration::WEEK),
			..Default::default()
		},
		Goal {
			id: NonZeroUsize::new(3).unwrap(),
			task_duration: Duration::hours(12),
			interval: Some(Duration::WEEK * 4f32),
			..Default::default()
		},
		Goal {
			id: NonZeroUsize::new(4).unwrap(),
			task_duration: Duration::hours(12),
			interval: Some(Duration::WEEK * 4f32),
			..Default::default()
		},
		Goal {
			id: NonZeroUsize::new(5).unwrap(),
			task_duration: Duration::hours(12),
			interval: Some(Duration::WEEK * 52f32),
			..Default::default()
		},
	];

	let tasks = PreProcessor::process_task_count(goals, Duration::hours(24 * 7 * 4 * 2));
	dbg!(tasks);
}

#[test]
pub(crate) fn test_scheduler() {
	use crate::scheduler::generate_schedule;

	let date_a = time::Date::from_calendar_date(2019, time::Month::June, 1).unwrap();
	let date_b = time::Date::from_calendar_date(2019, time::Month::June, 21).unwrap();

	let timeline = (
		time::PrimitiveDateTime::new(date_a, time::Time::MIDNIGHT),
		time::PrimitiveDateTime::new(date_b, time::Time::MIDNIGHT),
	);

	let goals = &mut [
		Goal {
			id: NonZeroUsize::new(1).unwrap(),
			task_duration: Duration::hours(10),
			interval: Some(Duration::DAY),
			..Default::default()
		},
		Goal {
			id: NonZeroUsize::new(2).unwrap(),
			task_duration: Duration::hours(10),
			interval: Some(Duration::DAY),
			..Default::default()
		},
		Goal {
			id: NonZeroUsize::new(3).unwrap(),
			task_duration: Duration::minutes(30),
			interval: Some(Duration::WEEK),
			time_constraint: Some(timeline.0.replace_hour(11).unwrap()),
			..Default::default()
		},
	];

	for task in generate_schedule(goals, timeline).explode().slots {
		dbg!(task);
	}
}
