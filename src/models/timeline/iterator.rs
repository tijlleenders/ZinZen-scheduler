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

/// Iterator for a `Timeline` and provide some functionalities like count per duration interval
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

    /// Get count between start and end for all slots in
    /// timeline based on Duration interval
    pub fn interval_count(&self) -> usize {
        let mut count = 0;
        dbg!(count);
        self.clone().for_each(|slot| {
            dbg!(&slot);
            let slot_iterator = SlotIterator::new(slot, self.interval);
            dbg!(&slot_iterator);
            count += slot_iterator.interval_count();
            dbg!(&count);
        });

        dbg!(&count);
        count
    }
}

impl Iterator for TimelineIterator {
    type Item = Slot;

    fn next(&mut self) -> Option<Self::Item> {
        dbg!(&self);

        if let Some(slot) = self.timeline.slots.iter().nth(self.pointer) {
            dbg!(&slot);

            self.pointer += 1;
            dbg!(&self);

            Some(slot.clone())
        } else {
            return None;
        }

        // match self.timeline.slots.iter().next() {
        //     Some(slot) => {
        //         dbg!(&slot);
        //         self.pointer = slot.clone();
        //         dbg!(&self);
        //         Some(self.pointer)
        //     }
        //     None => None,
        // }
    }
}

//test case for Slot Iterator impl
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timeline_iterator() {
        let interval_duration = Duration::days(1);

        let slot1 = Slot::mock(Duration::days(2), 2023, 05, 1, 0, 0);
        let slot2 = Slot::mock(Duration::days(4), 2023, 05, 4, 0, 0);
        let timeline = Timeline {
            slots: vec![slot1, slot2].into_iter().collect(),
        };

        let mut iterator = TimelineIterator::new(timeline, interval_duration);

        assert_eq!(iterator.next(), Some(slot1));
        assert_eq!(iterator.next(), Some(slot2));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn test_inteval_count_for_single_slot_timeline() {
        // interval_count
        // interval duration: 1 day
        // timeline: single slot for 5 days
        let interval_duration = Duration::days(1);
        let expected_count: usize = 5;

        let timeline = Timeline::mock(Duration::days(5), 2023, 05, 1);
        let timeline_iterator = TimelineIterator::new(timeline.clone(), interval_duration);
        dbg!(&timeline, &timeline_iterator);

        let result = timeline_iterator.interval_count();
        dbg!(&result);

        assert_eq!(expected_count, result);
    }

    #[test]
    fn test_inteval_count_for_multiple_slot_timeline_1() {
        // interval_count
        // interval duration: 1 day
        // timeline: mutlipe slots (1st: 4 days, 2nd:6 days)
        // expected: 10 days

        let interval_duration = Duration::days(1);
        let expected_count: usize = 10;

        let slot1 = Slot::mock(Duration::days(4), 2023, 05, 1, 0, 0);
        let slot2 = Slot::mock(Duration::days(6), 2023, 05, 10, 0, 0);
        let timeline = Timeline {
            slots: vec![slot1, slot2].into_iter().collect(),
        };

        let timeline_iterator = TimelineIterator::new(timeline.clone(), interval_duration);
        dbg!(&timeline, &timeline_iterator);

        let result = timeline_iterator.interval_count();
        dbg!(&result);

        assert_eq!(expected_count, result);
    }

    #[test]
    fn test_inteval_count_for_multiple_slot_timeline_2() {
        // interval_count
        // interval duration: 1 day
        // timeline: mutlipe slots (1st: 2 months, 2nd: 6 days, 3rd: 10 days )
        // expected: 76 days

        let interval_duration = Duration::days(1);
        let expected_count: usize = 76;

        let slot1 = Slot::mock(Duration::days(60), 2023, 3, 1, 0, 0);
        let slot2 = Slot::mock(Duration::days(6), 2023, 05, 1, 0, 0);
        let slot3 = Slot::mock(Duration::days(10), 2023, 08, 10, 0, 0);
        let timeline = Timeline {
            slots: vec![slot1, slot2, slot3].into_iter().collect(),
        };

        let timeline_iterator = TimelineIterator::new(timeline.clone(), interval_duration);
        dbg!(&timeline, &timeline_iterator);

        let result = timeline_iterator.interval_count();
        dbg!(&result);

        assert_eq!(expected_count, result);
    }
}
