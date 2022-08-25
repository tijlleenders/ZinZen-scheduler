use crate::{goal::*, input::*, output_formatter::*, task::TaskStatus::*, task::*, task_generator::*, task_placer::*};
use chrono::*;

fn get_test_tasks() -> Vec<Task> {
	vec![
		Task {
			id: 20,
			goal_id: 2,
			title: "dentist".to_string(),
			duration: 1,
			status: TaskStatus::UNSCHEDULED,
			flexibility: 0,
			start: NaiveDate::from_ymd(2022, 1, 1).and_hms(10, 0, 0),
			deadline: NaiveDate::from_ymd(2022, 1, 1).and_hms(11, 0, 0),
			slots: vec![(
				NaiveDate::from_ymd(2022, 1, 1).and_hms(10, 0, 0),
				NaiveDate::from_ymd(2022, 1, 1).and_hms(11, 0, 0),
			)],
			confirmed_start: None,
			confirmed_deadline: None,
		},
		Task {
			id: 10,
			goal_id: 1,
			title: "shopping".to_string(),
			duration: 1,
			status: TaskStatus::UNSCHEDULED,
			flexibility: 0,
			start: NaiveDate::from_ymd(2022, 1, 1).and_hms(10, 0, 0),
			deadline: NaiveDate::from_ymd(2022, 1, 1).and_hms(13, 0, 0),
			slots: vec![
				(
					NaiveDate::from_ymd(2022, 1, 1).and_hms(10, 0, 0),
					NaiveDate::from_ymd(2022, 1, 1).and_hms(11, 0, 0),
				),
				(
					NaiveDate::from_ymd(2022, 1, 1).and_hms(11, 0, 0),
					NaiveDate::from_ymd(2022, 1, 1).and_hms(12, 0, 0),
				),
				(
					NaiveDate::from_ymd(2022, 1, 1).and_hms(12, 0, 0),
					NaiveDate::from_ymd(2022, 1, 1).and_hms(13, 0, 0),
				),
			],
			confirmed_start: None,
			confirmed_deadline: None,
		},
		Task {
			id: 30,
			goal_id: 3,
			title: "exercise".to_string(),
			duration: 1,
			status: TaskStatus::UNSCHEDULED,
			flexibility: 0,
			start: NaiveDate::from_ymd(2022, 1, 1).and_hms(10, 0, 0),
			deadline: NaiveDate::from_ymd(2022, 1, 1).and_hms(18, 0, 0),
			slots: vec![
				(
					NaiveDate::from_ymd(2022, 1, 1).and_hms(10, 0, 0),
					NaiveDate::from_ymd(2022, 1, 1).and_hms(11, 0, 0),
				),
				(
					NaiveDate::from_ymd(2022, 1, 1).and_hms(11, 0, 0),
					NaiveDate::from_ymd(2022, 1, 1).and_hms(12, 0, 0),
				),
				(
					NaiveDate::from_ymd(2022, 1, 1).and_hms(12, 0, 0),
					NaiveDate::from_ymd(2022, 1, 1).and_hms(13, 0, 0),
				),
				(
					NaiveDate::from_ymd(2022, 1, 1).and_hms(13, 0, 0),
					NaiveDate::from_ymd(2022, 1, 1).and_hms(14, 0, 0),
				),
				(
					NaiveDate::from_ymd(2022, 1, 1).and_hms(14, 0, 0),
					NaiveDate::from_ymd(2022, 1, 1).and_hms(15, 0, 0),
				),
				(
					NaiveDate::from_ymd(2022, 1, 1).and_hms(15, 0, 0),
					NaiveDate::from_ymd(2022, 1, 1).and_hms(16, 0, 0),
				),
				(
					NaiveDate::from_ymd(2022, 1, 1).and_hms(16, 0, 0),
					NaiveDate::from_ymd(2022, 1, 1).and_hms(17, 0, 0),
				),
				(
					NaiveDate::from_ymd(2022, 1, 1).and_hms(17, 0, 0),
					NaiveDate::from_ymd(2022, 1, 1).and_hms(18, 0, 0),
				),
			],
			confirmed_start: None,
			confirmed_deadline: None,
		},
	]
}

