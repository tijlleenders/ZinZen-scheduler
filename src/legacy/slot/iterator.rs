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
    #[allow(dead_code)]
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
