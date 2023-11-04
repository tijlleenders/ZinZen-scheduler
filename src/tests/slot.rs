// test case to fix if (other.start <= self.start) && (other.end >= self.end)
// Code snippet: else if (other.start <= self.start) && (other.end >= self.end) {

use chrono::Duration;

use crate::models::slot::Slot;

/// Test if subtracting few hours from full day (or more duration)
/// - Expexted to return empty list
#[test]
fn test_subtract_few_hours_from_fullday() {
    let year = 2022;
    let month = 1;
    let day = 1;

    let slot_few_hours = Slot::mock(Duration::hours(10), year, month, day, 5, 0);
    let slot_full_day = Slot::mock(Duration::days(1), year, month, day, 0, 0);

    let expected_result: Vec<Slot> = vec![];

    let result = slot_few_hours - slot_full_day;

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

    let slot_full_day = Slot::mock(Duration::days(1), year, month, day, 0, 0);
    let slot_few_hours = Slot::mock(Duration::hours(10), year, month, day, 5, 0);

    let expected_result: Vec<Slot> = vec![
        Slot::mock(Duration::hours(5), year, month, day, 0, 0),
        Slot::mock(Duration::hours(9), year, month, day, 15, 0),
    ];

    let result = slot_full_day - slot_few_hours;

    assert_eq!(expected_result, result);
}

#[test]
fn test_subtract_same_datetime() {
    let year = 2022;
    let month = 1;
    let day = 1;
    let hour: u32 = 0;
    let min: u32 = 0;
    let duration = Duration::hours(10);

    let slot1 = Slot::mock(duration, year, month, day, hour, min);
    let slot2 = Slot::mock(duration, year, month, day, hour, min);

    let expected_result: Vec<Slot> = vec![];
    let result = slot1 - slot2;

    assert_eq!(expected_result, result);
}

#[test]
fn test_subtract_when_no_overlap() {
    let year = 2022;
    let month = 1;
    let day = 1;
    let hour: u32 = 0;
    let min: u32 = 0;
    let duration = Duration::hours(10);

    let slot1 = Slot::mock(duration, year, month, day, hour, min);
    let slot2 = Slot::mock(duration, year, month, day + 1, hour, min);

    let expected_result: Vec<Slot> = vec![slot1];

    let result = slot1 - slot2;

    assert_eq!(expected_result, result);
}

#[test]
fn test_is_conflicts_with() {
    let year = 2023;
    let month = 5;
    let day = 5;
    let hour: u32 = 0;
    let min: u32 = 0;
    let duration = Duration::hours(10);

    let base_slot = Slot::mock(duration, year, month, day, hour, min);
    let conflicted_last_of_base = Slot::mock(duration, year, month, day, 9, min);
    let conflicted_start_of_base = Slot::mock(duration, year, month, day - 1, 20, min);
    let not_conflicted_with_base = Slot::mock(duration, year, month, day + 1, hour, min);

    let is_conflicted_start_of_base = base_slot.conflicts_with(&conflicted_start_of_base);
    let is_conflicted_last_of_base = base_slot.conflicts_with(&conflicted_last_of_base);
    let is_not_conflicted_with_base = base_slot.conflicts_with(&not_conflicted_with_base);

    // assert_eq!(expected_result, result);
    assert!(is_conflicted_last_of_base);
    assert!(is_conflicted_start_of_base);
    assert!(!is_not_conflicted_with_base);
}

#[test]
fn test_is_contains_slot() {
    let year = 2023;
    let month = 5;
    let day = 5;
    let hour: u32 = 0;
    let min: u32 = 0;
    let duration = Duration::hours(10);

    let base_slot = Slot::mock(duration, year, month, day, hour, min);
    let contained_in_base = Slot::mock(Duration::hours(3), year, month, day, 2, min);
    let equal_to_base = Slot::mock(duration, year, month, day, hour, min);
    let overflow_base_from_start = Slot::mock(Duration::hours(3), year, month, day - 1, 23, min);
    let overflow_base_from_end = Slot::mock(Duration::hours(3), year, month, day, 9, min);
    let not_contained_in_base = Slot::mock(duration, year, month, day + 1, hour, min);

    let is_contained_in_base = base_slot.contains_slot(&contained_in_base);

    let is_equal_to_base_contained = base_slot.contains_slot(&equal_to_base);

    let is_overflow_base_from_start = base_slot.contains_slot(&overflow_base_from_start);

    let is_overflow_base_from_end = base_slot.contains_slot(&overflow_base_from_end);

    let is_not_contained_in_base = base_slot.contains_slot(&not_contained_in_base);

    assert!(is_contained_in_base);
    assert!(is_equal_to_base_contained);
    assert!(!is_overflow_base_from_start);
    assert!(!is_overflow_base_from_end);
    assert!(!is_not_contained_in_base);
}

#[test]
fn test_hours_intersecting_with_slot() {
    let year = 2023;
    let month = 5;
    let day = 5;
    let hour: u32 = 0;
    let min: u32 = 0;
    let duration = Duration::hours(10);

    let base_slot = Slot::mock(duration, year, month, day, hour, min);
    let intersected_last_of_base = Slot::mock(duration, year, month, day, 9, min);
    let intersected_start_of_base = Slot::mock(duration, year, month, day - 1, 20, min);
    let not_intersected_with_base = Slot::mock(duration, year, month, day + 1, hour, min);

    let is_intersected_start_of_base = base_slot.intersection(&intersected_start_of_base);

    let is_intersected_last_of_base = base_slot.intersection(&intersected_last_of_base);

    let is_not_intersected_with_base = base_slot.intersection(&not_intersected_with_base);

    assert_eq!(is_intersected_last_of_base, 1);
    assert_eq!(is_intersected_start_of_base, 6);
    assert_eq!(is_not_intersected_with_base, 0);
}
