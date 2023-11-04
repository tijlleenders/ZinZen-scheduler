use super::Slot;
use chrono::NaiveDateTime;
use std::{
    cmp::{max, min},
    ops::{Add, Sub},
};
use crate::models::date::slot_span;

impl Sub for Slot {
    type Output = Vec<Slot>;

    fn sub(self, rhs: Slot) -> Vec<Slot> {
        let mut result = Vec::new();

        if rhs.start > self.end || rhs.end < self.start {
            // If the two slots don't overlap, just return the original slot
            result.push(self);
            return result;
        }
        if rhs.start == self.start && rhs.end == self.end {
            // If rhs is the same as self, just return empty
            return result;
        }
        if rhs.start < self.start && rhs.end > self.end {
            // If rhs completely encompasses self, then return empty list
            return vec![];
        }

        if rhs.start > self.start {
            result.push(Slot::new(self.start, rhs.start));
        }

        if rhs.end < self.end {
            result.push(Slot::new(rhs.end, self.end));
        }

        result
    }
}

// ===========================
// ===========================

/// Add a slot if they are consequent. If not, will return None
impl Add for Slot {
    type Output = Option<Slot>;

    fn add(self, other: Self) -> Self::Output {
        if (other.start < self.start) && (other.end == self.start) {
            //other is before self and touching it
            let slot = Slot {
                start: other.start,
                end: self.end,
            };
            return Some(slot);
        }
        if (other.start == self.end) && (other.end > self.end) {
            //other is after self and touching it
            let slot = Slot {
                start: self.start,
                end: other.end,
            };
            return Some(slot);
        }

        None
    }
}

impl Slot {
    pub fn new(start: NaiveDateTime, end: NaiveDateTime) -> Slot {
        Slot { start, end }
    }

    pub fn span(&self) -> usize {
        slot_span(&self.start, &self.end)
    }

    pub fn conflicts_with(&self, other_slot: &Slot) -> bool {
        !((self.start < other_slot.start && self.end <= other_slot.start)
            || (self.start >= other_slot.end && self.end > other_slot.end))
    }

    pub fn contains_slot(&self, other: &Slot) -> bool {
        (other.start >= self.start) && (other.end <= self.end)
    }

    pub fn intersection(&self, other: &Slot) -> usize {
        let overlap = min(self.end, other.end) - max(self.start, other.start);
        overlap.num_hours().max(0) as usize
    }
}
