use std::ops::Add;
use chrono::{Duration, NaiveDateTime, NaiveTime};

pub struct DayFilter {
    apply_after: Option<NaiveTime>,
    apply_before: Option<NaiveTime>,
}

impl DayFilter {
    /// Input string must be of format "02:34"
    pub fn from_str(after_str: Option<&str>, before_str: Option<&str>) -> Self {
        Self {
            apply_after: Self::get_time(after_str),
            apply_before: Self::get_time(before_str),
        }
    }

    pub fn after(&self, date: &NaiveDateTime) -> NaiveDateTime {
        if let Some(ref time) = self.apply_after {
            NaiveDateTime::new(date.date(), time.clone())

        } else {
            NaiveDateTime::new(date.date(), NaiveTime::default())
        }
    }
    pub fn before(&self, date: &NaiveDateTime) -> NaiveDateTime {
        if let Some(ref time) = self.apply_before {
            NaiveDateTime::new(date.date(), time.clone())

        } else {
            NaiveDateTime::new(date.date().add(Duration::days(1)), NaiveTime::default())
        }
    }

    fn get_time(time_str: Option<&str>) -> Option<NaiveTime> {
        time_str.map(|time_str| {
            if time_str.len() != 5 {
                None
            }
            else {
                match (
                    u32::from_str_radix(&time_str[..2], 10).ok(),
                    u32::from_str_radix(&time_str[..2], 10).ok(),
                ) {
                    (Some(hour), Some(minute)) => NaiveTime::from_hms_opt(hour, minute, 0),
                    _ => None,
                }
            }
        })
            .unwrap().or(None)
    }
}
