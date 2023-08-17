use super::Timeline;
use crate::models::slot::{iterator::SlotIterator, Slot};
use chrono::{Duration, NaiveDateTime, Timelike};

// TODO 2023-05-20  | create edge cases to test behavior when first slot start time in the timeline is not 00 (midnight)
// - Test idea to froce make the start of the timeline from 00 (midnight) of the first whatever even if it is other time in the same day

/*
TimelineIterator goals:
- ability to cros over timeline slots like SlotIterator through interval duration
    - cross over timeline by 1 day duration
- Get count of days, hours, minutes, etc for a timeline

Ideas:
- Ability to move to a specific time in the timeline.
- Ability to perform some action in a timeline without
forcing to split slots into hours, or similar.
-
*/

/// Iterator for a `Timeline` and provide functionalities to walk through
/// slots in a `Timeline` based on custom interval duration
#[derive(Debug, Clone)]
pub struct TimelineIterator {
    timeline: Timeline,
    /// Duration interval for pointer to corss over timeline timelines
    interval: Duration,
}

impl TimelineIterator {
    /// Initialize new TimelineIterator with default interval duration to 1 day
    pub fn initialize(timeline: Timeline) -> TimelineIterator {
        // if let Some(_) = timeline.slots.first() {
        if timeline.slots.first().is_some() {
            TimelineIterator {
                timeline,
                interval: Duration::days(1),
            }
        } else {
            panic!("Timeline slots are empty")
        }
    }

    /// Create new TimelineIterator with custom interval duration
    pub fn new(timeline: Timeline, interval_duration: Duration) -> TimelineIterator {
        if timeline.slots.first().is_some() {
            TimelineIterator {
                timeline,
                interval: interval_duration,
            }
        } else {
            panic!("Timeline slots are empty")
        }
    }

    /// Create new TimelineIterator which iterate for a daily calendar
    /// day regardless time of slots in the timeline
    pub fn new_calendar_day(timeline: Timeline) -> TimelineIterator {
        // TODO 2023-07-11: based on debugging in https://github.com/tijlleenders/ZinZen-scheduler/pull/363
        // for case bug_215, agreed to create a custom TimelineIterator to iterate on daily basis from
        // midnight to midnight.
        if let Some(first_slot) = timeline.slots.first() {
            let start_date = first_slot
                .start
                .with_hour(0)
                .unwrap()
                .with_minute(0)
                .unwrap()
                .with_second(0)
                .unwrap();
            let end_date: NaiveDateTime;
            if timeline.slots.len() == 1 {
                end_date = first_slot.end;
            } else if let Some(last_slot) = timeline.slots.last() {
                end_date = last_slot.end;
            } else {
                panic!("Can't get last timeline slot")
            }

            let custom_timeline = Timeline::initialize(start_date, end_date).unwrap();
            TimelineIterator::initialize(custom_timeline)
        } else {
            panic!("Timeline slots are empty")
        }
    }
}

/// Walk through list of slots in timeline based on custom interval duration
impl Iterator for TimelineIterator {
    type Item = Vec<Slot>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.timeline.slots.is_empty() {
            return None;
        }

        if let Some(first_slot) = self.timeline.slots.first() {
            match self.timeline.slots.take(&first_slot.clone()) {
                Some(slot) => {
                    let slot_duration = slot.end.signed_duration_since(slot.start);

                    // A condition to avoid iteration over slots when inerval > slot duration
                    let slot_iterator: SlotIterator = if self.interval > slot_duration {
                        SlotIterator::new(slot, slot_duration)
                    } else {
                        SlotIterator::new(slot, self.interval)
                    };

                    let mut walking_slots: Vec<Slot> = vec![];
                    for slot in slot_iterator {
                        walking_slots.push(slot);
                    }

                    Some(walking_slots)
                }
                None => None,
            }
        } else {
            None
        }
    }
}

//test case for Slot Iterator impl
#[cfg(test)]
mod tests {
    use super::*;

    /// Testing walk through a Timeline as below:
    /// - Single Slot with 5 days
    /// - walk through each day
    /// Expected to return list of 5 days slots
    #[test]
    fn test_walk_through_single_slot() {
        let timeline_duration = Duration::days(5);
        let interval_duration = Duration::days(1);

        let timeline = Timeline::mock(timeline_duration, 2023, 5, 1);

        let expected_result: Vec<Slot> = vec![
            Slot::mock(Duration::days(1), 2023, 5, 1, 0, 0),
            Slot::mock(Duration::days(1), 2023, 5, 2, 0, 0),
            Slot::mock(Duration::days(1), 2023, 5, 3, 0, 0),
            Slot::mock(Duration::days(1), 2023, 5, 4, 0, 0),
            Slot::mock(Duration::days(1), 2023, 5, 5, 0, 0),
        ];

        let timeline_iterator = TimelineIterator::new(timeline.clone(), interval_duration);

        let mut result: Vec<Slot> = vec![];

        for walking_slots in timeline_iterator {
            result.extend(walking_slots);
        }

        assert_eq!(expected_result, result);
    }

