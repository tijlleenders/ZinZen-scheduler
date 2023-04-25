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

/// Utility function to return a timeline list of slots splitted on daily basis
pub fn get_timeline_days(
    days_count: i64,
    start_year: i32,
    start_month: u32,
    start_day: u32,
) -> Timeline {
    let init_slot = get_slot(
        Duration::days(days_count),
        start_year,
        start_month,
        start_day,
        0,
        0,
    );
    let slots_days = init_slot.divide_into_days();

    Timeline {
        slots: slots_days.into_iter().collect(),
    }
}
