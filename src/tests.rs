use time::{Duration, PrimitiveDateTime};

use crate::goal::{self};
#[allow(unused_imports)]
use crate::{
	goal::{Goal, Repetition},
	preprocessor::PreProcessor,
};

#[test]
pub(crate) fn test_preprocessor() {
	let goals = &mut [
		Goal {
			id: 0,
			repetition: Repetition::Once,
			duration: Duration::hours(12),
			..Default::default()
		},
		Goal {
			id: 1,
			duration: Duration::hours(12),
			repetition: Repetition::Daily,
			..Default::default()
		},
		Goal {
			id: 2,
			duration: Duration::hours(12),
			repetition: Repetition::Weekly,
			..Default::default()
		},
		Goal {
			id: 3,
			duration: Duration::hours(12),
			repetition: Repetition::Monthly,
			..Default::default()
		},
		Goal {
			id: 4,
			duration: Duration::hours(12),
			repetition: Repetition::Monthly,
			..Default::default()
		},
		Goal {
			id: 5,
			duration: Duration::hours(12),
			repetition: Repetition::Annually,
			..Default::default()
		},
	];

	let tasks = PreProcessor::process_task_count(goals, Duration::hours(24 * 7 * 4 * 2));

	for (task_count, goal) in tasks {
		let task_count = task_count as usize;

		match goal.repetition {
			Repetition::Exact(x) => assert_eq!(x, task_count),
			Repetition::Once => assert_eq!(task_count, 1),
			Repetition::Daily => assert_eq!(task_count, 48),
			Repetition::Weekly => assert_eq!(task_count, 8),
			Repetition::Monthly => assert_eq!(task_count, 2),
			Repetition::Annually => assert_eq!(task_count, 1),
		}
	}
}

#[test]
pub(crate) fn test_scheduler() {
	use crate::scheduler::Schedule;

	let goals = &mut [
		Goal {
			id: 0,
			repetition: Repetition::Once,
			duration: Duration::hours(12),
			..Default::default()
		},
		Goal {
			id: 1,
			duration: Duration::hours(12),
			repetition: Repetition::Daily,
			..Default::default()
		},
		Goal {
			id: 2,
			duration: Duration::hours(12),
			repetition: Repetition::Weekly,
			..Default::default()
		},
		Goal {
			id: 3,
			duration: Duration::hours(12),
			repetition: Repetition::Monthly,
			..Default::default()
		},
		Goal {
			id: 5,
			duration: Duration::hours(12),
			repetition: Repetition::Annually,
			..Default::default()
		},
		Goal {
			id: 5,
			duration: Duration::hours(12),
			repetition: Repetition::Exact(36),
			..Default::default()
		},
	];

	let date_a = time::Date::from_calendar_date(2019, time::Month::June, 1).unwrap();
	let date_b = time::Date::from_calendar_date(2019, time::Month::August, 1).unwrap();

	let timeline = (
		PrimitiveDateTime::new(date_a, time::Time::MIDNIGHT),
		PrimitiveDateTime::new(date_b, time::Time::MIDNIGHT),
	);

	Schedule::generate_schedule(goals, timeline).unwrap();
}
