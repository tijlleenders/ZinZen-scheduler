use std::vec;

use crate::{
    models::{
        goal::{Day, TimeFilter},
        slot::Slot,
        slot_iterator::TimeSlotsIterator,
    },
    services::filter::apply_filter,
    tests::utils::{get_slot, get_timeline_single_slot},
};
use chrono::{Duration, NaiveDate};

// TODO 2023-05-01 | temp disabled the test because the main reason is to compare
//old apply_filter function with new one. But it is not gonna succeed since the
//new filter function have merging consequent slots applied.
// #[test]
fn _test_filter_for_workday() {
    let init_year = 2022;
    let init_month = 10;
    let init_day = 1;
    let init_duration = Duration::days(31);
    let start_time: u32 = 5;
    let end_time: u32 = 15;

    let before_time: Option<usize> = Some(end_time as usize);
    let after_time: Option<usize> = Some(start_time as usize);
    let on_days: Option<Vec<Day>> = Some(vec![Day::Sun, Day::Mon, Day::Tue, Day::Wed, Day::Thu]);
    let not_on: Option<Vec<Slot>> = Some(vec![
        get_slot(Duration::hours(10), init_year, init_month, 2, start_time, 0),
        get_slot(Duration::hours(10), init_year, init_month, 6, start_time, 0),
        get_slot(
            Duration::hours(10),
            init_year,
            init_month,
            11,
            start_time,
            0,
        ),
    ]);

    let start = NaiveDate::from_ymd_opt(init_year, init_month, init_day)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    let deadline = NaiveDate::from_ymd_opt(init_year, init_month + 1, init_day)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();

    let filters = Some(TimeFilter {
        before_time,
        after_time,
        on_days,
        not_on,
    });
    dbg!(&start, &deadline);

    let timeline = get_timeline_single_slot(init_duration, init_year, init_month, init_day);
    dbg!(&timeline);

    let mut slot_iterator = TimeSlotsIterator {
        timeline: timeline.clone(),
        repetition: None,
        filters: filters.clone(),
        current_start_position: start,
    };

    // Test on the old filter function
    slot_iterator._apply_filters();
    let timeline_by_old_filter = slot_iterator.timeline;

    // Test on the new filter function
    let timeline_by_new_filter = apply_filter(&timeline, &filters);

    dbg!(&timeline_by_old_filter);
    dbg!(&timeline_by_new_filter);

    // assert!(false);
    assert_eq!(timeline_by_new_filter, timeline_by_old_filter);
}

#[test]
fn test_i284_7days() {
    // Simulating issue-284-filter-days-of-week-7days
    let init_year = 2023;
    let init_month = 03;
    let init_day = 9;
    let end_day: u32 = 20;

    let init_duration = Duration::days((end_day - init_day) as i64);
    let start_time: u32 = 8;
    let end_time: u32 = 12;

    let after_time: Option<usize> = Some(start_time as usize);
    let before_time: Option<usize> = Some(end_time as usize);
    let on_days: Option<Vec<Day>> = Some(vec![
        Day::Thu,
        Day::Fri,
        Day::Sat,
        Day::Sun,
        Day::Mon,
        Day::Tue,
        Day::Wed,
    ]);
    let not_on: Option<Vec<Slot>> = None;

    let start = NaiveDate::from_ymd_opt(init_year, init_month, init_day)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    let deadline = NaiveDate::from_ymd_opt(init_year, init_month, end_day)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();

    let filters = Some(TimeFilter {
        before_time,
        after_time,
        on_days,
        not_on,
    });
    dbg!(&start, &deadline, &filters);

    let timeline = get_timeline_single_slot(init_duration, init_year, init_month, init_day);
    dbg!(&timeline);

    let mut slot_iterator = TimeSlotsIterator {
        timeline: timeline.clone(),
        repetition: None,
        filters: filters.clone(),
        current_start_position: start,
    };

    // Test on the old filter function
    slot_iterator._apply_filters();
    let timeline_by_old_filter = slot_iterator.timeline;

    // Test on the new filter function
    let timeline_by_new_filter = apply_filter(&timeline, &filters);

    dbg!(&timeline_by_old_filter);
    dbg!(&timeline_by_new_filter);

    // assert!(false);
    assert_eq!(timeline_by_new_filter, timeline_by_old_filter);
}

