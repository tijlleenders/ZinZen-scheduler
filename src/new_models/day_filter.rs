use std::cmp::Ordering;
use chrono::NaiveTime;
use crate::new_models::date::DateTime;

#[derive(Debug, Clone)]
pub struct DayFilter {
    apply_after: Option<DateTime>,
    apply_before: Option<DateTime>,
}

impl DayFilter {
    /// Input string must be of format "02:34"
    pub fn from_str(after_str: Option<&str>, before_str: Option<&str>) -> Self {
        Self {
            apply_after: Self::get_time(after_str),
            apply_before: Self::get_time(before_str),
        }
    }

    pub fn after(&self, date: &DateTime) -> DateTime {
        date.time_after(&self.apply_after)
    }
    pub fn before(&self, date: &DateTime) -> DateTime {
        date.time_before(&self.apply_before)
    }

    fn get_time(time_str: Option<&str>) -> Option<DateTime> {
        time_str.map(|time_str| {
            if time_str.len() != 5 && &time_str[2..3] != ":"  {
                None
            }
            else {
                match (
                    time_str[..2].parse::<u32>().ok(),
                    time_str[3..].parse::<u32>().ok(),
                ) {
                    (Some(hour), Some(minute)) => NaiveTime::from_hms_opt(hour, minute, 0),
                    _ => None,
                }
            }
        })
            .unwrap().or(None)
            .map(|ref nt| DateTime::from_naive_time(nt))
    }
}

impl Eq for DayFilter {}
impl PartialEq for DayFilter {
    fn eq(&self, other: &Self) -> bool {
        self.apply_after.eq(&other.apply_after)
        && self.apply_before.eq(&other.apply_before)
    }
}

impl Ord for DayFilter {
    fn cmp(&self, other: &Self) -> Ordering {
        let after = self.apply_after.cmp(&other.apply_after);
        if after != Ordering::Equal {
            return after;
        }
        self.apply_before.cmp(&other.apply_before)
    }
}
impl PartialOrd for DayFilter {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
