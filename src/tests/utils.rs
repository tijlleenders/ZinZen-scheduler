use crate::models::{slot::Slot, timeline::Timeline};
use chrono::{Duration, NaiveDate};

/// Get a slot based on Calendar datetime with Duration
pub fn get_slot(
    duration: Duration,
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
) -> Slot {
    let start = NaiveDate::from_ymd_opt(year, month, day)
        .unwrap()
        .and_hms_opt(hour, minute, 0)
        .unwrap();
    let end = start + duration;

    Slot { start, end }
}

pub fn get_timeline(duration: Duration, year: i32, month: u32, day: u32) -> Timeline {
    let slot = get_slot(duration, year, month, day, 0, 0);
    Timeline {
        slots: vec![slot].into_iter().collect(),
    }
}