#[test]
fn test_i276() {
    // Simulating issue-276-weekdays-filter-on-budget
    let init_year = 2023;
    let init_month: u32 = 03;
    let init_day: u32 = 9;
    let end_day: u32 = 20;

    let init_duration = Duration::days((end_day - init_day) as i64);
    let start_time: u32 = 8;
    let end_time: u32 = 16;

    let after_time: Option<usize> = Some(start_time as usize);
    let before_time: Option<usize> = Some(end_time as usize);
    let on_days: Option<Vec<Day>> = Some(vec![Day::Mon, Day::Tue, Day::Wed, Day::Thu, Day::Fri]);
    let not_on: Option<Vec<Slot>> = None;

    let start = NaiveDate::from_ymd_opt(init_year, init_month, init_day)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    let deadline = NaiveDate::from_ymd_opt(init_year, init_month, end_day)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();

    let filters = Some(TimeFilter {
        before_time,
        after_time,
        on_days,
        not_on,
    });
    dbg!(&start, &deadline, &filters);

    let timeline = get_timeline_single_slot(init_duration, init_year, init_month, init_day);
    dbg!(&timeline);

    let mut slot_iterator = TimeSlotsIterator {
        timeline: timeline.clone(),
        repetition: None,
        filters: filters.clone(),
        current_start_position: start,
    };

    // Test on the old filter function
    slot_iterator._apply_filters();
    let timeline_by_old_filter = slot_iterator.timeline;

    // Test on the new filter function
    let timeline_by_new_filter = apply_filter(&timeline, &filters);

    dbg!(&timeline_by_old_filter);
    dbg!(&timeline_by_new_filter);

    // assert!(false);
    assert_eq!(timeline_by_new_filter, timeline_by_old_filter);
}

// TODO 2023-05-01 | temp disabled the test because the main reason is to compare
//old apply_filter function with new one. But it is not gonna succeed since the
//new filter function have merging consequent slots applied.
// #[test]
fn _test_i293_postpone_2() {
    // Simulating i293_postpone_2
    // Here issue related to divide slots into hours inside function `filter_not_on`
    //which not wasn't implemented in the new filter functionlity.
    let init_year = 2023;
    let init_month: u32 = 04;
    let init_day: u32 = 1;
    let end_day: u32 = 7;

    let init_duration = Duration::days((end_day - init_day) as i64);
    let start_time: u32 = 0;
    let _end_time: u32 = 0;

    let after_time: Option<usize> = Some(start_time as usize);
    let before_time: Option<usize> = None;
    let on_days: Option<Vec<Day>> = None;
    let not_on: Option<Vec<Slot>> = Some(vec![
        get_slot(
            Duration::hours(3),
            init_year,
            init_month,
            init_day,
            start_time,
            0,
        ),
        get_slot(Duration::hours(1), init_year, init_month, init_day, 5, 0),
    ]);

    let start = NaiveDate::from_ymd_opt(init_year, init_month, init_day)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    let deadline = NaiveDate::from_ymd_opt(init_year, init_month, end_day)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();

    let filters = Some(TimeFilter {
        before_time,
        after_time,
        on_days,
        not_on,
    });
    dbg!(&start, &deadline, &filters);

    let timeline = get_timeline_single_slot(init_duration, init_year, init_month, init_day);
    dbg!(&timeline);

    let mut slot_iterator = TimeSlotsIterator {
        timeline: timeline.clone(),
        repetition: None,
        filters: filters.clone(),
        current_start_position: start,
    };

    // Test on the old filter function
    slot_iterator._apply_filters();
    let timeline_by_old_filter = slot_iterator.timeline;

    // Test on the new filter function
    let timeline_by_new_filter = apply_filter(&timeline, &filters);

    dbg!(&timeline_by_old_filter);
    dbg!(&timeline_by_new_filter);

    // assert!(false);
    assert_eq!(timeline_by_new_filter, timeline_by_old_filter);
}
