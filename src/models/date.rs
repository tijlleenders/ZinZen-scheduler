use chrono::{NaiveDateTime, NaiveTime, Timelike};

// date functions
/// trim NaiveDateTime down to the hour
pub fn normalize_date(date: &NaiveDateTime) -> NaiveDateTime {
    NaiveDateTime::new(date.date(), NaiveTime::from_hms_opt(date.time().hour(), 0, 0).unwrap())
}
pub fn is_date_between(date: &NaiveDateTime, start: &NaiveDateTime, end: &NaiveDateTime) -> bool {
    normalize_date(date).ge(start) && normalize_date(date).lt(end)
}

pub fn slot_span(start: &NaiveDateTime, end: &NaiveDateTime) -> usize {
    end.signed_duration_since(*start).num_hours().abs() as usize
}