fn get_calendar_bounds() -> (NaiveDateTime, NaiveDateTime) {
	(
		(NaiveDate::from_ymd(2022, 1, 1).and_hms(0, 0, 0)),
		NaiveDate::from_ymd(2022, 1, 2).and_hms(0, 0, 0),
	)
}

#[test]
fn date_range_iter_simple() {
	let r = DateRange {
		start: NaiveDate::from_ymd(2022, 1, 1).and_hms(0, 0, 0),
		end: NaiveDate::from_ymd(2022, 1, 2).and_hms(0, 0, 0),
		interval: Some(Duration::hours(8)),
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
		interval: Some(Duration::hours(8)),
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
		interval: Some(Duration::hours(8)),
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
		interval: Some(Duration::hours(8)),
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
		interval: Some(Duration::hours(8)),
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
		interval: Some(Duration::hours(8)),
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
fn date_range_splits_into_single_days() {
	let r = DateRange {
		start: NaiveDate::from_ymd(2022, 1, 1).and_hms(0, 0, 0),
		end: NaiveDate::from_ymd(2022, 1, 7).and_hms(23, 59, 59),
		interval: Some(Duration::days(1)),
	};

	assert_eq!(
		r.into_iter().collect::<Vec<_>>(),
		vec![
			(
				NaiveDate::from_ymd(2022, 1, 1).and_hms(0, 0, 0),
				NaiveDate::from_ymd(2022, 1, 2).and_hms(0, 0, 0),
			),
			(
				NaiveDate::from_ymd(2022, 1, 2).and_hms(0, 0, 0),
				NaiveDate::from_ymd(2022, 1, 3).and_hms(0, 0, 0),
			),
			(
				NaiveDate::from_ymd(2022, 1, 3).and_hms(0, 0, 0),
				NaiveDate::from_ymd(2022, 1, 4).and_hms(0, 0, 0),
			),
			(
				NaiveDate::from_ymd(2022, 1, 4).and_hms(0, 0, 0),
				NaiveDate::from_ymd(2022, 1, 5).and_hms(0, 0, 0),
			),
			(
				NaiveDate::from_ymd(2022, 1, 5).and_hms(0, 0, 0),
				NaiveDate::from_ymd(2022, 1, 6).and_hms(0, 0, 0),
			),
			(
				NaiveDate::from_ymd(2022, 1, 6).and_hms(0, 0, 0),
				NaiveDate::from_ymd(2022, 1, 7).and_hms(0, 0, 0),
			),
			(
				NaiveDate::from_ymd(2022, 1, 7).and_hms(0, 0, 0),
				NaiveDate::from_ymd(2022, 1, 7).and_hms(23, 59, 59),
			),
		]
	)
}

#[test]
fn date_range_splits_day_into_24_hrs() {
	let r = DateRange {
		start: NaiveDate::from_ymd(2022, 1, 1).and_hms(0, 0, 0),
		end: NaiveDate::from_ymd(2022, 1, 2).and_hms(0, 0, 0),
		interval: Some(Duration::hours(1)),
	};

	assert_eq!(
		r.into_iter().collect::<Vec<_>>(),
		vec![
			(
				NaiveDate::from_ymd(2022, 1, 1).and_hms(0, 0, 0),
				NaiveDate::from_ymd(2022, 1, 1).and_hms(1, 0, 0),
			),
			(
				NaiveDate::from_ymd(2022, 1, 1).and_hms(1, 0, 0),
				NaiveDate::from_ymd(2022, 1, 1).and_hms(2, 0, 0),
			),
			(
				NaiveDate::from_ymd(2022, 1, 1).and_hms(2, 0, 0),
				NaiveDate::from_ymd(2022, 1, 1).and_hms(3, 0, 0),
			),
			(
				NaiveDate::from_ymd(2022, 1, 1).and_hms(3, 0, 0),
				NaiveDate::from_ymd(2022, 1, 1).and_hms(4, 0, 0),
			),
			(
				NaiveDate::from_ymd(2022, 1, 1).and_hms(4, 0, 0),
				NaiveDate::from_ymd(2022, 1, 1).and_hms(5, 0, 0),
			),
			(
				NaiveDate::from_ymd(2022, 1, 1).and_hms(5, 0, 0),
				NaiveDate::from_ymd(2022, 1, 1).and_hms(6, 0, 0),
			),
			(
				NaiveDate::from_ymd(2022, 1, 1).and_hms(6, 0, 0),
				NaiveDate::from_ymd(2022, 1, 1).and_hms(7, 0, 0),
			),
			(
				NaiveDate::from_ymd(2022, 1, 1).and_hms(7, 0, 0),
				NaiveDate::from_ymd(2022, 1, 1).and_hms(8, 0, 0),
			),
			(
				NaiveDate::from_ymd(2022, 1, 1).and_hms(8, 0, 0),
				NaiveDate::from_ymd(2022, 1, 1).and_hms(9, 0, 0),
			),
			(
				NaiveDate::from_ymd(2022, 1, 1).and_hms(9, 0, 0),
				NaiveDate::from_ymd(2022, 1, 1).and_hms(10, 0, 0),
			),
			(
				NaiveDate::from_ymd(2022, 1, 1).and_hms(10, 0, 0),
				NaiveDate::from_ymd(2022, 1, 1).and_hms(11, 0, 0),
			),
			(
				NaiveDate::from_ymd(2022, 1, 1).and_hms(11, 0, 0),
				NaiveDate::from_ymd(2022, 1, 1).and_hms(12, 0, 0),
			),
			(
				NaiveDate::from_ymd(2022, 1, 1).and_hms(12, 0, 0),
				NaiveDate::from_ymd(2022, 1, 1).and_hms(13, 0, 0),
			),
			(
				NaiveDate::from_ymd(2022, 1, 1).and_hms(13, 0, 0),
				NaiveDate::from_ymd(2022, 1, 1).and_hms(14, 0, 0),
			),
			(
				NaiveDate::from_ymd(2022, 1, 1).and_hms(14, 0, 0),
				NaiveDate::from_ymd(2022, 1, 1).and_hms(15, 0, 0),
			),
			(
				NaiveDate::from_ymd(2022, 1, 1).and_hms(15, 0, 0),
				NaiveDate::from_ymd(2022, 1, 1).and_hms(16, 0, 0),
			),
			(
				NaiveDate::from_ymd(2022, 1, 1).and_hms(16, 0, 0),
				NaiveDate::from_ymd(2022, 1, 1).and_hms(17, 0, 0),
			),
			(
				NaiveDate::from_ymd(2022, 1, 1).and_hms(17, 0, 0),
				NaiveDate::from_ymd(2022, 1, 1).and_hms(18, 0, 0),
			),
			(
				NaiveDate::from_ymd(2022, 1, 1).and_hms(18, 0, 0),
				NaiveDate::from_ymd(2022, 1, 1).and_hms(19, 0, 0),
			),
			(
				NaiveDate::from_ymd(2022, 1, 1).and_hms(19, 0, 0),
				NaiveDate::from_ymd(2022, 1, 1).and_hms(20, 0, 0),
			),
			(
				NaiveDate::from_ymd(2022, 1, 1).and_hms(20, 0, 0),
				NaiveDate::from_ymd(2022, 1, 1).and_hms(21, 0, 0),
			),
			(
				NaiveDate::from_ymd(2022, 1, 1).and_hms(21, 0, 0),
				NaiveDate::from_ymd(2022, 1, 1).and_hms(22, 0, 0),
			),
			(
				NaiveDate::from_ymd(2022, 1, 1).and_hms(22, 0, 0),
				NaiveDate::from_ymd(2022, 1, 1).and_hms(23, 0, 0),
			),
			(
				NaiveDate::from_ymd(2022, 1, 1).and_hms(23, 0, 0),
				NaiveDate::from_ymd(2022, 1, 2).and_hms(0, 0, 0),
			),
		]
	)
}

#[test]
fn goal_generates_single_nonrepetitive_task() {
	let goal = Goal::new(1)
		.duration(1)
		.start(NaiveDate::from_ymd(2022, 1, 1).and_hms(0, 0, 0))
		.deadline(NaiveDate::from_ymd(2022, 1, 4).and_hms(0, 0, 0));

	assert_eq!(
		goal.generate_tasks(
			NaiveDate::from_ymd(2022, 1, 1).and_hms(0, 0, 0),
			NaiveDate::from_ymd(2022, 1, 4).and_hms(0, 0, 0)
		),
		vec![Task {
			id: 10,
			goal_id: 1,
			title: "Test".to_string(),
			duration: 1,
			status: TaskStatus::UNSCHEDULED,
			flexibility: 0,
			start: NaiveDate::from_ymd(2022, 1, 1).and_hms(0, 0, 0),
			deadline: NaiveDate::from_ymd(2022, 1, 4).and_hms(0, 0, 0),
			slots: Vec::new(),
			confirmed_start: None,
			confirmed_deadline: None,
		},]
	)
}

#[test]
fn output_formatter_works() {
	let desired_output = r#"[{"taskid":20,"goalid":2,"title":"dentist","duration":1,"start":"2022-01-01T10:00:00","deadline":"2022-01-01T11:00:00"},{"taskid":30,"goalid":3,"title":"exercise","duration":1,"start":"2022-01-01T13:00:00","deadline":"2022-01-01T14:00:00"},{"taskid":10,"goalid":1,"title":"shopping","duration":1,"start":"2022-01-01T11:00:00","deadline":"2022-01-01T12:00:00"}]"#;

	let (calendar_start, calendar_end) = get_calendar_bounds();
	let scheduled_tasks = task_placer(get_test_tasks(), calendar_start, calendar_end);
	let output = output_formatter(scheduled_tasks).unwrap();
	assert_eq!(desired_output, serde_json::to_string(&output).unwrap());
}

#[test]
fn calculate_flexibility_works() {
	let mut tasks = get_test_tasks();
	for task in &mut tasks {
		task.calculate_flexibility();
	}
	assert_eq!(tasks[0].flexibility, 1);
	assert_eq!(tasks[1].flexibility, 3);
	assert_eq!(tasks[2].flexibility, 8);
}

#[test]
fn task_placer_slots_tasks_correctly() {
	let tasks = get_test_tasks();
	let (calendar_start, calendar_end) = get_calendar_bounds();
	let scheduled_tasks = task_placer(tasks, calendar_start, calendar_end);
	assert_eq!(scheduled_tasks[0].status, SCHEDULED);
	assert_eq!(scheduled_tasks[1].status, SCHEDULED);
	assert_eq!(scheduled_tasks[2].status, SCHEDULED);
	assert_eq!(
		scheduled_tasks[0].confirmed_start.unwrap(),
		NaiveDate::from_ymd(2022, 1, 1).and_hms(10, 0, 0)
	);
	assert_eq!(
		scheduled_tasks[0].confirmed_deadline.unwrap(),
		NaiveDate::from_ymd(2022, 1, 1).and_hms(11, 0, 0)
	);
	assert_eq!(
		scheduled_tasks[1].confirmed_start.unwrap(),
		NaiveDate::from_ymd(2022, 1, 1).and_hms(13, 0, 0)
	);
	assert_eq!(
		scheduled_tasks[1].confirmed_deadline.unwrap(),
		NaiveDate::from_ymd(2022, 1, 1).and_hms(14, 0, 0)
	);
	assert_eq!(
		scheduled_tasks[2].confirmed_start.unwrap(),
		NaiveDate::from_ymd(2022, 1, 1).and_hms(11, 0, 0)
	);
	assert_eq!(
		scheduled_tasks[2].confirmed_deadline.unwrap(),
		NaiveDate::from_ymd(2022, 1, 1).and_hms(12, 0, 0)
	);
}
