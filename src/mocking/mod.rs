use chrono::{Duration, NaiveDate, NaiveDateTime};

use crate::models::{slot::Slot, timeline::Timeline};

pub struct DateTime {
    pub datetime: NaiveDateTime,
}
impl DateTime {
    /// Get a NaiveDateTime based on a ymd with 0 for hms
    pub fn get_by_date(year: i32, month: u32, day: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(year, month, day)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
    }

    /// Get a NaiveDateTime based on a ymd and hms
    pub fn get_by_datetime(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
    ) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(year, month, day)
            .unwrap()
            .and_hms_opt(hour, minute, second)
            .unwrap()
    }
}

impl Slot {
    pub fn mock(
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
}

impl Timeline {
    /// Utility function to return a timeline list of slots splitted on daily basis
    pub fn mock_as_days(
        days_count: i64,
        start_year: i32,
        start_month: u32,
        start_day: u32,
    ) -> Timeline {
        let init_slot = Slot::mock(
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

    /// Utility function to return a timeline with a single slot with respect to duration
    pub fn mock(duration: Duration, year: i32, month: u32, day: u32) -> Timeline {
        let slot = Slot::mock(duration, year, month, day, 0, 0);
        Timeline {
            slots: vec![slot].into_iter().collect(),
        }
    }
}
