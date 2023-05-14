use chrono::{Duration, NaiveDate, NaiveDateTime};

use crate::models::{
    slot::{iterator::SlotIterator, Slot},
    timeline::Timeline,
};

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
        if days_count < 1 {
            return Timeline::new();
        }
        let init_slot = Slot::mock(
            Duration::days(days_count),
            start_year,
            start_month,
            start_day,
            0,
            0,
        );

        let slot_iter = SlotIterator::new(init_slot, Duration::days(1));

        let mut slots_days: Vec<Slot> = vec![];
        for slot in slot_iter {
            slots_days.push(slot);
        }

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

#[cfg(test)]
mod tests {
    use chrono::{Datelike, Timelike};

    use super::*;

    #[test]
    fn test_mock_slot() {
        let duration = Duration::hours(1);
        let year = 2023;
        let month = 05;
        let day = 1;
        let hour = 5;
        let minute = 0;

        let slot = Slot::mock(duration, year, month, day, hour, minute);

        assert_eq!(slot.start.year(), year);
        assert_eq!(slot.start.month(), month);
        assert_eq!(slot.start.day(), day);
        assert_eq!(slot.start.hour(), hour);
        assert_eq!(slot.start.minute(), minute);

        assert_eq!(slot.end.hour(), hour + 1);
        assert_eq!(slot.end.minute(), minute);
    }

    #[test]
    fn test_mock_slot_for_day() {
        let duration = Duration::days(1);
        let year = 2023;
        let month = 05;
        let day = 1;
        let hour = 5;
        let minute = 0;

        let slot = Slot::mock(duration, year, month, day, hour, minute);

        assert_eq!(slot.start.year(), year);
        assert_eq!(slot.start.month(), month);
        assert_eq!(slot.start.day(), day);
        assert_eq!(slot.start.hour(), hour);
        assert_eq!(slot.start.minute(), minute);

        assert_eq!(slot.end.day(), day + 1);
        assert_eq!(slot.end.hour(), hour);
        assert_eq!(slot.end.minute(), minute);
    }

    #[test]
    fn test_mock_as_days() {
        // Test for days_count = 0
        let timeline = Timeline::mock_as_days(0, 2023, 5, 1);
        assert_eq!(timeline, Timeline::new());

        // Test for days_count = 1
        let timeline = Timeline::mock_as_days(1, 2023, 5, 1);
        assert_eq!(
            timeline,
            Timeline {
                slots: vec![Slot::mock(Duration::days(1), 2023, 5, 1, 0, 0)]
                    .into_iter()
                    .collect()
            }
        );

        // Test for days_count = 3
        let timeline = Timeline::mock_as_days(3, 2023, 5, 1);
        assert_eq!(
            timeline,
            Timeline {
                slots: vec![
                    Slot::mock(Duration::days(1), 2023, 5, 1, 0, 0),
                    Slot::mock(Duration::days(1), 2023, 5, 2, 0, 0),
                    Slot::mock(Duration::days(1), 2023, 5, 3, 0, 0),
                ]
                .into_iter()
                .collect()
            }
        );
    }
}
