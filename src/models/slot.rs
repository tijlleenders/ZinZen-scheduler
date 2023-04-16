use chrono::{Days, NaiveDateTime, Timelike};
use serde::Deserialize;
use std::{
    cmp::{max, min},
    fmt::Display,
    ops::{Add, Sub},
};

#[derive(Debug, Eq, PartialEq, Clone, Copy, Deserialize)]
pub struct Slot {
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
}

impl Display for Slot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Slot - start: {} - end: {}", self.start, self.end)
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct SlotConflict {
    pub slot: Slot,
    pub num_conflicts: usize,
}

impl Sub for Slot {
    type Output = Vec<Slot>;

    fn sub(self, other: Self) -> Self::Output {
        let mut result = Vec::new();
        if (other.start < self.start) && (other.end <= self.start)
            || (other.start >= self.end) && (other.end > self.end)
        {
            //other is completely before self
            result.push(self);
            result
        } else if (other.start == self.start) && (other.end < self.end) {
            //other starts same time as self and ends before self ends
            let slot = Slot {
                start: other.end,
                end: self.end,
            };
            result.push(slot);
            result
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
            result
        } else if (other.start > self.start) && (other.end == self.end) {
            //other is inside self but end is same as self end
            let slot = Slot {
                start: self.start,
                end: other.start,
            };
            result.push(slot);
            result
        } else if (other.start <= self.start) && (other.end >= self.end) {
            //other engulfs self
            result
        } else if (other.start < self.start) && (other.end > self.start) && (other.end <= self.end)
        {
            //other starts before self and ends in-between self or when self ends
            let slot = Slot {
                start: other.end,
                end: self.end,
            };
            if slot.start != slot.end {
                result.push(slot);
            }
            result
        } else {
            //other starts in-between self or when self starts and ends after self ends
            let slot = Slot {
                start: self.start,
                end: other.start,
            };
            if slot.start != slot.end {
                result.push(slot);
            }
            result
        }
    }
}

impl Add for Slot {
    type Output = Slot;

    fn add(self, other: Self) -> Self::Output {
        if (other.start < self.start) && (other.end == self.start) {
            //other is before self and touching it
            Slot {
                start: other.start,
                end: self.end,
            }
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
    pub fn contains_hour_slot(&self, other: &Slot) -> bool {
        (other.start >= self.start) && (other.end <= self.end)
    }
    pub fn get_1h_slots(&self) -> Vec<Slot> {
        let mut result = vec![];
        for hour in 0..self.num_hours() {
            result.push(Slot {
                start: self.start.add(chrono::Duration::hours(hour as i64)),
                end: self.start.add(chrono::Duration::hours((hour + 1) as i64)),
            })
        }
        result
    }
    pub fn divide_in_days(&self) -> Vec<Slot> {
        let mut result = vec![];
        let mut start_slider = self.start;
        while start_slider.lt(&self.end) {
            if start_slider.date().eq(&self.end.date()) {
                result.push(Slot {
                    start: start_slider,
                    end: self.end,
                });
                start_slider = start_slider
                    .with_hour(0)
                    .unwrap()
                    .checked_add_days(Days::new(1))
                    .unwrap();
                continue;
            } else {
                result.push(Slot {
                    start: start_slider,
                    end: start_slider
                        .with_hour(0)
                        .unwrap()
                        .checked_add_days(Days::new(1))
                        .unwrap(),
                });
                start_slider = start_slider
                    .with_hour(0)
                    .unwrap()
                    .checked_add_days(Days::new(1))
                    .unwrap();
            }
        }
        result
    }

    pub fn is_intersect(&self, other: &Slot) -> bool {
        let overlap = min(self.end, other.end) - max(self.start, other.start);
        overlap.num_hours() > 0
    }
}
