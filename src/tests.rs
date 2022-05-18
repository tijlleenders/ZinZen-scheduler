#![allow(unused_imports)]
use std::{collections::HashMap, num::NonZeroUsize};

use crate::{
	console,
	error::{ErrorCode, Explode},
	goal::Goal,
	preprocessor::PreProcessor,
};
use time::Duration;

#[test]
pub(crate) fn test_preprocessor() {
	let date_a = time::Date::from_calendar_date(2019, time::Month::June, 1).unwrap();
	let date_b = time::Date::from_calendar_date(2019, time::Month::June, 2).unwrap();

	let timeline = (
		time::PrimitiveDateTime::new(date_a, time::Time::MIDNIGHT),
		time::PrimitiveDateTime::new(date_b, time::Time::MIDNIGHT),
	);

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

	for (idx, task) in PreProcessor::process_task_count(goals, timeline).enumerate() {
		dbg!(idx, task);
	}
}

#[test]
pub(crate) fn test_scheduler() {
	use crate::scheduler::generate_schedule;

	let date_a = time::Date::from_calendar_date(2019, time::Month::June, 1).unwrap();
	let date_b = time::Date::from_calendar_date(2019, time::Month::June, 20).unwrap();

	let timeline = (
		time::PrimitiveDateTime::new(date_a, time::Time::MIDNIGHT),
		time::PrimitiveDateTime::new(date_b, time::Time::MIDNIGHT),
	);

	let goals = &mut [
		Goal {
			id: NonZeroUsize::new(1).explode(),
			description: "shopping".to_string(),
			task_duration: Duration::hours(1),
			interval: Some(Duration::WEEK),
			start: Some(timeline.0.replace_hour(10).unwrap()),
			..Default::default()
		},
		Goal {
			id: NonZeroUsize::new(2).explode(),
			description: "dentist".to_string(),
			task_duration: Duration::hours(2),
			interval: None,
			start: Some(timeline.0.replace_hour(13).unwrap()),
			..Default::default()
		},
		Goal {
			id: NonZeroUsize::new(3).explode(),
			description: "exercise".to_string(),
			task_duration: Duration::hours(1),
			interval: Some(Duration::DAY),
			start: Some(timeline.0.replace_hour(15).unwrap()),
			..Default::default()
		},
	];

	let mut counts = HashMap::new();
	let slots = generate_schedule(goals, timeline).explode().slots;

	for task in slots {
		if let Some(count) = counts.get_mut(&task.goal_id) {
			*count += 1
		} else {
			counts.insert(task.goal_id, 1);
		}

		dbg!(task.flexibility, task.goal_id);
	}

	dbg!(counts);
}

#[test]
pub(crate) fn test_scheduler_02() {
	use crate::scheduler::generate_schedule;

	let date_a = time::Date::from_calendar_date(2019, time::Month::June, 9).unwrap();
	let date_b = time::Date::from_calendar_date(2019, time::Month::June, 20).unwrap();

	let timeline = (
		time::PrimitiveDateTime::new(date_a, time::Time::MIDNIGHT),
		time::PrimitiveDateTime::new(date_b, time::Time::MIDNIGHT),
	);

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

	let mut counts = HashMap::new();
	let slots = generate_schedule(goals, timeline).explode().slots;

	for task in slots {
		if let Some(count) = counts.get_mut(&task.goal_id) {
			*count += 1
		} else {
			counts.insert(task.goal_id, 1);
		}

		dbg!(task.goal_id, task.flexibility);
	}

	dbg!(counts);
}
