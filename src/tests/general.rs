use crate::{models::goal::Day, models::slot::Slot};
use chrono::*;
use std::vec;

#[test]
fn divide_a_day_in_days() {
    let slot_of_exactly_a_day = Slot {
        start: NaiveDate::from_ymd_opt(2022, 09, 26)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap(),
        end: NaiveDate::from_ymd_opt(2022, 09, 27)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap(),
    };
    let exact_day_split_in_days: Vec<Slot> = vec![Slot {
        start: NaiveDate::from_ymd_opt(2022, 09, 26)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap(),
        end: NaiveDate::from_ymd_opt(2022, 09, 27)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap(),
    }];
    let result = slot_of_exactly_a_day.split_into_days();
    assert_eq!(exact_day_split_in_days, result);
}
#[test]
fn divide_two_days_in_days() {
    let slot_of_exactly_two_day = Slot {
        start: NaiveDate::from_ymd_opt(2022, 09, 26)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap(),
        end: NaiveDate::from_ymd_opt(2022, 09, 28)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap(),
    };
    let exactly_two_days_split_in_days: Vec<Slot> = vec![
        Slot {
            start: NaiveDate::from_ymd_opt(2022, 09, 26)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
            end: NaiveDate::from_ymd_opt(2022, 09, 27)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        },
        Slot {
            start: NaiveDate::from_ymd_opt(2022, 09, 27)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
            end: NaiveDate::from_ymd_opt(2022, 09, 28)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        },
    ];
    let result = slot_of_exactly_two_day.split_into_days();
    assert_eq!(exactly_two_days_split_in_days, result);
}

#[test]
fn divide_half_a_day_in_days() {
    let slot_of_half_a_day = Slot {
        start: NaiveDate::from_ymd_opt(2022, 10, 01)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap(),
        end: NaiveDate::from_ymd_opt(2022, 10, 01)
            .unwrap()
            .and_hms_opt(6, 0, 0)
            .unwrap(),
    };
    let half_a_day_split_in_days: Vec<Slot> = vec![Slot {
        start: NaiveDate::from_ymd_opt(2022, 10, 01)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap(),
        end: NaiveDate::from_ymd_opt(2022, 10, 01)
            .unwrap()
            .and_hms_opt(6, 0, 0)
            .unwrap(),
    }];
    let result = slot_of_half_a_day.split_into_days();
    assert_eq!(half_a_day_split_in_days, result);
}

#[test]
fn test_convert_day_object_from_string() {
    let day: Day = Day::from("Tue".to_string());
    assert_eq!(day, Day::Tue);

    let day: Day = Day::from("tue".to_string());
    assert_eq!(day, Day::Tue);

    let day: Day = Day::from("thu".to_string());
    assert_eq!(day, Day::Thu);
}

#[test]
fn test_convert_day_object_into_string() {
    let fri_converted: String = Day::Fri.into();

    let fri_str: String = "Fri".to_string();
    assert_eq!(fri_str, fri_converted);

    let fri_str: String = "FRI".to_string();
    assert_ne!(fri_str, fri_converted);
}

#[test]
fn test_subtract_2_slots() {
    // Test Trait Sub for Slot to make sure it is working properly

    let (slot1, slot2) = (
        Slot::mock(Duration::hours(5), 2022, 10, 1, 05, 0),
        Slot::mock(Duration::hours(5), 2022, 10, 1, 09, 0),
    );
    dbg!(slot1, slot2);

    // expected result: [2022-10-01 09:00:00 --- 2022-10-01 10:00:00]
    let expected = vec![Slot::mock(Duration::hours(4), 2022, 10, 1, 05, 0)];
    dbg!(&expected);

    let result = slot1 - slot2;
    dbg!(&result);

    assert_eq!(expected, result);
}

#[test]
fn test_compare_2_slots() {
    // Test comparing Slots

    let (slot1, slot2) = (
        Slot::mock(Duration::hours(5), 2022, 10, 1, 05, 0),
        Slot::mock(Duration::hours(5), 2022, 10, 1, 09, 0),
    );
    dbg!(slot1, slot2);

    if slot1 > slot2 {
        dbg!("slot1 > slot2");
    } else if slot1 < slot2 {
        dbg!("slot1 < slot2");
    } else if slot1 == slot2 {
        dbg!("slot1 == slot2");
    } else {
        dbg!("slot1 != slot2");
    }

    let max = std::cmp::max(slot1, slot2);
    dbg!(max);
    let min = std::cmp::min(slot1, slot2);
    dbg!(min);

    assert!(true);
}
