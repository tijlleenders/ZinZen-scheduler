use chrono::NaiveDate;

use crate::models::{
    repetition::Repetition,
    slots_iterator::utils::{get_start_of_repeat_step, next_week},
};

mod mocking {
    use chrono::{NaiveDate, NaiveDateTime};

    pub struct DateTime {
        pub datetime: NaiveDateTime,
    }
    impl DateTime {
        /// Get a NaiveDateTime based on a year, month and day with 0 for hms
        pub fn get_by_date(year: i32, month: u32, day: u32) -> NaiveDateTime {
            NaiveDate::from_ymd_opt(year, month, day)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
        }
    }
}

#[test]
fn test_next_week_on_monday() {
    let mut input = mocking::DateTime::get_by_date(2023, 5, 1);
    let expected = mocking::DateTime::get_by_date(2023, 5, 8);

    let output = next_week(&mut input);

    assert_eq!(output, expected);
}

#[test]
fn test_next_week_on_tuesday() {
    let mut input = mocking::DateTime::get_by_date(2023, 5, 2);
    let expected = mocking::DateTime::get_by_date(2023, 5, 9);

    let output = next_week(&mut input);

    assert_eq!(output, expected);
}

#[test]
fn test_next_week_on_wednesday() {
    let mut input = mocking::DateTime::get_by_date(2023, 5, 3);
    let expected = mocking::DateTime::get_by_date(2023, 5, 10);

    let output = next_week(&mut input);

    assert_eq!(output, expected);
}

#[test]
fn test_next_week_on_thursday() {
    let mut input = mocking::DateTime::get_by_date(2023, 5, 4);
    let expected = mocking::DateTime::get_by_date(2023, 5, 11);

    let output = next_week(&mut input);

    assert_eq!(output, expected);
}

#[test]
fn test_next_week_on_friday() {
    let mut input = mocking::DateTime::get_by_date(2023, 5, 5);
    let expected = mocking::DateTime::get_by_date(2023, 5, 12);

    let output = next_week(&mut input);

    assert_eq!(output, expected);
}

#[test]
fn get_next_weekend() {
    let repetition = Repetition::WEEKENDS;

    let monday = NaiveDate::from_ymd_opt(2022, 09, 26)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    let saturday = NaiveDate::from_ymd_opt(2022, 10, 1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    let monday_with_time = NaiveDate::from_ymd_opt(2022, 09, 26)
        .unwrap()
        .and_hms_opt(1, 33, 7)
        .unwrap();
    let saturday_with_time = NaiveDate::from_ymd_opt(2022, 10, 1)
        .unwrap()
        .and_hms_opt(1, 33, 7)
        .unwrap();
    let _next_weekend_from_monday = NaiveDate::from_ymd_opt(2022, 10, 8)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    let _next_weekend_from_weekend = NaiveDate::from_ymd_opt(2022, 10, 15)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    let next_weekend_from_monday = get_start_of_repeat_step(&monday, repetition);
    let next_weekend_from_saturday = get_start_of_repeat_step(&saturday, repetition);
    let next_weekend_from_monday_with_time =
        get_start_of_repeat_step(&monday_with_time, repetition);
    let next_weekend_from_saturday_with_time =
        get_start_of_repeat_step(&saturday_with_time, repetition);
    assert_eq!(next_weekend_from_monday, next_weekend_from_monday);
    assert_eq!(next_weekend_from_saturday, next_weekend_from_saturday);
    assert_eq!(next_weekend_from_monday_with_time, next_weekend_from_monday);
    assert_eq!(
        next_weekend_from_saturday_with_time,
        next_weekend_from_saturday
    );
}
