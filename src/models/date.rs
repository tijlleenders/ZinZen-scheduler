use chrono::{NaiveDateTime, NaiveTime, Timelike};
use serde::{Deserialize, Deserializer};

// date functions
/// trim NaiveDateTime down to the hour
pub fn normalize_date(date: &NaiveDateTime) -> NaiveDateTime {
    NaiveDateTime::new(date.date(), NaiveTime::from_hms_opt(date.time().hour(), 0, 0).unwrap())
}
pub fn is_date_between(date: &NaiveDateTime, start: &NaiveDateTime, end: &NaiveDateTime) -> bool {
    normalize_date(date).ge(start) && normalize_date(date).lt(end)
}

pub fn slot_span(start: &NaiveDateTime, end: &NaiveDateTime) -> usize {
    normalize_date(end).signed_duration_since(normalize_date(start)).num_hours().abs() as usize
}

pub fn deserialize_normalized_date<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where D: Deserializer<'de>
{
    Ok(normalize_date(&Deserialize::deserialize(deserializer)?))
}