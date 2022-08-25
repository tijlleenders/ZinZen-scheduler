use crate::{goal::*, input::*, output_formatter::*, task::*, task_generator::*, task_placer::*};
use chrono::*;

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
fn goal_generates_daily_tasks() {
	let goal = Goal::new(1)
		.duration(1)
		.repetition(Repetition::DAILY)
		.start(NaiveDate::from_ymd(2022, 1, 1).and_hms(0, 0, 0))
		.deadline(NaiveDate::from_ymd(2022, 1, 4).and_hms(0, 0, 0));

	assert_eq!(
		goal.generate_tasks(
			NaiveDate::from_ymd(2022, 1, 1).and_hms(0, 0, 0),
			NaiveDate::from_ymd(2022, 1, 4).and_hms(0, 0, 0)
		),
		vec![
			Task {
				id: 10,
				goal_id: 1,
				title: "Test".to_string(),
				duration: 1,
				status: TaskStatus::UNSCHEDULED,
				flexibility: 24,
				start: NaiveDate::from_ymd(2022, 1, 1).and_hms(0, 0, 0),
				end: NaiveDate::from_ymd(2022, 1, 2).and_hms(0, 0, 0),
				slots: Vec::new(),
			},
			Task {
				id: 11,
				goal_id: 1,
				title: "Test".to_string(),
				duration: 1,
				status: TaskStatus::UNSCHEDULED,
				flexibility: 24,
				start: NaiveDate::from_ymd(2022, 1, 2).and_hms(0, 0, 0),
				end: NaiveDate::from_ymd(2022, 1, 3).and_hms(0, 0, 0),
				slots: Vec::new(),
			},
			Task {
				id: 12,
				goal_id: 1,
				title: "Test".to_string(),
				duration: 1,
				status: TaskStatus::UNSCHEDULED,
				flexibility: 24,
				start: NaiveDate::from_ymd(2022, 1, 3).and_hms(0, 0, 0),
				end: NaiveDate::from_ymd(2022, 1, 4).and_hms(0, 0, 0),
				slots: Vec::new(),
			},
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
			flexibility: 72,
			start: NaiveDate::from_ymd(2022, 1, 1).and_hms(0, 0, 0),
			end: NaiveDate::from_ymd(2022, 1, 4).and_hms(0, 0, 0),
			slots: Vec::new(),
		},]
	)
}

#[test]
fn task_placer_works_simple() {
	let input: Input = serde_json::from_str(
		r#"
{
    "startDate": "2022-01-01T00:00:00",
    "endDate": "2022-01-02T00:00:00",
    "goals": [
        {
          "id": 1,
          "title" : "shopping",
          "duration": 1,
          "start": "2022-01-01T10:00:00",
          "deadline": "2022-01-01T13:00:00"
        },
        {
          "id": 2,
          "title": "dentist",
          "duration": 1,
          "start": "2022-01-01T10:00:00",
          "deadline": "2022-01-01T11:00:00"
        },
        {
          "id": 3,
          "title" : "exercise",
          "duration": 1,
          "start": "2022-01-01T10:00:00",
          "deadline": "2022-01-01T18:00:00"
        }
    ]
}
        "#,
	)
	.unwrap();
	let calendar_start = input.calendar_start;
	let calendar_end = input.calendar_end;
	let mut tasks = task_generator(input);
	task_placer(&mut tasks, calendar_start, calendar_end);

	assert_eq!(
		tasks,
		vec![
			Task {
				id: 20,
				goal_id: 2,
				title: "dentist".to_string(),
				duration: 1,
				status: TaskStatus::SCHEDULED,
				flexibility: 1,
				start: NaiveDate::from_ymd(2022, 1, 1).and_hms(10, 0, 0),
				end: NaiveDate::from_ymd(2022, 1, 1).and_hms(11, 0, 0),
				slots: vec![(
					NaiveDate::from_ymd(2022, 1, 1).and_hms(10, 0, 0),
					NaiveDate::from_ymd(2022, 1, 1).and_hms(11, 0, 0)
				)],
			},
			Task {
				id: 10,
				goal_id: 1,
				title: "shopping".to_string(),
				duration: 1,
				status: TaskStatus::SCHEDULED,
				flexibility: 3,
				start: NaiveDate::from_ymd(2022, 1, 1).and_hms(10, 0, 0),
				end: NaiveDate::from_ymd(2022, 1, 1).and_hms(13, 0, 0),
				slots: vec![(
					NaiveDate::from_ymd(2022, 1, 1).and_hms(11, 0, 0),
					NaiveDate::from_ymd(2022, 1, 1).and_hms(12, 0, 0)
				)],
			},
			Task {
				id: 30,
				goal_id: 3,
				title: "exercise".to_string(),
				duration: 1,
				status: TaskStatus::SCHEDULED,
				flexibility: 8,
				start: NaiveDate::from_ymd(2022, 1, 1).and_hms(10, 0, 0),
				end: NaiveDate::from_ymd(2022, 1, 1).and_hms(18, 0, 0),
				slots: vec![(
					NaiveDate::from_ymd(2022, 1, 1).and_hms(12, 0, 0),
					NaiveDate::from_ymd(2022, 1, 1).and_hms(13, 0, 0)
				)],
			}
		]
	)
}

