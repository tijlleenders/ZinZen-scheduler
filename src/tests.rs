use crate::{goal::*, input::*, task::*, task_generator::*};
use chrono::*;

#[test]
fn basic_test() {
	let input: Input = serde_json::from_str(
		r#"
{
    "startDate": "2022-01-01",
    "endDate": "2022-01-02",
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

	let scheduler = task_generator(input);
	let result = scheduler.task_placer().unwrap();
	let result_json = serde_json::to_string_pretty(&result).unwrap();

	//println!("{}", result_json);
	assert_eq!(
		result_json,
		r#"{
  "tasks": [
    {
      "id": 0,
      "goal_id": 1,
      "duration_to_schedule": 0,
      "duration_scheduled": 1,
      "status": "SCHEDULED"
    },
    {
      "id": 1,
      "goal_id": 2,
      "duration_to_schedule": 0,
      "duration_scheduled": 1,
      "status": "SCHEDULED"
    },
    {
      "id": 2,
      "goal_id": 3,
      "duration_to_schedule": 0,
      "duration_scheduled": 1,
      "status": "SCHEDULED"
    }
  ],
  "slots": [
    {
      "task_id": 0,
      "start": 11,
      "end": 12
    },
    {
      "task_id": 1,
      "start": 10,
      "end": 11
    },
    {
      "task_id": 2,
      "start": 13,
      "end": 14
    }
  ]
}"#
	);
}

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
