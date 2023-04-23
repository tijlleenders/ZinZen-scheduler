use std::collections::BTreeSet;

use chrono::NaiveDate;

use crate::models::{slot::Slot, timeline::Timeline};

pub fn get_timeline_fullday() -> Timeline {
    let mut fullday_collection = BTreeSet::new();
    fullday_collection.insert(get_slot_fullday());

    Timeline {
        slots: fullday_collection,
    }
}

/// Generate fullday slot
/// slot: [2022-10-01 00:00:00 --- 2022-10-02 00:00:00]
pub fn get_slot_fullday() -> Slot {
    // slot: [2022-10-01 00:00:00 --- 2022-10-02 00:00:00]
    Slot {
        start: NaiveDate::from_ymd_opt(2022, 10, 1)
            .unwrap()
            .and_hms_opt(00, 0, 0)
            .unwrap(),
        end: NaiveDate::from_ymd_opt(2022, 10, 2)
            .unwrap()
            .and_hms_opt(00, 0, 0)
            .unwrap(),
    }
}

/// Generate 1st halfday slot
/// slot: [2022-10-01 00:00:00 --- 2022-10-01 12:00:00]
pub fn get_slot_halfday_1st() -> Slot {
    Slot {
        start: NaiveDate::from_ymd_opt(2022, 10, 1)
            .unwrap()
            .and_hms_opt(00, 0, 0)
            .unwrap(),
        end: NaiveDate::from_ymd_opt(2022, 10, 1)
            .unwrap()
            .and_hms_opt(12, 0, 0)
            .unwrap(),
    }
}

/// Generate 2nd halfday slot
/// slot: [2022-10-01 12:00:00 --- 2022-10-02 00:00:00]
pub fn get_slot_halfday_2nd() -> Slot {
    // slot: [2022-10-01 12:00:00 --- 2022-10-02 00:00:00]
    Slot {
        start: NaiveDate::from_ymd_opt(2022, 10, 1)
            .unwrap()
            .and_hms_opt(12, 0, 0)
            .unwrap(),
        end: NaiveDate::from_ymd_opt(2022, 10, 2)
            .unwrap()
            .and_hms_opt(00, 0, 0)
            .unwrap(),
    }
}

/// Generate afternoon hours slot
/// slot: [2022-10-01 12:00:00 --- 2022-10-01 15:00:00]
pub fn get_slot_afternoon_hours() -> Slot {
    Slot {
        start: NaiveDate::from_ymd_opt(2022, 10, 1)
            .unwrap()
            .and_hms_opt(12, 0, 0)
            .unwrap(),
        end: NaiveDate::from_ymd_opt(2022, 10, 1)
            .unwrap()
            .and_hms_opt(15, 0, 0)
            .unwrap(),
    }
}

/// Generate afternoon hours slot
/// 1st slot: [2022-10-01 00:00:00 --- 2022-10-01 12:00:00]
/// 2nd slot: [2022-10-01 15:00:00 --- 2022-10-02 00:00:00]
pub fn get_slots_aday_without_afternoon_hours() -> Vec<Slot> {
    vec![
        Slot {
            start: NaiveDate::from_ymd_opt(2022, 10, 1)
                .unwrap()
                .and_hms_opt(00, 0, 0)
                .unwrap(),
            end: NaiveDate::from_ymd_opt(2022, 10, 1)
                .unwrap()
                .and_hms_opt(12, 0, 0)
                .unwrap(),
        },
        Slot {
            start: NaiveDate::from_ymd_opt(2022, 10, 1)
                .unwrap()
                .and_hms_opt(15, 0, 0)
                .unwrap(),
            end: NaiveDate::from_ymd_opt(2022, 10, 2)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        },
    ]
}

// ============

/// Get 2 timeslots:
/// slot1: [2022-10-01 05:00:00 --- 2022-10-01 10:00:00]
/// slot2: [2022-10-01 09:00:00 --- 2022-10-01 02:00:00]
pub fn get_2_slots() -> (Slot, Slot) {
    // slot1: [2022-10-01 05:00:00 --- 2022-10-01 10:00:00]
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
pub fn get_slot_1() -> Slot {
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

pub fn get_slot_2() -> Slot {
    // slot: [2022-10-01 05:00:00 --- 2022-10-01 20:00:00]

    let start = NaiveDate::from_ymd_opt(2022, 10, 1)
        .unwrap()
        .and_hms_opt(20, 0, 0)
        .unwrap();
    let end = NaiveDate::from_ymd_opt(2022, 10, 2)
        .unwrap()
        .and_hms_opt(00, 0, 0)
        .unwrap();

    Slot { start, end }
}
