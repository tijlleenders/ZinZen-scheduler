use crate::models::{
    goal::{Day, TimeFilter},
    slot::Slot,
    timeline::Timeline,
};

impl Timeline {
    pub fn apply_filter(&mut self, _filter: &Option<TimeFilter>) -> Timeline {
        todo!("apply_filter not implemented");
    }

    /// Filtering timeline based on before_time and after_time fields in TimeFilter
    fn _filter_timing(&mut self, _before_time: Option<usize>, _after_time: Option<usize>) {
        todo!("filter_timing not implemented");
    }

    /// Filtering timeline based on on_days field in TimeFilter
    fn _filter_on_days(&mut self, _days_to_filter: &[Day]) {
        todo!("filter_on_days not implemented");
    }

    /// Filtering timeline based on not_on field in TimeFilter
    fn _filter_not_on(&mut self, _slots_to_filter: &[Slot]) {
        todo!("filter_not_on not implemented");
    }
}

/// Validate that a given value is valid time number which must be between 0 and 24
fn _validate_time(time: Option<usize>, time_name: &str) {
    if let Some(time) = time {
        if time > 24 {
            panic!("{} must be between 0 and 24", time_name);
        }
    }
}

// create a test case

#[cfg(test)]
mod tests {
    use chrono::{Duration, NaiveDate};

    use crate::models::{slot::Slot, timeline::Timeline};

    #[test]
    fn test_aftertime_only() {
        let before_time = None;
        let after_time = Some(5);

        let init_year = 2022;
        let init_month = 1;
        let init_day = 1;
        let init_duration = Duration::days(1);
        let mut timeline = Timeline::mock(init_duration, init_year, init_month, init_day);
        dbg!(&timeline);

        let expected_result = Timeline {
            slots: vec![Slot {
                start: NaiveDate::from_ymd_opt(init_year, init_month, init_day)
                    .unwrap()
                    .and_hms_opt(5, 0, 0)
                    .unwrap(),
                end: NaiveDate::from_ymd_opt(init_year, init_month, init_day + 1)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
            }]
            .into_iter()
            .collect(),
        };
        dbg!(&expected_result);

        timeline.filter_timing(before_time, after_time);
        dbg!(&timeline);

        assert_eq!(timeline, expected_result);
    }
}
