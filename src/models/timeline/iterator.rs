use super::Timeline;
use crate::models::slot::{iterator::SlotIterator, Slot};
use chrono::Duration;

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
    pointer: usize,
    /// Duration interval for pointer to corss over timeline timelines
    interval: Duration,
}

impl TimelineIterator {
    /// Initialize new TimelineIterator with default interval duration to 1 day
    pub fn initialize(timeline: Timeline) -> TimelineIterator {
        if let Some(_pionter_start) = timeline.slots.first() {
            TimelineIterator {
                timeline: timeline.clone(),
                pointer: 0,
                interval: Duration::days(1),
            }
        } else {
            panic!("Timeline slots are empty")
        }
    }

    /// Create new TimelineIterator with custom interval duration
    pub fn new(timeline: Timeline, interval_duration: Duration) -> TimelineIterator {
        if let Some(_pionter_start) = timeline.slots.first() {
            TimelineIterator {
                timeline: timeline.clone(),
                pointer: 0,
                interval: interval_duration,
            }
        } else {
            panic!("Timeline slots are empty")
        }
    }
}

/// Walk through list of slots in timeline based on custom interval duration
impl Iterator for TimelineIterator {
    type Item = Vec<Slot>;

    fn next(&mut self) -> Option<Self::Item> {
        dbg!(&self);

        if self.timeline.slots.is_empty() {
            return None;
        }

        if let Some(slot) = self.timeline.slots.iter().nth(self.pointer) {
            dbg!(&slot);

            let slot_iterator = SlotIterator::new(*slot, self.interval);
            dbg!(&slot_iterator);

            let mut walking_slots: Vec<Slot> = vec![];
            for slot in slot_iterator {
                walking_slots.push(slot);
            }
            dbg!(&walking_slots);

            self.pointer += 1;
            dbg!(&self);

            Some(walking_slots)
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

        let timeline = Timeline::mock(timeline_duration, 2023, 05, 1);
        dbg!(&timeline);

        let expected_result: Vec<Slot> = vec![
            Slot::mock(Duration::days(1), 2023, 05, 1, 0, 0),
            Slot::mock(Duration::days(1), 2023, 05, 2, 0, 0),
            Slot::mock(Duration::days(1), 2023, 05, 3, 0, 0),
            Slot::mock(Duration::days(1), 2023, 05, 4, 0, 0),
            Slot::mock(Duration::days(1), 2023, 05, 5, 0, 0),
        ];
        dbg!(&expected_result);

        let timeline_iterator = TimelineIterator::new(timeline.clone(), interval_duration);
        dbg!(&timeline, &timeline_iterator);

        let mut result: Vec<Slot> = vec![];

        for walking_slots in timeline_iterator {
            dbg!(&walking_slots);
            result.extend(walking_slots);
        }
        dbg!(&expected_result, &result);

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
                Slot::mock(Duration::days(5), 2023, 05, 1, 0, 0),
                Slot::mock(Duration::days(3), 2023, 06, 1, 0, 0),
            ]
            .into_iter()
            .collect(),
        };
        dbg!(&timeline);

        let expected_result: Vec<Slot> = vec![
            Slot::mock(Duration::days(1), 2023, 05, 1, 0, 0),
            Slot::mock(Duration::days(1), 2023, 05, 2, 0, 0),
            Slot::mock(Duration::days(1), 2023, 05, 3, 0, 0),
            Slot::mock(Duration::days(1), 2023, 05, 4, 0, 0),
            Slot::mock(Duration::days(1), 2023, 05, 5, 0, 0),
            Slot::mock(Duration::days(1), 2023, 06, 1, 0, 0),
            Slot::mock(Duration::days(1), 2023, 06, 2, 0, 0),
            Slot::mock(Duration::days(1), 2023, 06, 3, 0, 0),
        ];
        dbg!(&expected_result);

        let timeline_iterator = TimelineIterator::new(timeline.clone(), interval_duration);
        dbg!(&timeline, &timeline_iterator);

        let mut result: Vec<Slot> = vec![];

        for walking_slots in timeline_iterator {
            dbg!(&walking_slots);
            result.extend(walking_slots);
        }
        dbg!(&expected_result, &result);

        assert_eq!(expected_result, result);
    }
}
