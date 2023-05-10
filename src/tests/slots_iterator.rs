use crate::models::slots_iterator::utils::next_week;

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
