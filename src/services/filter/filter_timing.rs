use chrono::{Duration, Timelike};

use crate::models::{
    slot::Slot,
    timeline::{iterator::TimelineIterator, Timeline},
};

/// Filtering timeline based on before_time and after_time fields in TimeFilter
pub(crate) fn filter_timing(
    timeline: Timeline,
    after_time: Option<usize>,
    before_time: Option<usize>,
) -> Timeline {
    // return the same timeline if there is no slots, if after_time and before_time is None
    if timeline.slots.is_empty() || (after_time.is_none() && before_time.is_none()) {
        return timeline;
    }
    // Some validations
    validate_time(after_time, "after_time");
    validate_time(before_time, "before_time");
    dbg!(&timeline, &before_time, &after_time);

    let mut expected_timeline = Timeline::new();
    let timeline_iterator = TimelineIterator::new(timeline, Duration::days(1));

    let mut slots: Vec<Slot> = vec![];

    for mut walking_slots in timeline_iterator {
        dbg!(&walking_slots);
        walking_slots.iter_mut().for_each(|mut slot| {
            dbg!(&slot);

            if after_time.is_some() && before_time.is_some() {}
            let slot_duration = slot.end.signed_duration_since(slot.start);
            dbg!(&slot_duration);
            if let Some(start_time) = after_time {
                slot.start = slot.start.with_hour(start_time as u32).unwrap();
            }
            dbg!(&slot);

            if let Some(end_time) = before_time {
                if after_time.is_some() && end_time < after_time.unwrap() {
                    slot.end = slot.end.with_hour(end_time as u32).unwrap();
                    dbg!(&slot);
                } else {
                    slot.end = slot.end.with_hour(end_time as u32).unwrap() - Duration::days(1);
                    dbg!(&slot);
                }
            }
        });
        dbg!(&walking_slots);

        slots.extend(walking_slots);
    }
    expected_timeline.slots = slots.into_iter().collect();

    expected_timeline
}

