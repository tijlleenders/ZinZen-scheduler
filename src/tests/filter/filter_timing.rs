use crate::{
    models::{slot::Slot, timeline::Timeline},
    services::filter::filter_timing,
    tests::utils::get_timeline,
};
use chrono::{Duration, NaiveDate};

#[test]
fn test_aftertime_only() {
    let before_time = None;
    let after_time = Some(5);

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

    let filtered_timeline = filter_timing(&timeline, before_time, after_time);
    dbg!(&filtered_timeline);

    assert_eq!(filtered_timeline, expected_result);
}

#[test]
fn test_beforetime_only() {
    let before_time = Some(20);
    let after_time = None;

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
                .and_hms_opt(0, 0, 0)
                .unwrap(),
            end: NaiveDate::from_ymd_opt(init_year, init_month, init_day)
                .unwrap()
                .and_hms_opt(20, 0, 0)
                .unwrap(),
        }]
        .into_iter()
        .collect(),
    };
    dbg!(&expected_result);

    let filtered_timeline = filter_timing(&timeline, before_time, after_time);
    dbg!(&filtered_timeline);

    assert_eq!(filtered_timeline, expected_result);
}

#[test]
fn test_both_beforetime_and_aftertime() {
    let before_time = Some(20);
    let after_time = Some(5);

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
            end: NaiveDate::from_ymd_opt(init_year, init_month, init_day)
                .unwrap()
                .and_hms_opt(20, 0, 0)
                .unwrap(),
        }]
        .into_iter()
        .collect(),
    };
    dbg!(&expected_result);

    let filtered_timeline = filter_timing(&timeline, before_time, after_time);
    dbg!(&filtered_timeline);

    assert_eq!(filtered_timeline, expected_result);
}

#[test]
fn test_beforetime_is_before_aftertime() {
    // Testing when before_time before after_time
    let before_time = Some(5);
    let after_time = Some(20);

    let init_year = 2022;
    let init_month = 1;
    let init_day = 1;
    let init_duration = Duration::days(1);
    let timeline = get_timeline(init_duration, init_year, init_month, init_day);
    dbg!(&timeline);

    let expected_result = Timeline {
        slots: vec![
            Slot {
                start: NaiveDate::from_ymd_opt(init_year, init_month, init_day)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
                end: NaiveDate::from_ymd_opt(init_year, init_month, init_day)
                    .unwrap()
                    .and_hms_opt(05, 0, 0)
                    .unwrap(),
            },
            Slot {
                start: NaiveDate::from_ymd_opt(init_year, init_month, init_day)
                    .unwrap()
                    .and_hms_opt(20, 0, 0)
                    .unwrap(),
                end: NaiveDate::from_ymd_opt(init_year, init_month, init_day + 1)
                    .unwrap()
                    .and_hms_opt(00, 0, 0)
                    .unwrap(),
            },
        ]
        .into_iter()
        .collect(),
    };
    dbg!(&expected_result);

    let filtered_timeline = filter_timing(&timeline, before_time, after_time);
    dbg!(&filtered_timeline);

    assert_eq!(filtered_timeline, expected_result);
}
