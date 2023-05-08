use super::Slot;
use chrono::{Duration, NaiveDateTime};

/// Iterator for a `Slot` and provide some functionalities like count per duration interval
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

    /// Get count between slot.start and slot.end based on Duration interval
    pub fn interval_count(&self) -> usize {
        let mut count = 0;
        self.clone().for_each(|_| count += 1);
        count
    }
}

impl Iterator for SlotIterator {
    type Item = NaiveDateTime;

    fn next(&mut self) -> Option<Self::Item> {
        dbg!(&self);

        if self.pointer >= self.slot.end {
            return None;
        }
        self.pointer = self.pointer.checked_add_signed(self.interval).unwrap();
        dbg!(&self.pointer);
        Some(self.pointer)
    }
}

//test case for Slot Iterator impl
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDateTime;

    #[test]
    fn test_count_interval_by_iterator_for_4_days() {
        let expected_count: usize = 4;

        let slot = Slot {
            start: NaiveDateTime::parse_from_str("2023-04-26T00:00:00", "%Y-%m-%dT%H:%M:%S")
                .unwrap(),
            end: NaiveDateTime::parse_from_str("2023-04-30T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
        };
        let slot_iterator = SlotIterator::initialize(slot);
        dbg!(&slot, &slot_iterator);

        let mut results: Vec<NaiveDateTime> = vec![];
        for pointer in slot_iterator {
            dbg!(pointer);
            results.push(pointer)
        }

        assert_eq!(expected_count, results.len());
    }

    #[test]
    fn test_interval_count_as_interval_duration_1_day() {
        let interval_duration = Duration::days(1);
        let expected_count: usize = 4;

        let slot = Slot {
            start: NaiveDateTime::parse_from_str("2023-04-26T00:00:00", "%Y-%m-%dT%H:%M:%S")
                .unwrap(),
            end: NaiveDateTime::parse_from_str("2023-04-30T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
        };
        let slot_iterator = SlotIterator::new(slot, interval_duration);
        dbg!(&slot, &slot_iterator);

        let result = slot_iterator.interval_count();
        dbg!(&result);
        assert_eq!(expected_count, result);
    }

    #[test]
    fn test_interval_count_as_interval_duration_1_mintue() {
        let interval_duration = Duration::minutes(1);
        let expected_count: usize = 5760;

        let slot = Slot {
            start: NaiveDateTime::parse_from_str("2023-04-26T00:00:00", "%Y-%m-%dT%H:%M:%S")
                .unwrap(),
            end: NaiveDateTime::parse_from_str("2023-04-30T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
        };
        let slot_iterator = SlotIterator::new(slot, interval_duration);
        dbg!(&slot, &slot_iterator);

        let mut results: Vec<NaiveDateTime> = vec![];
        for pointer in slot_iterator {
            dbg!(pointer);
            results.push(pointer)
        }

        assert_eq!(expected_count, results.len());
    }

    #[test]
    fn test_interval_count_as_interval_duration_1_hour() {
        let interval_duration = Duration::hours(1);
        let expected_count: usize = 5760;

        let slot = Slot {
            start: NaiveDateTime::parse_from_str("2023-04-26T00:00:00", "%Y-%m-%dT%H:%M:%S")
                .unwrap(),
            end: NaiveDateTime::parse_from_str("2023-04-30T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
        };
        let slot_iterator = SlotIterator::new(slot, interval_duration);
        dbg!(&slot, &slot_iterator);

        let mut results: Vec<NaiveDateTime> = vec![];
        for pointer in slot_iterator {
            dbg!(pointer);
            results.push(pointer)
        }

        assert_eq!(expected_count, results.len());
    }
}
