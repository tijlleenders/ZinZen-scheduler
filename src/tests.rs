#![allow(unused_imports)]
use crate::{goal::Goal, preprocessor::PreProcessor};
use time::Duration;

#[test]
pub(crate) fn test_preprocessor() {
	let goals = &mut [
		Goal {
			id: 0,
			interval: Duration::MAX,
			duration: Duration::hours(12),
			..Default::default()
		},
		Goal {
			id: 1,
			duration: Duration::hours(12),
			interval: Duration::DAY,
			..Default::default()
		},
		Goal {
			id: 2,
			duration: Duration::hours(12),
			interval: Duration::WEEK,
			..Default::default()
		},
		Goal {
			id: 3,
			duration: Duration::hours(12),
			interval: Duration::WEEK * 4f32,
			..Default::default()
		},
		Goal {
			id: 4,
			duration: Duration::hours(12),
			interval: Duration::WEEK * 4f32,
			..Default::default()
		},
		Goal {
			id: 5,
			duration: Duration::hours(12),
			interval: Duration::WEEK * 52f32,
			..Default::default()
		},
	];

	let tasks = PreProcessor::process_task_count(goals, Duration::hours(24 * 7 * 4 * 2));
	dbg!(tasks);
}

#[test]
pub(crate) fn test_scheduler() {
	use crate::scheduler::Schedule;

	let goals = &mut [
		Goal {
			id: 0,
			interval: Duration::MAX,
			duration: Duration::hours(12),
			..Default::default()
		},
		Goal {
			id: 1,
			duration: Duration::hours(12),
			interval: Duration::DAY,
			..Default::default()
		},
		Goal {
			id: 2,
			duration: Duration::hours(12),
			interval: Duration::WEEK,
			..Default::default()
		},
		Goal {
			id: 3,
			duration: Duration::hours(12),
			interval: Duration::WEEK * 4f32,
			..Default::default()
		},
		Goal {
			id: 5,
			duration: Duration::hours(12),
			interval: Duration::WEEK * 52f32,
			..Default::default()
		},
		Goal {
			id: 5,
			duration: Duration::hours(12),
			interval: Duration::hours(36),
			..Default::default()
		},
	];

	let date_a = time::Date::from_calendar_date(2019, time::Month::June, 1).unwrap();
	let date_b = time::Date::from_calendar_date(2019, time::Month::August, 1).unwrap();

	let timeline = (
		time::PrimitiveDateTime::new(date_a, time::Time::MIDNIGHT),
		time::PrimitiveDateTime::new(date_b, time::Time::MIDNIGHT),
	);

	Schedule::generate_schedule(goals, timeline).unwrap();
}
