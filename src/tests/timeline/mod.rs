pub mod remove_slots;

use crate::{
    models::{slot::Slot, timeline::Timeline},
    tests::utils::*,
};
use chrono::Duration;
use std::collections::BTreeSet;

#[test]
fn test_initialize() {
    let sample_slot = get_slot(Duration::hours(15), 2022, 10, 1, 05, 0);

    let expected_slot_in_timeline = Slot {
        start: sample_slot.start,
        end: sample_slot.end,
    };
    let mut expected_collection_in_timeline = BTreeSet::new();
    expected_collection_in_timeline.insert(expected_slot_in_timeline);
    let expected_timeline = Timeline {
        slots: vec![expected_slot_in_timeline].into_iter().collect(),
    };

    if let Some(timeline) = Timeline::initialize(sample_slot.start, sample_slot.end) {
        assert_eq!(expected_timeline, timeline);
    } else {
        assert!(false);
    }
}

#[test]
fn test_get_next() {
    let (slot1, slot2, slot3, slot4) = (
        get_slot(Duration::hours(2), 2022, 10, 1, 01, 0),
        get_slot(Duration::hours(3), 2022, 10, 1, 03, 0),
        get_slot(Duration::hours(4), 2022, 10, 1, 07, 0),
        get_slot(Duration::hours(10), 2022, 10, 1, 12, 0),
    );
    dbg!(&(slot1, slot2, slot3, slot4));
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
fn test_split_into_days() {
    let init_year = 2022;
    let init_month = 1;
    let init_day = 1;
    let hour: u32 = 0;
    let minute: u32 = 0;
    let days_count: i64 = 5;
    let duration = Duration::days(days_count);

    let timeline = get_timeline_single_slot(duration, init_year, init_month, init_day);

    let expected_result = Timeline {
        slots: vec![
            get_slot(Duration::days(1), init_year, init_month, 1, hour, minute),
            get_slot(Duration::days(1), init_year, init_month, 2, hour, minute),
            get_slot(Duration::days(1), init_year, init_month, 3, hour, minute),
            get_slot(Duration::days(1), init_year, init_month, 4, hour, minute),
            get_slot(Duration::days(1), init_year, init_month, 5, hour, minute),
        ]
        .into_iter()
        .collect(),
    };

    let splitted_timeline = timeline.get_split_into_days();

    dbg!(&splitted_timeline);
    assert_eq!(expected_result, splitted_timeline);
}
