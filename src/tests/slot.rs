// test case to fix if (other.start <= self.start) && (other.end >= self.end)
// Code snippet: else if (other.start <= self.start) && (other.end >= self.end) {

use chrono::Duration;

use crate::{models::slot::Slot, tests::utils::get_slot};

#[test]
fn test_subtract_few_hours_from_fullday() {
    /*
    slot_few_hours = Slot {
        start: 2022-01-02T05:00:00,
        end: 2022-01-02T15:00:00,
    }
    slot_full_day = Slot {
        start: 2022-01-02T00:00:00,
        end: 2022-01-03T00:00:00,
    }
    */

    let year = 2022;
    let month = 1;
    let day = 1;

    let slot_few_hours = get_slot(Duration::hours(10), year, month, day, 5, 0);
    let slot_full_day = get_slot(Duration::days(1), year, month, day, 0, 0);

    let expected_result: Vec<Slot> = vec![
        get_slot(Duration::hours(5), year, month, day, 0, 0),
        get_slot(Duration::hours(9), year, month, day, 15, 0),
    ];
    dbg!(&expected_result);

    let result = slot_few_hours - slot_full_day;
    dbg!(&result);

    assert_eq!(expected_result, result);
}

#[test]
fn test_subtract_fullday_from_few_hours() {
    /*
    slot_full_day = Slot {
        start: 2022-01-02T00:00:00,
        end: 2022-01-03T00:00:00,
    }
    slot_few_hours = Slot {
        start: 2022-01-02T05:00:00,
        end: 2022-01-02T15:00:00,
    }
    */

    let year = 2022;
    let month = 1;
    let day = 1;

    let slot_full_day = get_slot(Duration::days(1), year, month, day, 0, 0);
    let slot_few_hours = get_slot(Duration::hours(10), year, month, day, 5, 0);

    let expected_result: Vec<Slot> = vec![
        get_slot(Duration::hours(5), year, month, day, 0, 0),
        get_slot(Duration::hours(9), year, month, day, 15, 0),
    ];
    dbg!(&expected_result);

    let result = slot_full_day - slot_few_hours;
    dbg!(&result);

    assert_eq!(expected_result, result);
}