    /// Testing walk through a Timeline as below:
    /// - Multiple Slot {slot1: 5 days, slot2: 3 days}
    /// - walk through each day
    /// Expected to return list of 8 days slots
    #[test]
    fn test_walk_through_mutliple_slots() {
        let interval_duration = Duration::days(1);

        let timeline = Timeline {
            slots: vec![
                Slot::mock(Duration::days(5), 2023, 5, 1, 0, 0),
                Slot::mock(Duration::days(3), 2023, 6, 1, 0, 0),
            ]
            .into_iter()
            .collect(),
        };

        let expected_result: Vec<Slot> = vec![
            Slot::mock(Duration::days(1), 2023, 5, 1, 0, 0),
            Slot::mock(Duration::days(1), 2023, 5, 2, 0, 0),
            Slot::mock(Duration::days(1), 2023, 5, 3, 0, 0),
            Slot::mock(Duration::days(1), 2023, 5, 4, 0, 0),
            Slot::mock(Duration::days(1), 2023, 5, 5, 0, 0),
            Slot::mock(Duration::days(1), 2023, 6, 1, 0, 0),
            Slot::mock(Duration::days(1), 2023, 6, 2, 0, 0),
            Slot::mock(Duration::days(1), 2023, 6, 3, 0, 0),
        ];

        let timeline_iterator = TimelineIterator::new(timeline.clone(), interval_duration);
        let mut result: Vec<Slot> = vec![];

        for walking_slots in timeline_iterator {
            result.extend(walking_slots);
        }

        assert_eq!(expected_result, result);
    }

    /// Test editing slots generated from TimelineIterator as below:
    /// - Single Slot with 5 days
    /// - walk through each day
    /// - edit each day hours to be from 0 - 10AM
    /// Expected to return list of 5 days slots with edited hours
    #[test]
    fn test_edit_slot_from_iterator() {
        let timeline_duration = Duration::days(5);
        let interval_duration = Duration::days(1);

        let timeline = Timeline::mock(timeline_duration, 2023, 5, 1);

        let expected_result: Vec<Slot> = vec![
            Slot::mock(Duration::hours(10), 2023, 5, 1, 0, 0),
            Slot::mock(Duration::hours(10), 2023, 5, 2, 0, 0),
            Slot::mock(Duration::hours(10), 2023, 5, 3, 0, 0),
            Slot::mock(Duration::hours(10), 2023, 5, 4, 0, 0),
            Slot::mock(Duration::hours(10), 2023, 5, 5, 0, 0),
        ];

        let timeline_iterator = TimelineIterator::new(timeline.clone(), interval_duration);

        let mut result: Vec<Slot> = vec![];

        for mut walking_slots in timeline_iterator {
            walking_slots.iter_mut().for_each(|slot| {
                slot.end -= Duration::hours(14);
            });
            result.extend(walking_slots);
        }

        assert_eq!(expected_result, result);
    }

    /// Test when interval duration > slot duration
    /// - Multiple Slots with 5 days (from 5am-3pm)
    /// - interval_duration is 1 days
    /// Expected ??? TODO
    #[test]
    fn test_iterate_when_interval_dur_more_than_slot_duration() {
        let interval_duration = Duration::days(1);
        let year = 2023;
        let start_time: u32 = 5;

        let timeline = Timeline {
            slots: vec![
                Slot::mock(Duration::hours(10), year, 5, 1, start_time, 0),
                Slot::mock(Duration::hours(10), year, 5, 2, start_time, 0),
                Slot::mock(Duration::hours(10), year, 5, 3, start_time, 0),
                Slot::mock(Duration::hours(10), year, 5, 4, start_time, 0),
                Slot::mock(Duration::hours(10), year, 5, 5, start_time, 0),
            ]
            .into_iter()
            .collect(),
        };

        let expected_result: Vec<Slot> = vec![
            Slot::mock(Duration::hours(10), 2023, 5, 1, start_time, 0),
            Slot::mock(Duration::hours(10), 2023, 5, 2, start_time, 0),
            Slot::mock(Duration::hours(10), 2023, 5, 3, start_time, 0),
            Slot::mock(Duration::hours(10), 2023, 5, 4, start_time, 0),
            Slot::mock(Duration::hours(10), 2023, 5, 5, start_time, 0),
        ];

        let timeline_iterator = TimelineIterator::new(timeline.clone(), interval_duration);

        let mut result: Vec<Slot> = vec![];

        for walking_slots in timeline_iterator {
            result.extend(walking_slots);
        }

        assert_eq!(expected_result, result);
    }

    mod new_calendar_day {
        use crate::models::{
            slot::Slot,
            timeline::{iterator::TimelineIterator, Timeline},
        };
        use chrono::Duration;

        /// Test goal hurdl in test case bug_215 but for 3 days
        #[test]
        fn test_hurdle_in_case_bug_215() {
            let timeline_duration = Duration::days(3);

            let timeline_slot = Slot::mock(timeline_duration, 2023, 1, 3, 1, 0);
            let timeline = Timeline {
                slots: vec![timeline_slot].into_iter().collect(),
            };

            let expected_result: Vec<Slot> = vec![
                Slot::mock(Duration::days(1), 2023, 1, 3, 0, 0),
                Slot::mock(Duration::days(1), 2023, 1, 4, 0, 0),
                Slot::mock(Duration::days(1), 2023, 1, 5, 0, 0),
                Slot::mock(Duration::days(1), 2023, 1, 6, 0, 0),
            ];

            let timeline_iterator = TimelineIterator::new_calendar_day(timeline.clone());

            let mut result: Vec<Slot> = vec![];

            for walking_slots in timeline_iterator {
                result.extend(walking_slots);
            }

            assert_eq!(expected_result, result);
        }
    }
}
