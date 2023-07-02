pub mod merge_slots;
pub mod remove_slots;

use crate::models::{slot::Slot, timeline::Timeline};
use chrono::Duration;
use std::collections::BTreeSet;

#[test]
fn test_initialize() {
    let sample_slot = Slot::mock(Duration::hours(15), 2022, 10, 1, 05, 0);

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
        Slot::mock(Duration::hours(2), 2022, 10, 1, 01, 0),
        Slot::mock(Duration::hours(3), 2022, 10, 1, 03, 0),
        Slot::mock(Duration::hours(4), 2022, 10, 1, 07, 0),
        Slot::mock(Duration::hours(10), 2022, 10, 1, 12, 0),
    );

    let timeline = Timeline {
        slots: vec![slot1, slot2, slot3, slot4].into_iter().collect(),
    };

    if let Some(next_slot) = timeline.get_slot(1) {
        assert_eq!(slot2, next_slot);
    } else {
        assert!(false);
    }
}
