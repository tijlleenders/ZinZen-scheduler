use crate::{
    goal::*, input::*, output_formatter::*, task::TaskStatus::*, task::*, task_generator::*,
    task_placer::*, time_slice_iterator::*,
};
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
            start: NaiveDate::from_ymd(2022, 1, 1).and_hms(0, 0, 0),
            deadline: NaiveDate::from_ymd(2022, 1, 2).and_hms(0, 0, 0),
            after_time: 10,
            before_time: 11,
            slots: Vec::new(),
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
            start: NaiveDate::from_ymd(2022, 1, 1).and_hms(0, 0, 0),
            deadline: NaiveDate::from_ymd(2022, 1, 2).and_hms(0, 0, 0),
            after_time: 10,
            before_time: 13,
            slots: Vec::new(),
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
            start: NaiveDate::from_ymd(2022, 1, 1).and_hms(0, 0, 0),
            deadline: NaiveDate::from_ymd(2022, 1, 2).and_hms(0, 0, 0),
            after_time: 10,
            before_time: 18,
            slots: Vec::new(),
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
fn time_slice_iterator_splits_into_single_days() {
    let r = TimeSliceIterator {
        start: NaiveDate::from_ymd(2022, 1, 1).and_hms(0, 0, 0),
        end: NaiveDate::from_ymd(2022, 1, 7).and_hms(23, 59, 59),
        repetition: Repetition::DAILY,
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
fn time_slice_iterator_returns_all_mondays() {
    let r = TimeSliceIterator {
        start: NaiveDate::from_ymd(2022, 9, 1).and_hms(0, 0, 0),
        end: NaiveDate::from_ymd(2022, 9, 30).and_hms(0, 0, 0),
        repetition: Repetition::MONDAYS,
    };

    assert_eq!(
        r.into_iter().collect::<Vec<_>>(),
        vec![
            (
                NaiveDate::from_ymd(2022, 9, 5).and_hms(0, 0, 0),
                NaiveDate::from_ymd(2022, 9, 6).and_hms(0, 0, 0),
            ),
            (
                NaiveDate::from_ymd(2022, 9, 12).and_hms(0, 0, 0),
                NaiveDate::from_ymd(2022, 9, 13).and_hms(0, 0, 0),
            ),
            (
                NaiveDate::from_ymd(2022, 9, 19).and_hms(0, 0, 0),
                NaiveDate::from_ymd(2022, 9, 20).and_hms(0, 0, 0),
            ),
            (
                NaiveDate::from_ymd(2022, 9, 26).and_hms(0, 0, 0),
                NaiveDate::from_ymd(2022, 9, 27).and_hms(0, 0, 0),
            ),
        ]
    )
}

#[test]
fn time_slice_iterator_range_returns_all_tuesdays() {
    let r = TimeSliceIterator {
        start: NaiveDate::from_ymd(2022, 9, 1).and_hms(0, 0, 0),
        end: NaiveDate::from_ymd(2022, 9, 30).and_hms(0, 0, 0),
        repetition: Repetition::TUESDAYS,
    };

    assert_eq!(
        r.into_iter().collect::<Vec<_>>(),
        vec![
            (
                NaiveDate::from_ymd(2022, 9, 6).and_hms(0, 0, 0),
                NaiveDate::from_ymd(2022, 9, 7).and_hms(0, 0, 0),
            ),
            (
                NaiveDate::from_ymd(2022, 9, 13).and_hms(0, 0, 0),
                NaiveDate::from_ymd(2022, 9, 14).and_hms(0, 0, 0),
            ),
            (
                NaiveDate::from_ymd(2022, 9, 20).and_hms(0, 0, 0),
                NaiveDate::from_ymd(2022, 9, 21).and_hms(0, 0, 0),
            ),
            (
                NaiveDate::from_ymd(2022, 9, 27).and_hms(0, 0, 0),
                NaiveDate::from_ymd(2022, 9, 28).and_hms(0, 0, 0),
            ),
        ]
    )
}

#[test]
fn time_slice_iterator_range_returns_all_wednesdays() {
    let r = TimeSliceIterator {
        start: NaiveDate::from_ymd(2022, 9, 1).and_hms(0, 0, 0),
        end: NaiveDate::from_ymd(2022, 9, 30).and_hms(0, 0, 0),
        repetition: Repetition::WEDNESDAYS,
    };

    assert_eq!(
        r.into_iter().collect::<Vec<_>>(),
        vec![
            (
                NaiveDate::from_ymd(2022, 9, 7).and_hms(0, 0, 0),
                NaiveDate::from_ymd(2022, 9, 8).and_hms(0, 0, 0),
            ),
            (
                NaiveDate::from_ymd(2022, 9, 14).and_hms(0, 0, 0),
                NaiveDate::from_ymd(2022, 9, 15).and_hms(0, 0, 0),
            ),
            (
                NaiveDate::from_ymd(2022, 9, 21).and_hms(0, 0, 0),
                NaiveDate::from_ymd(2022, 9, 22).and_hms(0, 0, 0),
            ),
            (
                NaiveDate::from_ymd(2022, 9, 28).and_hms(0, 0, 0),
                NaiveDate::from_ymd(2022, 9, 29).and_hms(0, 0, 0),
            ),
        ]
    )
}

fn time_slice_iterator_range_returns_all_thursdays() {
    let r = TimeSliceIterator {
        start: NaiveDate::from_ymd(2022, 9, 1).and_hms(0, 0, 0),
        end: NaiveDate::from_ymd(2022, 9, 30).and_hms(0, 0, 0),
        repetition: Repetition::THURSDAYS,
    };

    assert_eq!(
        r.into_iter().collect::<Vec<_>>(),
        vec![
            (
                NaiveDate::from_ymd(2022, 9, 1).and_hms(0, 0, 0),
                NaiveDate::from_ymd(2022, 9, 2).and_hms(0, 0, 0),
            ),
            (
                NaiveDate::from_ymd(2022, 9, 8).and_hms(0, 0, 0),
                NaiveDate::from_ymd(2022, 9, 9).and_hms(0, 0, 0),
            ),
            (
                NaiveDate::from_ymd(2022, 9, 15).and_hms(0, 0, 0),
                NaiveDate::from_ymd(2022, 9, 16).and_hms(0, 0, 0),
            ),
            (
                NaiveDate::from_ymd(2022, 9, 22).and_hms(0, 0, 0),
                NaiveDate::from_ymd(2022, 9, 23).and_hms(0, 0, 0),
            ),
            (
                NaiveDate::from_ymd(2022, 9, 29).and_hms(0, 0, 0),
                NaiveDate::from_ymd(2022, 9, 30).and_hms(0, 0, 0),
            ),
        ]
    )
}

fn time_slice_iterator_range_returns_all_fridays() {
    let r = TimeSliceIterator {
        start: NaiveDate::from_ymd(2022, 9, 1).and_hms(0, 0, 0),
        end: NaiveDate::from_ymd(2022, 9, 30).and_hms(0, 0, 0),
        repetition: Repetition::FRIDAYS,
    };

    assert_eq!(
        r.into_iter().collect::<Vec<_>>(),
        vec![
            (
                NaiveDate::from_ymd(2022, 9, 2).and_hms(0, 0, 0),
                NaiveDate::from_ymd(2022, 9, 3).and_hms(0, 0, 0),
            ),
            (
                NaiveDate::from_ymd(2022, 9, 9).and_hms(0, 0, 0),
                NaiveDate::from_ymd(2022, 9, 10).and_hms(0, 0, 0),
            ),
            (
                NaiveDate::from_ymd(2022, 9, 16).and_hms(0, 0, 0),
                NaiveDate::from_ymd(2022, 9, 17).and_hms(0, 0, 0),
            ),
            (
                NaiveDate::from_ymd(2022, 9, 23).and_hms(0, 0, 0),
                NaiveDate::from_ymd(2022, 9, 24).and_hms(0, 0, 0),
            ),
            (
                NaiveDate::from_ymd(2022, 9, 30).and_hms(0, 0, 0),
                NaiveDate::from_ymd(2022, 10, 1).and_hms(0, 0, 0),
            ),
        ]
    )
}

fn time_slice_iterator_range_returns_all_saturdays() {
    let r = TimeSliceIterator {
        start: NaiveDate::from_ymd(2022, 9, 1).and_hms(0, 0, 0),
        end: NaiveDate::from_ymd(2022, 9, 30).and_hms(0, 0, 0),
        repetition: Repetition::SATURDAYS,
    };

    assert_eq!(
        r.into_iter().collect::<Vec<_>>(),
        vec![
            (
                NaiveDate::from_ymd(2022, 9, 3).and_hms(0, 0, 0),
                NaiveDate::from_ymd(2022, 9, 4).and_hms(0, 0, 0),
            ),
            (
                NaiveDate::from_ymd(2022, 9, 10).and_hms(0, 0, 0),
                NaiveDate::from_ymd(2022, 9, 11).and_hms(0, 0, 0),
            ),
            (
                NaiveDate::from_ymd(2022, 9, 17).and_hms(0, 0, 0),
                NaiveDate::from_ymd(2022, 9, 18).and_hms(0, 0, 0),
            ),
            (
                NaiveDate::from_ymd(2022, 9, 24).and_hms(0, 0, 0),
                NaiveDate::from_ymd(2022, 9, 25).and_hms(0, 0, 0),
            ),
        ]
    )
}

fn time_slice_iterator_range_returns_all_sundays() {
    let r = TimeSliceIterator {
        start: NaiveDate::from_ymd(2022, 9, 1).and_hms(0, 0, 0),
        end: NaiveDate::from_ymd(2022, 9, 30).and_hms(0, 0, 0),
        repetition: Repetition::SUNDAYS,
    };

    assert_eq!(
        r.into_iter().collect::<Vec<_>>(),
        vec![
            (
                NaiveDate::from_ymd(2022, 9, 4).and_hms(0, 0, 0),
                NaiveDate::from_ymd(2022, 9, 5).and_hms(0, 0, 0),
            ),
            (
                NaiveDate::from_ymd(2022, 9, 11).and_hms(0, 0, 0),
                NaiveDate::from_ymd(2022, 9, 12).and_hms(0, 0, 0),
            ),
            (
                NaiveDate::from_ymd(2022, 9, 18).and_hms(0, 0, 0),
                NaiveDate::from_ymd(2022, 9, 19).and_hms(0, 0, 0),
            ),
            (
                NaiveDate::from_ymd(2022, 9, 25).and_hms(0, 0, 0),
                NaiveDate::from_ymd(2022, 9, 26).and_hms(0, 0, 0),
            ),
        ]
    )
}

#[test]
fn time_slice_iterator_range_splits_day_into_24_hrs() {
    let r = TimeSliceIterator {
        start: NaiveDate::from_ymd(2022, 1, 1).and_hms(0, 0, 0),
        end: NaiveDate::from_ymd(2022, 1, 2).and_hms(0, 0, 0),
        repetition: Repetition::HOURLY,
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
    let mut counter = 0;
    assert_eq!(
        goal.generate_tasks(
            NaiveDate::from_ymd(2022, 1, 1).and_hms(0, 0, 0),
            NaiveDate::from_ymd(2022, 1, 4).and_hms(0, 0, 0),
            &mut counter
        ),
        vec![Task {
            id: 0,
            goal_id: 1,
            title: "Test".to_string(),
            duration: 1,
            status: TaskStatus::UNSCHEDULED,
            flexibility: 0,
            start: NaiveDate::from_ymd(2022, 1, 1).and_hms(0, 0, 0),
            deadline: NaiveDate::from_ymd(2022, 1, 4).and_hms(0, 0, 0),
            after_time: 0,
            before_time: 24,
            slots: Vec::new(),
            confirmed_start: None,
            confirmed_deadline: None,
        },]
    )
}

#[test]
fn output_formatter_works() {
    let desired_output = r#"[{"taskid":30,"goalid":3,"title":"exercise","duration":1,"start":"2022-01-01T13:00:00","deadline":"2022-01-01T14:00:00"},{"taskid":10,"goalid":1,"title":"shopping","duration":1,"start":"2022-01-01T11:00:00","deadline":"2022-01-01T12:00:00"},{"taskid":20,"goalid":2,"title":"dentist","duration":1,"start":"2022-01-01T10:00:00","deadline":"2022-01-01T11:00:00"}]"#;

    let (calendar_start, calendar_end) = get_calendar_bounds();
    let scheduled_tasks = task_placer(get_test_tasks(), calendar_start, calendar_end);
    let output = output_formatter(scheduled_tasks).unwrap();
    assert_eq!(desired_output, serde_json::to_string(&output).unwrap());
}

#[test]
fn calculate_flexibility_works() {
    let mut tasks = get_test_tasks();
    tasks[0].slots = vec![(
        NaiveDate::from_ymd(2022, 1, 1).and_hms(10, 0, 0),
        NaiveDate::from_ymd(2022, 1, 1).and_hms(11, 0, 0),
    )];
    tasks[1].slots = vec![
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
    ];
    tasks[2].slots = vec![
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
    ];
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
        NaiveDate::from_ymd(2022, 1, 1).and_hms(13, 0, 0)
    );
    assert_eq!(
        scheduled_tasks[0].confirmed_deadline.unwrap(),
        NaiveDate::from_ymd(2022, 1, 1).and_hms(14, 0, 0)
    );
    assert_eq!(
        scheduled_tasks[1].confirmed_start.unwrap(),
        NaiveDate::from_ymd(2022, 1, 1).and_hms(11, 0, 0)
    );
    assert_eq!(
        scheduled_tasks[1].confirmed_deadline.unwrap(),
        NaiveDate::from_ymd(2022, 1, 1).and_hms(12, 0, 0)
    );
    assert_eq!(
        scheduled_tasks[2].confirmed_start.unwrap(),
        NaiveDate::from_ymd(2022, 1, 1).and_hms(10, 0, 0)
    );
    assert_eq!(
        scheduled_tasks[2].confirmed_deadline.unwrap(),
        NaiveDate::from_ymd(2022, 1, 1).and_hms(11, 0, 0)
    );
}
