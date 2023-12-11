use chrono::{NaiveDateTime, NaiveTime, Timelike};
use serde::{Deserialize, Deserializer};
pub fn normalize_date(date: &NaiveDateTime) -> NaiveDateTime {
    create_date_by_hour(date, date.hour() as usize)
}
pub fn create_date_by_hour(date: &NaiveDateTime, hour: usize) -> NaiveDateTime {
    NaiveDateTime::new(
        date.date(),
        NaiveTime::from_hms_opt(hour as u32, 0, 0).unwrap(),
    )
}
pub fn deserialize_normalized_date<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(normalize_date(&Deserialize::deserialize(deserializer)?))
}
