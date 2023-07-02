use super::Slot;
use chrono::{Duration, NaiveDateTime};

/// Iterator for a `Slot` and provide functionalities to walk through
/// a `Slot` based on custom interval duration
#[derive(Debug, Clone)]
pub struct SlotIterator {
    slot: Slot,
    pointer: NaiveDateTime,
    /// Duration interval for pointer to corss over slot
    interval: Duration,
}
impl SlotIterator {
    /// Initialize new SlotIterator with default interval duration to 1 day
    pub fn initialize(slot: Slot) -> SlotIterator {
        SlotIterator {
            slot,
            pointer: slot.start,
            interval: Duration::days(1),
        }
    }

    /// Create new SlotIterator with custom interval duration
    pub fn new(slot: Slot, interval_duration: Duration) -> SlotIterator {
        SlotIterator {
            slot,
            pointer: slot.start,
            interval: interval_duration,
        }
    }
}

impl Iterator for SlotIterator {
    type Item = Slot;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pointer >= self.slot.end {
            return None;
        }
        let next_pointer = self.pointer + self.interval;

        let slot = Slot {
            start: self.pointer,
            end: next_pointer,
        };
        self.pointer = next_pointer;

        Some(slot)
    }
}

//test case for Slot Iterator impl
#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    /// Testing walk through a slot from 5 days and iterate over each day
    #[test]
    fn test_walk_through_by_day() {
        let slot_duration = Duration::days(5);
        let interval_duration = Duration::days(1);

        let slot = Slot::mock(slot_duration, 2023, 05, 1, 0, 0);

        let expected_result: Vec<Slot> = vec![
            Slot::mock(Duration::days(1), 2023, 05, 1, 0, 0),
            Slot::mock(Duration::days(1), 2023, 05, 2, 0, 0),
            Slot::mock(Duration::days(1), 2023, 05, 3, 0, 0),
            Slot::mock(Duration::days(1), 2023, 05, 4, 0, 0),
            Slot::mock(Duration::days(1), 2023, 05, 5, 0, 0),
        ];

        let slot_iterator = SlotIterator::new(slot, interval_duration);

        let mut result: Vec<Slot> = vec![];

        for slot in slot_iterator {
            result.push(slot);
        }

        assert_eq!(expected_result, result);
    }

    /// Testing walk through a slot from 5 days and iterate over each day
    #[test]
    fn test_walk_through_by_hours() {
        let slot_duration = Duration::hours(5);
        let interval_duration = Duration::hours(1);

        let slot = Slot::mock(slot_duration, 2023, 05, 1, 0, 0);

        let expected_result: Vec<Slot> = vec![
            Slot::mock(Duration::hours(1), 2023, 05, 1, 0, 0),
            Slot::mock(Duration::hours(1), 2023, 05, 1, 1, 0),
            Slot::mock(Duration::hours(1), 2023, 05, 1, 2, 0),
            Slot::mock(Duration::hours(1), 2023, 05, 1, 3, 0),
            Slot::mock(Duration::hours(1), 2023, 05, 1, 4, 0),
        ];

        let slot_iterator = SlotIterator::new(slot, interval_duration);

        let mut result: Vec<Slot> = vec![];

        for slot in slot_iterator {
            result.push(slot);
        }

        assert_eq!(expected_result, result);
    }

    /// Test editing slots generated from SlotIterator as below:
    /// - Single Slot with 5 days
    /// - walk through each day
    /// - edit each day hours to be from 0 - 10AM
    /// Expected to return list of 5 days slots with edited hours
    #[test]
    fn test_edit_slot_from_iterator() {
        let slot_duration = Duration::days(5);
        let interval_duration = Duration::days(1);

        let slot = Slot::mock(slot_duration, 2023, 05, 1, 0, 0);

        let expected_result: Vec<Slot> = vec![
            Slot::mock(Duration::hours(10), 2023, 05, 1, 0, 0),
            Slot::mock(Duration::hours(10), 2023, 05, 2, 0, 0),
            Slot::mock(Duration::hours(10), 2023, 05, 3, 0, 0),
            Slot::mock(Duration::hours(10), 2023, 05, 4, 0, 0),
            Slot::mock(Duration::hours(10), 2023, 05, 5, 0, 0),
        ];

        let slot_iterator = SlotIterator::new(slot, interval_duration);

        let mut result: Vec<Slot> = vec![];

        for mut slot in slot_iterator {
            slot.end = slot.end - Duration::hours(14);
            result.push(slot);
        }

        assert_eq!(expected_result, result);
    }
}
