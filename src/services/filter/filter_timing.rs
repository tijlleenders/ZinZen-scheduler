use chrono::{Duration, Timelike};

use crate::models::{
    slot::Slot,
    slots_iterator::{utils::determine_timing_scenario, TimingScenario},
    timeline::{iterator::TimelineIterator, Timeline},
};

/// Filtering timeline based on before_time and after_time fields in TimeFilter
pub(crate) fn filter_timing(
    timeline: Timeline,
    after_time: Option<usize>,
    before_time: Option<usize>,
) -> Timeline {
    // Return the same timeline if there are no slots, or if both `after_time` and `before_time` are None
    if timeline.slots.is_empty() || (after_time.is_none() && before_time.is_none()) {
        return timeline;
    }

    validate_time(after_time, "after_time");
    validate_time(before_time, "before_time");

    // Determine the timing scenario based on the `after_time` and `before_time` inputs
    let timing_scenario = determine_timing_scenario(after_time, before_time);
    let mut expected_timeline = Timeline::new();
    let timeline_iterator = TimelineIterator::new(timeline, Duration::days(1));
    let mut slots: Vec<Slot> = vec![];

    match timing_scenario {
        TimingScenario::Unbounded => {
            // If the timing scenario is `Unbounded`, keep all slots as is
            for walking_slots in timeline_iterator {
                slots.extend(walking_slots);
            }
        }
        TimingScenario::AfterOnly => {
            // If the timing scenario is `AfterOnly`, adjust the start time of each slot
            // Rule: make sure that 'after_time' within slot boundaries
            for mut walking_slots in timeline_iterator {
                walking_slots.iter_mut().for_each(|mut slot| {
                    let after_time = after_time.unwrap() as u32;
                    let slot_start_hour = slot.start.hour();
                    if after_time < slot_start_hour {
                        slot.start = slot.start.with_hour(slot_start_hour).unwrap();
                    } else {
                        slot.start = slot.start.with_hour(after_time).unwrap();
                    }
                    slots.push(*slot);
                });
            }
        }
        TimingScenario::BeforeOnly => {
            // If the timing scenario is `BeforeOnly`, adjust the end time of each slot
            for mut walking_slots in timeline_iterator {
                walking_slots.iter_mut().for_each(|mut slot| {
                    slot.end = slot.end.with_hour(before_time.unwrap() as u32).unwrap()
                        - Duration::days(1);
                    slots.push(*slot);
                });
            }
        }
        TimingScenario::Bounded => {
            // If the timing scenario is `Bounded`, adjust both the start and end times of each slot
            for mut walking_slots in timeline_iterator {
                walking_slots.iter_mut().for_each(|mut slot| {
                    slot.start = slot.start.with_hour(after_time.unwrap() as u32).unwrap();
                    slot.end = slot.end.with_hour(before_time.unwrap() as u32).unwrap()
                        - Duration::days(1);
                    slots.push(*slot);
                });
            }
        }
        TimingScenario::Overflow => {
            // If the timing scenario is `Overflow`
            for (iterator_index, mut walking_slots) in timeline_iterator.enumerate() {
                dbg!(&iterator_index, &walking_slots);
                let walking_slots_len = walking_slots.len();
                for (walking_index, mut slot) in walking_slots.iter_mut().enumerate() {
                    dbg!(&walking_index, &slot);
                    // ===
                    // Below condition to handle case as comment: https://github.com/tijlleenders/ZinZen-scheduler/pull/295#issuecomment-1550956264
                    // If this is the first slot in the first day of the timeline,
                    // add a new slot that starts at the beginning of the day
                    // and ends at the specified `before_time`
                    if iterator_index == 0 && walking_index == 0 {
                        slots.push(Slot {
                            start: slot.start,
                            end: slot.end.with_hour(before_time.unwrap() as u32).unwrap(),
                        });
                        dbg!(&slots);
                    }
                    // ===
                    // Condition added as per issue in PR https://github.com/tijlleenders/ZinZen-scheduler/pull/317
                    // if it is last slot in the timeline,
                    // add a new slot that starts at the specified `after_time`
                    // TODO 2023-06-08: confirm behavior if many slots provided in Timeline
                    else if walking_index == walking_slots_len - 1 {
                        slots.push(Slot {
                            start: slot.start.with_hour(after_time.unwrap() as u32).unwrap(),
                            end: slot.end,
                        });
                        dbg!(&slots);

                        continue;
                    }
                    // ===
                    slot.start = slot.start.with_hour(after_time.unwrap() as u32).unwrap();
                    slot.end = slot.end.with_hour(before_time.unwrap() as u32).unwrap();
                    slots.push(*slot);
                    dbg!(&slots);

                    // ===
                }
            }
        }
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

    mod timing_scenario {
        use crate::services::filter::filter_timing::{determine_timing_scenario, TimingScenario};

        /// Test the scenario where both `after_time` and `before_time` are `None`,
        /// which should result in the `Unbounded` variant
        #[test]
        pub(crate) fn test_unbounded() {
            let scenario = determine_timing_scenario(None, None);
            assert_eq!(scenario, TimingScenario::Unbounded);
        }

        /// Test the scenario where only `after_time` is defined,
        /// which should result in the `AfterOnly` variant
        #[test]
        pub(crate) fn test_after_only() {
            let scenario = determine_timing_scenario(Some(10), None);
            assert_eq!(scenario, TimingScenario::AfterOnly);
        }

        /// Test the scenario where only `before_time` is defined,
        /// which should result in the `BeforeOnly` variant
        #[test]
        pub(crate) fn test_before_only() {
            let scenario = determine_timing_scenario(None, Some(20));
            assert_eq!(scenario, TimingScenario::BeforeOnly);
        }

        /// Test the scenario where both `after_time` and `before_time` are defined and
        /// `after_time` is less than `before_time`, which should result in
        /// the `Bounded` variant
        #[test]
        pub(crate) fn test_bounded() {
            let scenario = determine_timing_scenario(Some(10), Some(20));
            assert_eq!(scenario, TimingScenario::Bounded);
        }

        /// Test the scenario where both `after_time` and `before_time` are defined and
        /// `after_time` is equal to `before_time`, which should result in
        /// the `Bounded` variant
        #[test]
        pub(crate) fn test_bounded_and_both_are_equal() {
            let scenario = determine_timing_scenario(Some(10), Some(10));
            assert_eq!(scenario, TimingScenario::Bounded);
        }

        /// Test the scenario where both `after_time` and `before_time` are defined and
        /// `after_time` is greater than `before_time`, which should result in the
        /// `Overflow` variant
        #[test]
        pub(crate) fn test_overflow() {
            let scenario = determine_timing_scenario(Some(20), Some(10));
            assert_eq!(scenario, TimingScenario::Overflow);
        }
    }

    /// Test filter_timing when Timeline.slots is empty
    /// and before_time and after_time are None
    /// - Expected to return the same empty timeline
    #[test]
    fn test_timeline_is_empty() {
        let timeline = Timeline::new();

        let result = filter_timing(timeline.clone(), None, None);
        assert_eq!(timeline, result);
    }

    /// Test filter_timing when Timeline.slots is empty
    /// and before_time and after_time have values
    /// - Expected to return the same empty timeline
    #[test]
    fn test_timeline_is_empty_with_unbounded_timing() {
        let timeline = Timeline::new();

        let result = filter_timing(timeline.clone(), Some(5), Some(20));
        assert_eq!(timeline, result);
    }

    /// Test filter_timing when before_time and after_time are None
    /// Expected to return same timeline
    #[test]
    fn test_unbounded() {
        let timeline = Timeline::mock(Duration::days(5), 2023, 05, 1);

        let result = filter_timing(timeline.clone(), None, None);
        assert_eq!(timeline, result);
    }

    /// Test filter_timing function if only after_time provided
    /// - timeline: 5 days
    /// - after_time: 5
    /// - before_time: None
    /// - Expected list of 5 days starting time from 5am for each day
    #[test]
    fn test_aftertime_only() {
        let timeline_duration = Duration::days(5);
        let after: u32 = 5;

        let timeline = Timeline::mock(timeline_duration, 2023, 05, 1);

        let expected_result: Timeline = Timeline {
            slots: vec![
                Slot::mock(Duration::hours(24 - 5), 2023, 05, 1, after, 0),
                Slot::mock(Duration::hours(24 - 5), 2023, 05, 2, after, 0),
                Slot::mock(Duration::hours(24 - 5), 2023, 05, 3, after, 0),
                Slot::mock(Duration::hours(24 - 5), 2023, 05, 4, after, 0),
                Slot::mock(Duration::hours(24 - 5), 2023, 05, 5, after, 0),
            ]
            .into_iter()
            .collect(),
        };

        let result = filter_timing(timeline, Some(after as usize), None);

        assert_eq!(expected_result, result);
    }

    /// Test filter_timing function if only before_time provided
    /// - timeline: 5 days
    /// - after_time: None
    /// - before_time: 20 (8pm)
    /// - Expected list of 5 days starting time from 00 and end at 20 for each day
    #[test]
    fn test_beforetime_only() {
        let timeline_duration = Duration::days(5);
        let before: u32 = 20;

        let timeline = Timeline::mock(timeline_duration, 2023, 05, 1);

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

        let result = filter_timing(timeline, None, Some(before as usize));

        assert_eq!(expected_result, result);
    }

    /// Test filter_timing function when both after_time and before_time are provided
    /// - timeline: 5 days
    /// - after_time: 5 (5am)
    /// - before_time: 20 (8pm)
    /// - Expected list of 5 days starting time from 05 and end at 20 for each day
    #[test]
    fn test_bounded() {
        let timeline_duration = Duration::days(5);
        let after: u32 = 5;
        let before: u32 = 20;

        let timeline = Timeline::mock(timeline_duration, 2023, 05, 1);

        let expected_result: Timeline = Timeline {
            slots: vec![
                Slot::mock(Duration::hours(15), 2023, 05, 1, after, 0),
                Slot::mock(Duration::hours(15), 2023, 05, 2, after, 0),
                Slot::mock(Duration::hours(15), 2023, 05, 3, after, 0),
                Slot::mock(Duration::hours(15), 2023, 05, 4, after, 0),
                Slot::mock(Duration::hours(15), 2023, 05, 5, after, 0),
            ]
            .into_iter()
            .collect(),
        };

        let result = filter_timing(timeline, Some(after as usize), Some(before as usize));

        assert_eq!(expected_result, result);
    }

    /// Test filter_timing function when both after_time and before_time are provided
    /// but before_time less than after_time (overflow case)
    /// - timeline: 5 days
    /// - after_time: 20 (8pm)
    /// - before_time: 05 (5am)
    /// - Expected list of 5 days starting time from 8pm and end at 5am next day
    /// expept for last day will be till end of day not 5am next day
    #[test]
    fn test_overflow() {
        let timeline_duration = Duration::days(6);
        let after: u32 = 20;
        let before: u32 = 5;

        let mut timeline_slot = Slot::mock(timeline_duration, 2023, 4, 30, after, 0);
        timeline_slot.end -= Duration::hours((after - before) as i64);
        dbg!(&timeline_slot);
        let timeline = Timeline {
            slots: vec![timeline_slot].into_iter().collect(),
        };

        let expected_result: Timeline = Timeline {
            slots: vec![
                Slot::mock(Duration::hours(9), 2023, 04, 30, after, 0),
                Slot::mock(Duration::hours(9), 2023, 05, 1, after, 0),
                Slot::mock(Duration::hours(9), 2023, 05, 2, after, 0),
                Slot::mock(Duration::hours(9), 2023, 05, 3, after, 0),
                Slot::mock(Duration::hours(9), 2023, 05, 4, after, 0),
                Slot::mock(Duration::hours(24), 2023, 05, 5, after, 0),
            ]
            .into_iter()
            .collect(),
        };

        let result = filter_timing(timeline, Some(after as usize), Some(before as usize));
        dbg!(&expected_result, &result);
        assert_eq!(expected_result, result);
    }

    /// Test edge case to not exceed boundaries of provided timeline
    /// This edge case issued by test 'non_midnight_start_deadline'
    /// which after_time is before timeline boundaries
    /// - timeline: partial day 2022-11-30 10 to 11
    /// - after_time: 01 (1am)
    /// - before_time: None
    /// - Expected same timeline day
    #[test]
    fn test_after_time_within_timeline_boundaries() {
        let timeline: Timeline = Timeline {
            slots: vec![Slot::mock(Duration::hours(1), 2022, 04, 30, 10, 0)]
                .into_iter()
                .collect(),
        };

        let expected_result: Timeline = Timeline {
            slots: vec![Slot::mock(Duration::hours(1), 2022, 04, 30, 10, 0)]
                .into_iter()
                .collect(),
        };

        let result = filter_timing(timeline, Some(1), None);

        assert_eq!(expected_result, result);
    }

    // TODO 2023-05-15  | create a test scenario like test_beforetime_is_before_aftertime but slots passed in timeline not fullday
    // TODO 2023-04-25  | Remove panic and use error handling for below validations
    // TODO 2023-04-26  | Create test scenarios: when slot is not a complete day
}