#[test]
fn task_placer_works_multiple_days() {
	let input: Input = serde_json::from_str(
		r#"
{
    "startDate": "2022-01-01T00:00:00",
    "endDate": "2022-01-04T00:00:00",
    "goals": [
        {
          "id": 1,
          "title" : "shopping",
          "duration": 1,
          "start": "2022-01-01T10:00:00",
          "deadline": "2022-01-01T13:00:00"
        },
        {
          "id": 2,
          "title": "dentist",
          "duration": 1,
          "start": "2022-01-02T10:00:00",
          "deadline": "2022-01-02T11:00:00"
        },
        {
          "id": 3,
          "title" : "exercise",
          "duration": 1,
          "start": "2022-01-03T10:00:00",
          "deadline": "2022-01-03T18:00:00"
        }
    ]
}
        "#,
	)
	.unwrap();
	let calendar_start = input.calendar_start;
	let calendar_end = input.calendar_end;
	let mut tasks = task_generator(input);
	task_placer(&mut tasks, calendar_start, calendar_end);

	assert_eq!(
		tasks,
		vec![
			Task {
				id: 20,
				goal_id: 2,
				title: "dentist".to_string(),
				duration: 1,
				status: TaskStatus::SCHEDULED,
				flexibility: 1,
				start: NaiveDate::from_ymd(2022, 1, 2).and_hms(10, 0, 0),
				end: NaiveDate::from_ymd(2022, 1, 2).and_hms(11, 0, 0),
				slots: vec![(
					NaiveDate::from_ymd(2022, 1, 2).and_hms(10, 0, 0),
					NaiveDate::from_ymd(2022, 1, 2).and_hms(11, 0, 0)
				)],
			},
			Task {
				id: 10,
				goal_id: 1,
				title: "shopping".to_string(),
				duration: 1,
				status: TaskStatus::SCHEDULED,
				flexibility: 3,
				start: NaiveDate::from_ymd(2022, 1, 1).and_hms(10, 0, 0),
				end: NaiveDate::from_ymd(2022, 1, 1).and_hms(13, 0, 0),
				slots: vec![(
					NaiveDate::from_ymd(2022, 1, 1).and_hms(10, 0, 0),
					NaiveDate::from_ymd(2022, 1, 1).and_hms(11, 0, 0)
				)],
			},
			Task {
				id: 30,
				goal_id: 3,
				title: "exercise".to_string(),
				duration: 1,
				status: TaskStatus::SCHEDULED,
				flexibility: 8,
				start: NaiveDate::from_ymd(2022, 1, 3).and_hms(10, 0, 0),
				end: NaiveDate::from_ymd(2022, 1, 3).and_hms(18, 0, 0),
				slots: vec![(
					NaiveDate::from_ymd(2022, 1, 3).and_hms(10, 0, 0),
					NaiveDate::from_ymd(2022, 1, 3).and_hms(11, 0, 0)
				)],
			}
		]
	)
}

#[test]
fn task_placer_works_long_duration_multiple_days() {
	let input: Input = serde_json::from_str(
		r#"
{
    "startDate": "2022-01-01T00:00:00",
    "endDate": "2022-01-04T00:00:00",
    "goals": [
        {
          "id": 1,
          "title" : "shopping",
          "duration": 2,
          "start": "2022-01-01T10:00:00",
          "deadline": "2022-01-01T13:00:00"
        },
        {
          "id": 2,
          "title": "dentist",
          "duration": 1,
          "start": "2022-01-01T10:00:00",
          "deadline": "2022-01-01T11:00:00"
        },
        {
          "id": 3,
          "title" : "exercise",
          "duration": 4,
          "start": "2022-01-03T10:00:00",
          "deadline": "2022-01-03T18:00:00"
        }
    ]
}
        "#,
	)
	.unwrap();
	let calendar_start = input.calendar_start;
	let calendar_end = input.calendar_end;
	let mut tasks = task_generator(input);
	task_placer(&mut tasks, calendar_start, calendar_end);

	assert_eq!(
		tasks,
		vec![
			Task {
				id: 20,
				goal_id: 2,
				title: "dentist".to_string(),
				duration: 1,
				status: TaskStatus::SCHEDULED,
				flexibility: 1,
				start: NaiveDate::from_ymd(2022, 1, 1).and_hms(10, 0, 0),
				end: NaiveDate::from_ymd(2022, 1, 1).and_hms(11, 0, 0),
				slots: vec![(
					NaiveDate::from_ymd(2022, 1, 1).and_hms(10, 0, 0),
					NaiveDate::from_ymd(2022, 1, 1).and_hms(11, 0, 0)
				)],
			},
			Task {
				id: 10,
				goal_id: 1,
				title: "shopping".to_string(),
				duration: 2,
				status: TaskStatus::SCHEDULED,
				flexibility: 3,
				start: NaiveDate::from_ymd(2022, 1, 1).and_hms(10, 0, 0),
				end: NaiveDate::from_ymd(2022, 1, 1).and_hms(13, 0, 0),
				slots: vec![
					(
						NaiveDate::from_ymd(2022, 1, 1).and_hms(11, 0, 0),
						NaiveDate::from_ymd(2022, 1, 1).and_hms(12, 0, 0)
					),
					(
						NaiveDate::from_ymd(2022, 1, 1).and_hms(12, 0, 0),
						NaiveDate::from_ymd(2022, 1, 1).and_hms(13, 0, 0)
					)
				],
			},
			Task {
				id: 30,
				goal_id: 3,
				title: "exercise".to_string(),
				duration: 4,
				status: TaskStatus::SCHEDULED,
				flexibility: 8,
				start: NaiveDate::from_ymd(2022, 1, 3).and_hms(10, 0, 0),
				end: NaiveDate::from_ymd(2022, 1, 3).and_hms(18, 0, 0),
				slots: vec![
					(
						NaiveDate::from_ymd(2022, 1, 3).and_hms(10, 0, 0),
						NaiveDate::from_ymd(2022, 1, 3).and_hms(11, 0, 0)
					),
					(
						NaiveDate::from_ymd(2022, 1, 3).and_hms(11, 0, 0),
						NaiveDate::from_ymd(2022, 1, 3).and_hms(12, 0, 0)
					),
					(
						NaiveDate::from_ymd(2022, 1, 3).and_hms(12, 0, 0),
						NaiveDate::from_ymd(2022, 1, 3).and_hms(13, 0, 0)
					),
					(
						NaiveDate::from_ymd(2022, 1, 3).and_hms(13, 0, 0),
						NaiveDate::from_ymd(2022, 1, 3).and_hms(14, 0, 0)
					)
				],
			}
		]
	)
}

