use chrono::{Duration, NaiveDateTime, NaiveTime, Timelike};
use serde::{Deserialize, Deserializer};
use std::ops::{Add, Sub};

// date functions
/// trim NaiveDateTime down to the hour
pub fn normalize_date(date: &NaiveDateTime) -> NaiveDateTime {
    create_date_by_hour(date, date.hour() as usize)
}
pub fn create_date_by_hour(date: &NaiveDateTime, hour: usize) -> NaiveDateTime {
    NaiveDateTime::new(
        date.date(),
        NaiveTime::from_hms_opt(hour as u32, 0, 0).unwrap(),
    )
}
pub fn is_date_between(date: &NaiveDateTime, start: &NaiveDateTime, end: &NaiveDateTime) -> bool {
    normalize_date(date).ge(start) && normalize_date(date).lt(end)
}

pub fn slot_span(start: &NaiveDateTime, end: &NaiveDateTime) -> usize {
    normalize_date(end)
        .signed_duration_since(normalize_date(start))
        .num_hours()
        .unsigned_abs() as usize
}

pub fn inc_span(date: &NaiveDateTime) -> NaiveDateTime {
    inc_span_by(date, 1)
}
pub fn inc_span_by(date: &NaiveDateTime, by: i64) -> NaiveDateTime {
    date.add(Duration::hours(by))
}
pub fn dec_span(date: &NaiveDateTime) -> NaiveDateTime {
    dec_span_by(date, 1)
}
pub fn dec_span_by(date: &NaiveDateTime, by: i64) -> NaiveDateTime {
    date.sub(Duration::hours(by))
}

pub fn deserialize_normalized_date<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(normalize_date(&Deserialize::deserialize(deserializer)?))
}