/// Validate that a given value is valid time number which must be between 0 and 24
fn validate_time(time: Option<usize>, time_name: &str) {
    if let Some(time) = time {
        if time > 24 {
            panic!("{} must be between 0 and 24", time_name);
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::Duration;

    use crate::{
        models::{slot::Slot, timeline::Timeline},
        services::filter::filter_timing::filter_timing,
    };

    /// Test filter_timing when Timeline.slots is empty
    /// and before_time and after_time are None
    /// - Expected to return the same empty timeline
    #[test]
    fn test_when_timeline_is_empty() {
        let timeline = Timeline::new();

        let result = filter_timing(timeline.clone(), None, None);
        assert_eq!(timeline, result);
    }

    /// Test filter_timing when Timeline.slots is empty
    /// and before_time and after_time have values
    /// - Expected to return the same empty timeline
    #[test]
    fn test_when_timeline_is_empty_with_param_is_value() {
        let timeline = Timeline::new();

        let result = filter_timing(timeline.clone(), Some(5), Some(20));
        assert_eq!(timeline, result);
    }

    /// Test filter_timing when before_time and after_time are None
    /// Expected to return same timeline
    #[test]
    fn test_when_filter_params_are_none() {
        let timeline = Timeline::mock(Duration::days(5), 2023, 05, 1);

        let result = filter_timing(timeline.clone(), None, None);
        assert_eq!(timeline, result);
    }

    /// Test filter_timing function if only after_time provided
    /// - timeline: 5 days
    /// - after_time (start_time): 5
    /// - before_time: None
    /// - Expected list of 5 days starting time from 5am for each day
    #[test]
    fn test_filter_aftertime_only() {
        let timeline_duration = Duration::days(5);
        let start_time: u32 = 5;

        let timeline = Timeline::mock(timeline_duration, 2023, 05, 1);
        dbg!(&timeline);

        let expected_result: Timeline = Timeline {
            slots: vec![
                Slot::mock(Duration::hours(24 - 5), 2023, 05, 1, start_time, 0),
                Slot::mock(Duration::hours(24 - 5), 2023, 05, 2, start_time, 0),
                Slot::mock(Duration::hours(24 - 5), 2023, 05, 3, start_time, 0),
                Slot::mock(Duration::hours(24 - 5), 2023, 05, 4, start_time, 0),
                Slot::mock(Duration::hours(24 - 5), 2023, 05, 5, start_time, 0),
            ]
            .into_iter()
            .collect(),
        };
        dbg!(&expected_result);

        let result = filter_timing(timeline, Some(start_time as usize), None);

        assert_eq!(expected_result, result);
    }

    /// Test filter_timing function if only before_time provided
    /// - timeline: 5 days
    /// - after_time (start_time): None
    /// - before_time (end_time): 20 (8pm)
    /// - Expected list of 5 days starting time from 00 and end at 20 for each day
    #[test]
    fn test_filter_beforetime_only() {
        let timeline_duration = Duration::days(5);
        let end_time: u32 = 20;

        let timeline = Timeline::mock(timeline_duration, 2023, 05, 1);
        dbg!(&timeline);

        let expected_result: Timeline = Timeline {
            slots: vec![
                Slot::mock(Duration::hours(20), 2023, 05, 1, 0, 0),
                Slot::mock(Duration::hours(20), 2023, 05, 2, 0, 0),
                Slot::mock(Duration::hours(20), 2023, 05, 3, 0, 0),
                Slot::mock(Duration::hours(20), 2023, 05, 4, 0, 0),
                Slot::mock(Duration::hours(20), 2023, 05, 5, 0, 0),
            ]
            .into_iter()
            .collect(),
        };
        dbg!(&expected_result);

        let result = filter_timing(timeline, None, Some(end_time as usize));

        assert_eq!(expected_result, result);
    }

    /// Test filter_timing function when both after_time and before_time are provided
    /// - timeline: 5 days
    /// - after_time (start_time): 5 (5am)
    /// - before_time (end_time): 20 (8pm)
    /// - Expected list of 5 days starting time from 05 and end at 20 for each day
    #[test]
    fn test_filter_with_both_beforetime_and_aftertime() {
        let timeline_duration = Duration::days(5);
        let start_time: u32 = 5;
        let end_time: u32 = 20;

        let timeline = Timeline::mock(timeline_duration, 2023, 05, 1);
        dbg!(&timeline);

        let expected_result: Timeline = Timeline {
            slots: vec![
                Slot::mock(Duration::hours(15), 2023, 05, 1, start_time, 0),
                Slot::mock(Duration::hours(15), 2023, 05, 2, start_time, 0),
                Slot::mock(Duration::hours(15), 2023, 05, 3, start_time, 0),
                Slot::mock(Duration::hours(15), 2023, 05, 4, start_time, 0),
                Slot::mock(Duration::hours(15), 2023, 05, 5, start_time, 0),
            ]
            .into_iter()
            .collect(),
        };
        dbg!(&expected_result);

        let result = filter_timing(timeline, Some(start_time as usize), Some(end_time as usize));
        dbg!(&expected_result, &result);
        assert_eq!(expected_result, result);
    }

    /// Test filter_timing function when both after_time and before_tNoneime are provided
    /// but before_time(end_time) less than after_time (start_time)
    /// - timeline: 5 days
    /// - after_time (start_time): 20 (8pm)
    /// - before_time (end_time): 05 (5am)
    /// - Expected list of 5 days starting time from 8pm and end at 5am next day
    #[test]
    fn test_beforetime_is_before_aftertime() {
        let timeline_duration = Duration::days(5);
        let start_time: u32 = 20;
        let end_time: u32 = 5;

        let timeline = Timeline::mock(timeline_duration, 2023, 05, 1);
        dbg!(&timeline);

        let expected_result: Timeline = Timeline {
            slots: vec![
                Slot::mock(Duration::hours(9), 2023, 05, 1, start_time, 0),
                Slot::mock(Duration::hours(9), 2023, 05, 2, start_time, 0),
                Slot::mock(Duration::hours(9), 2023, 05, 3, start_time, 0),
                Slot::mock(Duration::hours(9), 2023, 05, 4, start_time, 0),
                Slot::mock(Duration::hours(9), 2023, 05, 5, start_time, 0),
            ]
            .into_iter()
            .collect(),
        };
        dbg!(&expected_result);

        let result = filter_timing(timeline, Some(start_time as usize), Some(end_time as usize));
        dbg!(&expected_result, &result);
        assert_eq!(expected_result, result);
    }

    // TODO 2023-05-15  | create a test scenario like test_beforetime_is_before_aftertime but slots passed in timeline not fullday
    // TODO 2023-04-25  | Remove panic and use error handling for below validations
    // TODO 2023-04-26  | Create test scenarios: when slot is not a complete day
}
