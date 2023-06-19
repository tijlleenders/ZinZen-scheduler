use super::Slot;
use chrono::{Days, NaiveDateTime, Timelike};
use std::{
    cmp::{max, min},
    ops::{Add, Sub},
};

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

    pub fn duration_as_hours(&self) -> usize {
        (self.end - self.start).num_hours() as usize
    }

    pub fn is_conflicts_with(&self, other_slot: &Slot) -> bool {
        !((self.start < other_slot.start && self.end <= other_slot.start)
            || (self.start >= other_slot.end && self.end > other_slot.end))
    }

    pub fn is_contains_slot(&self, other: &Slot) -> bool {
        (other.start >= self.start) && (other.end <= self.end)
    }

    /// Divide a Slot into list of slots with 1 hour interval
    /// If you pass a Slot for 5 hours, then it will splitted
    ///  into 5 slots with 1 hour interval:
    /// ```markdown
    /// Param:
    ///     Slot [ 2022-01-01 00:00:00 - 2022-01-01 05:00:00 ]
    ///     Duration: 5 hours
    ///
    /// Returns:
    ///     Slot [ 2022-01-01 00:00:00 - 2022-01-01 01:00:00 ]
    ///     Slot [ 2022-01-01 01:00:00 - 2022-01-01 02:00:00 ]
    ///     Slot [ 2022-01-01 02:00:00 - 2022-01-01 03:00:00 ]
    ///     Slot [ 2022-01-01 03:00:00 - 2022-01-01 04:00:00 ]
    ///     Slot [ 2022-01-01 04:00:00 - 2022-01-01 05:00:00 ]
    ///
    /// ```
    pub fn divide_into_1h_slots(&self) -> Vec<Slot> {
        let mut result = vec![];
        let duration = self.duration_as_hours();

        for hour in 0..duration {
            result.push(Slot {
                start: self.start.add(chrono::Duration::hours(hour as i64)),
                end: self.start.add(chrono::Duration::hours((hour + 1) as i64)),
            });
        }
        result
    }

    /// Divide a Slot into list of slots with 1 day interval
    /// If you pass a Slot for a week, then it will splitted
    ///  into 7 slots for each day of the week:
    /// ```markdown
    /// Param:
    ///     Slot [ 2022-01-01 00:00:00 - 2022-01-08 00:00:00 ]
    ///
    /// Returns:
    ///     Slot [ 2022-01-01 00:00:00 - 2022-01-02 00:00:00 ]
    ///     Slot [ 2022-01-02 00:00:00 - 2022-01-03 00:00:00 ]
    ///     Slot [ 2022-01-03 00:00:00 - 2022-01-04 00:00:00 ]
    ///     Slot [ 2022-01-04 00:00:00 - 2022-01-05 00:00:00 ]
    ///     Slot [ 2022-01-05 00:00:00 - 2022-01-06 00:00:00 ]
    ///     Slot [ 2022-01-06 00:00:00 - 2022-01-07 00:00:00 ]
    ///     Slot [ 2022-01-07 00:00:00 - 2022-01-08 00:00:00 ]
    /// ```
    pub fn divide_into_days(&self) -> Vec<Slot> {
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

    pub fn is_intersect_with_slot(&self, other: &Slot) -> bool {
        let overlap = min(self.end, other.end) - max(self.start, other.start);
        overlap.num_hours() > 0
    }
}
