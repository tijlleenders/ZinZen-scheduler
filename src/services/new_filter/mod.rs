mod filter_not_on;
mod filter_on_days;
mod filter_timing;

use crate::models::{goal::TimeFilter, timeline::Timeline};

impl Timeline {
    /// Applies time filter on the given timeline
    pub fn apply_filter(&mut self, filter: &Option<TimeFilter>) {
        if self.slots.is_empty() {
            return;
        }

        if let Some(filter) = filter {
            dbg!(&filter);

            let mut filtered_timeline = self.clone();
            dbg!(&filtered_timeline);

            if filter.after_time.is_some() || filter.before_time.is_some() {
                filtered_timeline = filter_timing::filter_timing(
                    filtered_timeline,
                    filter.after_time,
                    filter.before_time,
                );
            }
            dbg!(&filtered_timeline);

            if let Some(days) = &filter.on_days {
                filtered_timeline = filter_on_days::filter_on_days(filtered_timeline, days);
            }
            dbg!(&filtered_timeline);

            if let Some(not_on) = &filter.not_on {
                dbg!(&not_on);
                filtered_timeline = filter_not_on::filter_not_on(filtered_timeline, not_on);
            }
            dbg!(&filtered_timeline);

            *self = filtered_timeline;
        } else {
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::Duration;

    use crate::models::{
        goal::{Day, TimeFilter},
        slot::Slot,
        timeline::Timeline,
    };

    #[test]
    fn test_when_timeline_is_empty() {
        let timeline = Timeline::new();

        let filters: Option<TimeFilter> = Some(TimeFilter {
            not_on: None,
            after_time: None,
            before_time: None,
            on_days: Some(vec![Day::Fri]),
        });
        let mut result_timeline = timeline.clone();
        result_timeline.apply_filter(&filters);

        assert_eq!(timeline, result_timeline);
    }

    #[test]
    fn test_when_empty_filters() {
        let timeline = Timeline::mock(Duration::days(1), 2023, 5, 1);

        let filters: Option<TimeFilter> = None;
        let mut result_timeline = timeline.clone();
        result_timeline.apply_filter(&filters);

        assert_eq!(timeline, result_timeline);
    }

    /// Test apply_filter function for a normal workday for an employee
    /// - timeline: 15 days (Starting Mon 2023-05-01 to Mon 2023-05-15)
    /// - filters:
    ///     - after_time: 5am
    ///     - before_time: 3pm
    ///     - on_days: Fri, Sat
    ///     - not_on: [
    ///                 2023-05-02 05 to 15 ,
    ///                 2023-05-06 05 to 15 ,
    ///                 2023-05-11 05 to 15
    ///               ]
    /// - Expected list of total 9 days except Fridays and Saturdays
    /// in addition to  2nd, 6th, and 11th May-2023
    #[test]
    fn test_normal_workday() {
        let year = 2023;
        let month = 5;
        let day = 1;
        let timeline_duration = Duration::days(15);

        let start_time: u32 = 5;
        let after_time: Option<usize> = Some(start_time as usize);
        let end_time: u32 = 15;
        let before_time: Option<usize> = Some(end_time as usize);
        let on_days: Option<Vec<Day>> =
            Some(vec![Day::Sun, Day::Mon, Day::Tue, Day::Wed, Day::Thu]);
        let not_on: Option<Vec<Slot>> = Some(vec![
            Slot::mock(Duration::hours(10), year, month, 2, start_time, 0),
            Slot::mock(Duration::hours(10), year, month, 6, start_time, 0),
            Slot::mock(Duration::hours(10), year, month, 11, start_time, 0),
        ]);

        // intiate a sample timeline
        let timeline = Timeline::mock(timeline_duration, year, month, day);
        dbg!(&timeline);

        let expected_slots: Vec<Slot> = vec![
            Slot::mock(Duration::hours(10), year, month, 1, start_time, 0),
            Slot::mock(Duration::hours(10), year, month, 3, start_time, 0),
            Slot::mock(Duration::hours(10), year, month, 4, start_time, 0),
            Slot::mock(Duration::hours(10), year, month, 7, start_time, 0),
            Slot::mock(Duration::hours(10), year, month, 8, start_time, 0),
            Slot::mock(Duration::hours(10), year, month, 9, start_time, 0),
            Slot::mock(Duration::hours(10), year, month, 10, start_time, 0),
            Slot::mock(Duration::hours(10), year, month, 14, start_time, 0),
            Slot::mock(Duration::hours(10), year, month, 15, start_time, 0),
        ];
        dbg!(&expected_slots);

        let expected_result = Timeline {
            slots: expected_slots.into_iter().collect(),
        };
        dbg!(&expected_result);

        let filters: Option<TimeFilter> = Some(TimeFilter {
            before_time,
            after_time,
            on_days,
            not_on,
        });

        let mut result_timeline = timeline.clone();
        result_timeline.apply_filter(&filters);
        dbg!(&result_timeline);

        assert_eq!(result_timeline, expected_result);
    }
}
