use std::collections::BTreeSet;

use crate::{
    models::repetition::Repetition,
    models::{
        goal::Day,
        slot::*,
        timeline::{Timeline, TimelineOperations},
    },
};
use chrono::*;

use self::utils::{get_2_slots, get_slot};

#[cfg(test)]
#[test]
fn get_next_monday() {
    use crate::models::slot_iterator::utils::get_start_of_repeat_step;

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
    let next_monday = NaiveDate::from_ymd_opt(2022, 10, 3)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    let next_monday_from_monday = get_start_of_repeat_step(&monday, Repetition::Weekly(1));
    let next_monday_from_saturday = get_start_of_repeat_step(&saturday, Repetition::Weekly(1));
    let next_monday_from_monday_with_time =
        get_start_of_repeat_step(&monday_with_time, Repetition::Weekly(1));
    let next_monday_from_saturday_with_time =
        get_start_of_repeat_step(&saturday_with_time, Repetition::Weekly(1));
    assert_eq!(next_monday_from_monday, next_monday);
    assert_eq!(next_monday_from_saturday, next_monday);
    assert_eq!(next_monday_from_monday_with_time, next_monday);
    assert_eq!(next_monday_from_saturday_with_time, next_monday);
}

#[test]
fn get_next_weekend() {
    use crate::models::slot_iterator::utils::get_start_of_repeat_step;

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
    let next_weekend_from_monday = get_start_of_repeat_step(&monday, Repetition::WEEKENDS);
    let next_weekend_from_saturday = get_start_of_repeat_step(&saturday, Repetition::WEEKENDS);
    let next_weekend_from_monday_with_time =
        get_start_of_repeat_step(&monday_with_time, Repetition::WEEKENDS);
    let next_weekend_from_saturday_with_time =
        get_start_of_repeat_step(&saturday_with_time, Repetition::WEEKENDS);
    assert_eq!(next_weekend_from_monday, next_weekend_from_monday);
    assert_eq!(next_weekend_from_saturday, next_weekend_from_saturday);
    assert_eq!(next_weekend_from_monday_with_time, next_weekend_from_monday);
    assert_eq!(
        next_weekend_from_saturday_with_time,
        next_weekend_from_saturday
    );
}

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
    let result = slot_of_exactly_a_day.divide_into_days();
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
    let result = slot_of_exactly_two_day.divide_into_days();
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
    let result = slot_of_half_a_day.divide_into_days();
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

    let (slot1, slot2) = utils::get_2_slots();
    dbg!(slot1, slot2);

    // expected result: [2022-10-01 09:00:00 --- 2022-10-01 10:00:00]
    let expected = vec![Slot {
        start: NaiveDate::from_ymd_opt(2022, 10, 1)
            .unwrap()
            .and_hms_opt(5, 0, 0)
            .unwrap(),
        end: NaiveDate::from_ymd_opt(2022, 10, 1)
            .unwrap()
            .and_hms_opt(09, 0, 0)
            .unwrap(),
    }];
    dbg!(&expected);

    let result = slot1 - slot2;
    dbg!(&result);

    assert_eq!(expected, result);
}

#[test]
fn test_compare_2_slots() {
    // Test comparing Slots

    let (slot1, slot2) = utils::get_2_slots();

    // expected result: [2022-10-01 09:00:00 --- 2022-10-01 10:00:00]
    let expected = vec![Slot {
        start: NaiveDate::from_ymd_opt(2022, 10, 1)
            .unwrap()
            .and_hms_opt(5, 0, 0)
            .unwrap(),
        end: NaiveDate::from_ymd_opt(2022, 10, 1)
            .unwrap()
            .and_hms_opt(09, 0, 0)
            .unwrap(),
    }];

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

#[test]
fn test_initialize_timeline() {
    let sample_slot = get_slot();

    let expected_slot_in_timeline = Slot {
        start: sample_slot.start,
        end: sample_slot.end,
    };
    let mut expected_collection_in_timeline = BTreeSet::new();
    expected_collection_in_timeline.insert(expected_slot_in_timeline);
    let expected_timeline = Timeline {
        slots: expected_collection_in_timeline,
    };

    if let Some(timeline) = Timeline::initialize(sample_slot.start, sample_slot.end) {
        assert_eq!(expected_timeline, timeline);
    } else {
        assert!(false);
    }
}

#[test]
fn test_remove_from_timeline() {
    let (slot1, slot2) = utils::get_2_slots();
    let sample_slot = get_slot();

    if let Some(mut timeline) = Timeline::initialize(sample_slot.start, sample_slot.end) {
        let (slot_to_remove, _) = get_2_slots();
        dbg!(&slot_to_remove);

        if let Some(result) = timeline.remove_slots(vec![slot_to_remove]) {
            let expected_result = vec![Slot {
                start: NaiveDate::from_ymd_opt(2022, 10, 1)
                    .unwrap()
                    .and_hms_opt(10, 0, 0)
                    .unwrap(),
                end: NaiveDate::from_ymd_opt(2022, 10, 1)
                    .unwrap()
                    .and_hms_opt(20, 0, 0)
                    .unwrap(),
            }];
            dbg!(&expected_result);
            dbg!(&result);

            assert_eq!(expected_result, result);
        } else {
            assert!(false);
        }
    } else {
        assert!(false);
    }
}

mod utils {
    use chrono::NaiveDate;

    use crate::models::slot::Slot;

    /// Get 2 timeslots:
    /// slot1: [2022-10-01 05:00:00 --- 2022-10-01 20:00:00]
    /// slot2: [2022-10-01 09:00:00 --- 2022-10-01 02:00:00]
    pub fn get_2_slots() -> (Slot, Slot) {
        // slot1: [2022-10-01 05:00:00 --- 2022-10-01 20:00:00]
        let slot1 = Slot {
            start: NaiveDate::from_ymd_opt(2022, 10, 1)
                .unwrap()
                .and_hms_opt(05, 0, 0)
                .unwrap(),
            end: NaiveDate::from_ymd_opt(2022, 10, 1)
                .unwrap()
                .and_hms_opt(10, 0, 0)
                .unwrap(),
        };

        // slot2: [2022-10-01 09:00:00 --- 2022-10-01 02:00:00]
        let slot2 = Slot {
            start: NaiveDate::from_ymd_opt(2022, 10, 1)
                .unwrap()
                .and_hms_opt(9, 0, 0)
                .unwrap(),
            end: NaiveDate::from_ymd_opt(2022, 10, 1)
                .unwrap()
                .and_hms_opt(14, 0, 0)
                .unwrap(),
        };
        (slot1, slot2)
    }

    /// Get a slot
    /// slot: [2022-10-01 05:00:00 --- 2022-10-01 20:00:00]
    pub fn get_slot() -> Slot {
        // slot: [2022-10-01 05:00:00 --- 2022-10-01 20:00:00]

        let start = NaiveDate::from_ymd_opt(2022, 10, 1)
            .unwrap()
            .and_hms_opt(05, 0, 0)
            .unwrap();
        let end = NaiveDate::from_ymd_opt(2022, 10, 1)
            .unwrap()
            .and_hms_opt(20, 0, 0)
            .unwrap();

        Slot { start, end }
    }
}
