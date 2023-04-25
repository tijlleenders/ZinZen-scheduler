use chrono::{Duration, NaiveDate};

use crate::{
    models::{
        goal::{Day, TimeFilter},
        slot::Slot,
        timeline::Timeline,
    },
    services::filter::{apply_filter, filter_not_on, filter_on_days, filter_timing},
    tests::utils::{get_slot, get_timeline, get_timeline_days},
};

#[test]
fn test_filter_after_5am() {
    let init_year = 2022;
    let init_month = 1;
    let init_day = 1;
    let init_duration = Duration::days(1);

    // intiate a sample timeline
    let timeline = get_timeline(init_duration, init_year, init_month, init_day);
    dbg!(&timeline);

    let expected_result = Timeline {
        slots: vec![Slot {
            start: NaiveDate::from_ymd_opt(init_year, init_month, init_day)
                .unwrap()
                .and_hms_opt(5, 0, 0)
                .unwrap(),
            end: NaiveDate::from_ymd_opt(init_year, init_month, init_day + 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        }]
        .into_iter()
        .collect(),
    };
    dbg!(&expected_result);

    let filtered_timeline = apply_filter(
        &timeline,
        &TimeFilter {
            before_time: None,
            after_time: Some(5),
            on_days: None,
            not_on: None,
        },
    );
    dbg!(&filtered_timeline);

    assert_eq!(filtered_timeline, expected_result);
}

#[test]
fn test_fn_filter_timing() {
    let init_year = 2022;
    let init_month = 1;
    let init_day = 1;
    let init_duration = Duration::days(1);
    let timeline = get_timeline(init_duration, init_year, init_month, init_day);
    dbg!(&timeline);

    let expected_result = Timeline {
        slots: vec![Slot {
            start: NaiveDate::from_ymd_opt(init_year, init_month, init_day)
                .unwrap()
                .and_hms_opt(5, 0, 0)
                .unwrap(),
            end: NaiveDate::from_ymd_opt(init_year, init_month, init_day + 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        }]
        .into_iter()
        .collect(),
    };
    dbg!(&expected_result);

    let before_time = None;
    let after_time = Some(5);

    let filtered_timeline = filter_timing(&timeline, before_time, after_time);
    dbg!(&filtered_timeline);

    assert_eq!(filtered_timeline, expected_result);
}

#[test]
fn test_fn_filter_on_days() {
    // 2022-01-01 is a Saturday

    let init_year = 2022;
    let init_month = 1;
    let init_day = 1;
    let days_count: i64 = 15;
    let duration = Duration::days(1);
    let hour: u32 = 0;
    let minute: u32 = 0;

    let timeline = get_timeline_days(days_count, init_year, init_month, init_day);
    dbg!(&timeline);

    let days_to_filter: Vec<Day> = vec![Day::Sun, Day::Tue, Day::Fri];

    let expected_result = Timeline {
        slots: vec![
            get_slot(duration, init_year, init_month, 2, hour, minute),
            get_slot(duration, init_year, init_month, 4, hour, minute),
            get_slot(duration, init_year, init_month, 7, hour, minute),
            get_slot(duration, init_year, init_month, 9, hour, minute),
            get_slot(duration, init_year, init_month, 11, hour, minute),
            get_slot(duration, init_year, init_month, 14, hour, minute),
        ]
        .into_iter()
        .collect(),
    };

    let filtered_timeline = filter_on_days(&timeline, &days_to_filter);
    dbg!(&expected_result);
    dbg!(&filtered_timeline);

    assert_eq!(filtered_timeline, expected_result);
}

#[test]
fn test_fn_filter_on_days_timeline_is_not_splitted() {
    // 2022-01-01 is a Saturday
    // TODO this test is not working
    let init_year = 2022;
    let init_month = 1;
    let init_day = 1;
    let days_count: i64 = 15;
    let duration = Duration::days(days_count);
    let hour: u32 = 0;
    let minute: u32 = 0;

    let timeline = get_timeline(duration, init_year, init_month, init_day);
    dbg!(&timeline);

    let days_to_filter: Vec<Day> = vec![Day::Sun, Day::Tue, Day::Fri];

    let expected_result = Timeline {
        slots: vec![
            get_slot(duration, init_year, init_month, 2, hour, minute),
            get_slot(duration, init_year, init_month, 4, hour, minute),
            get_slot(duration, init_year, init_month, 7, hour, minute),
            get_slot(duration, init_year, init_month, 9, hour, minute),
            get_slot(duration, init_year, init_month, 11, hour, minute),
            get_slot(duration, init_year, init_month, 14, hour, minute),
        ]
        .into_iter()
        .collect(),
    };

    let filtered_timeline = filter_on_days(&timeline, &days_to_filter);
    dbg!(&expected_result);
    dbg!(&filtered_timeline);

    assert_eq!(filtered_timeline, expected_result);
}

#[test]
fn test_fn_filter_not_on() {
    let init_year = 2022;
    let init_month = 1;
    let init_day = 1;
    let days_count: i64 = 5;
    let duration = Duration::days(1);
    let hour: u32 = 0;
    let minute: u32 = 0;

    let timeline = get_timeline_days(days_count, init_year, init_month, init_day);
    dbg!(&timeline);

    let slots_to_filter: Vec<Slot> =
        vec![get_slot(duration, init_year, init_month, 2, hour, minute)];

    let expected_result = Timeline {
        slots: vec![
            get_slot(duration, init_year, init_month, init_day, hour, minute),
            get_slot(duration, init_year, init_month, 3, hour, minute),
            get_slot(duration, init_year, init_month, 4, hour, minute),
            get_slot(duration, init_year, init_month, 5, hour, minute),
        ]
        .into_iter()
        .collect(),
    };
    dbg!(&expected_result);

    let filtered_timeline = filter_not_on(&timeline, &slots_to_filter);
    dbg!(&filtered_timeline);

    assert_eq!(filtered_timeline, expected_result);
}