#[test]
fn output_formatter_works() {
	/*
	Below is a pretty-printed version of the test output for easier reference.
	The one used in the actual test doesn't have line breaks and spaces and
	is therefore hard to read.

	let desired_output = r#"
	[
	  {
		"taskid": 20,
		"goalid": 2,
		"title": "dentist",
		"duration": 1,
		"start": "2022-01-01T10:00:00"
		"deadline": "2022-01-01T11:00:00"
	  },
	  {
		"taskid": 10,
		"goalid": 1,
		"title": "shopping",
		"duration": 1,
		"start": "2022-01-01T11:00:00"
		"deadline": "2022-01-01T12:00:00"
	  },
	  {
		"taskid": 30,
		"goalid": 3,
		"title": "exercise",
		"duration": 1,
		"start": "2022-01-01T12:00:00"
		"deadline": "2022-01-01T13:00:00"
	  }
	]
			"#;*/

	let desired_output = r#"[{"taskid":20,"goalid":2,"title":"dentist","duration":1,"start":"2022-01-01T10:00:00","deadline":"2022-01-01T11:00:00"},{"taskid":10,"goalid":1,"title":"shopping","duration":1,"start":"2022-01-01T11:00:00","deadline":"2022-01-01T12:00:00"},{"taskid":30,"goalid":3,"title":"exercise","duration":1,"start":"2022-01-01T12:00:00","deadline":"2022-01-01T13:00:00"}]"#;

	let tasks = vec![
		Task {
			id: 20,
			goal_id: 2,
			title: "dentist".to_string(),
			duration: 1,
			status: TaskStatus::SCHEDULED,
			flexibility: 1,
			start: NaiveDate::from_ymd(2022, 1, 1).and_hms(10, 0, 0),
			end: NaiveDate::from_ymd(2022, 1, 1).and_hms(11, 0, 0),
			slots: vec![(
				NaiveDate::from_ymd(2022, 1, 1).and_hms(10, 0, 0),
				NaiveDate::from_ymd(2022, 1, 1).and_hms(11, 0, 0),
			)],
		},
		Task {
			id: 10,
			goal_id: 1,
			title: "shopping".to_string(),
			duration: 1,
			status: TaskStatus::SCHEDULED,
			flexibility: 3,
			start: NaiveDate::from_ymd(2022, 1, 1).and_hms(10, 0, 0),
			end: NaiveDate::from_ymd(2022, 1, 1).and_hms(13, 0, 0),
			slots: vec![(
				NaiveDate::from_ymd(2022, 1, 1).and_hms(11, 0, 0),
				NaiveDate::from_ymd(2022, 1, 1).and_hms(12, 0, 0),
			)],
		},
		Task {
			id: 30,
			goal_id: 3,
			title: "exercise".to_string(),
			duration: 1,
			status: TaskStatus::SCHEDULED,
			flexibility: 8,
			start: NaiveDate::from_ymd(2022, 1, 1).and_hms(10, 0, 0),
			end: NaiveDate::from_ymd(2022, 1, 1).and_hms(18, 0, 0),
			slots: vec![(
				NaiveDate::from_ymd(2022, 1, 1).and_hms(12, 0, 0),
				NaiveDate::from_ymd(2022, 1, 1).and_hms(13, 0, 0),
			)],
		},
	];

	let output = output_formatter(tasks).unwrap();
	assert_eq!(desired_output, serde_json::to_string(&output).unwrap());
}
