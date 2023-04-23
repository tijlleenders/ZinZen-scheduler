use crate::{
    models::{
        slot::Slot,
        timeline::{Timeline, TimelineOperations},
    },
    tests::utils::*,
};
use chrono::NaiveDate;
use std::collections::BTreeSet;

#[test]
fn test_initialize() {
    let sample_slot = get_slot_1();

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
fn test_remove_from() {
    // sample_slot: [2022-10-01 05:00:00 --- 2022-10-01 20:00:00]
    let sample_slot = get_slot_1();

    if let Some(mut timeline) = Timeline::initialize(sample_slot.start, sample_slot.end) {
        // slot_to_remove: [2022-10-01 05:00:00 --- 2022-10-01 10:00:00]
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

#[test]
fn test_insert_into() {
    let sample_slot = get_slot_1();

    if let Some(mut timeline) = Timeline::initialize(sample_slot.start, sample_slot.end) {
        let slot_to_insert = get_slot_2();
        dbg!(&slot_to_insert);

        match timeline.insert_slots(vec![slot_to_insert]) {
            Some(_) => {
                let expected_result = Timeline {
                    slots: vec![sample_slot, slot_to_insert].into_iter().collect(),
                };
                dbg!(&expected_result);

                dbg!(&timeline);

                assert_eq!(expected_result, timeline);
            }
            None => assert!(false),
        }
    } else {
        assert!(false);
    }
}

#[test]
fn test_get_next() {
    let ((slot1, slot2), slot3, slot4) = (get_2_slots(), get_slot_1(), get_slot_2());

    let timeline = Timeline {
        slots: vec![slot1, slot2, slot3, slot4].into_iter().collect(),
    };
    dbg!(&timeline);

    if let Some(next_slot) = timeline.get_slot(1) {
        dbg!(&next_slot);
        assert_eq!(slot2, next_slot);
    } else {
        assert!(false);
    }
}

#[test]
fn test_remove_halfday_from_fullday() {
    let mut timeline = get_timeline_fullday();

    let halfday_slot = get_slot_halfday_2nd();
    let expected_result = vec![get_slot_halfday_1st()];

    dbg!(&timeline);
    dbg!(&halfday_slot);
    dbg!(&expected_result);

    if let Some(result) = timeline.remove_slots(vec![halfday_slot]) {
        dbg!(&result);

        assert_eq!(expected_result, result);
    } else {
        assert!(false);
    }
}

#[test]
fn test_remove_afternoon_hours_from_fullday() {
    let mut timeline = get_timeline_fullday();

    let slot_afternoon = get_slot_afternoon_hours();

    let expected_result = get_slots_aday_without_afternoon_hours();

    dbg!(&timeline);
    dbg!(&slot_afternoon);
    dbg!(&expected_result);

    if let Some(result) = timeline.remove_slots(vec![slot_afternoon]) {
        dbg!(&result);

        assert_eq!(expected_result, result);
    } else {
        assert!(false);
    }
}
