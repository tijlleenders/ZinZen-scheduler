use chrono::Duration;
use chrono::{NaiveDateTime, Timelike};
use serde::{Deserialize, Serialize};
use std::ops::{Add, Sub};

#[derive(Debug, Eq, PartialEq, Clone, Copy, Deserialize)]
pub struct Slot {
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
}

impl Sub for Slot {
    type Output = Vec<Slot>;

    fn sub(self, other: Self) -> Self::Output {
        let mut result = Vec::new();
        if (other.start < self.start) && (other.end <= self.start) {
            //other is completely before self
            result.push(self);
            return result;
        } else if (other.start >= self.end) && (other.end > self.end) {
            //other is completely after self
            result.push(self);
            return result;
        } else if (other.start == self.start) && (other.end < self.end) {
            //other starts same time as self and ends before self ends
            let slot = Slot {
                start: other.end,
                end: self.end,
            };
            result.push(slot);
            return result;
        } else if (other.start > self.start) && (other.end < self.end) {
            //other is inside self. starts after and ends before
            let slot1 = Slot {
                start: self.start,
                end: other.start,
            };
            let slot2 = Slot {
                start: other.end,
                end: self.end,
            };
            result.push(slot1);
            result.push(slot2);
            return result;
        } else if (other.start > self.start) && (other.end == self.end) {
            //other is inside self but end is same as self end
            let slot = Slot {
                start: self.start,
                end: other.start,
            };
            result.push(slot);
            return result;
        } else if (other.start <= self.start) && (other.end >= self.end) {
            //other engulfs self
            return result;
        } else if (other.start < self.start) && (other.end > self.start) && (other.end <= self.end)
        {
            //other starts before self and ends in-between self or when self ends
            let slot = Slot {
                start: other.end,
                end: self.end,
            };
            if !(slot.start == slot.end) {
                result.push(slot);
            }
            return result;
        } else {
            //other starts in-between self or when self starts and ends after self ends
            let slot = Slot {
                start: self.start,
                end: other.start,
            };
            if !(slot.start == slot.end) {
                result.push(slot);
            }
            return result;
        }
    }
}

impl Add for Slot {
    type Output = Slot;

    fn add(self, other: Self) -> Self::Output {
        if (other.start < self.start) && (other.end == self.start) {
            //other is before self and touching it
            let slot = Slot {
                start: other.start,
                end: self.end,
            };
            return slot;
        } else if (other.start == self.end) && (other.end > self.end) {
            //other is after self and touching it
            let slot = Slot {
                start: self.start,
                end: other.end,
            };
            return slot;
        } else {
            //for now any other scenario doesn't change self, we're using add for combining
            //slots that are adjacent to each other
            return self;
        }
    }
}

impl Slot {
    pub fn num_hours(&self) -> usize {
        (self.end - self.start).num_hours() as usize
    }

    pub fn conflicts_with(&self, other_slot: &Slot) -> bool {
        !((self.start < other_slot.start && self.end <= other_slot.start)
            || (self.start >= other_slot.end && self.end > other_slot.end))
    }
}
