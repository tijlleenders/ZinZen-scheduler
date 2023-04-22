use super::Slot;
use chrono::{Days, Timelike};
use std::{
    cmp::{max, min},
    fmt::Display,
    ops::{Add, Sub},
};

impl Display for Slot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Slot - start: {} - end: {}", self.start, self.end)
    }
}

impl Sub for Slot {
    type Output = Vec<Slot>;

    fn sub(self, other: Self) -> Self::Output {
        dbg!(&self);
        dbg!(&other);

        let mut result = Vec::new();
        if (other.start < self.start) && (other.end <= self.start)
            || (other.start >= self.end) && (other.end > self.end)
        {
            //other slot before self OR other slot after self
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

            // TODO 2023-04-22 | fix this which seems unlogical or unnecessary

            result
        } else if (other.start < self.start) && (other.end > self.start) && (other.end <= self.end)
        {
            //other starts before self and ends in-between self or when self ends

            // TODO 2023-04-22 | fix this which should have 2 slots:
            // slot1 [start: other.start, end: self.start]
            // slot2 [start: other.end, end: slef.end] IF other.end != self.end

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

/// Add a slot if they are consequent. If not, will return None
impl Add for Slot {
    type Output = Option<Slot>;

    fn add(self, other: Self) -> Self::Output {
        dbg!(&self);
        dbg!(&other);

        if (other.start < self.start) && (other.end == self.start) {
            //other is before self and touching it
            let slot = Slot {
                start: other.start,
                end: self.end,
            };
            dbg!(&slot);
            return Some(slot);
        }
        if (other.start == self.end) && (other.end > self.end) {
            //other is after self and touching it
            let slot = Slot {
                start: self.start,
                end: other.end,
            };
            dbg!(&slot);
            return Some(slot);
        }

        None
    }
}

impl Slot {
    pub fn calc_duration_in_hours(&self) -> usize {
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
        let duration = self.calc_duration_in_hours();
        dbg!(duration);

        for hour in 0..duration {
            dbg!(hour);
            result.push(Slot {
                start: self.start.add(chrono::Duration::hours(hour as i64)),
                end: self.start.add(chrono::Duration::hours((hour + 1) as i64)),
            });

            dbg!(&result);
        }
        dbg!(&result);
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
        dbg!(start_slider);
        dbg!(self.end);

        while start_slider.lt(&self.end) {
            if start_slider.date().eq(&self.end.date()) {
                result.push(Slot {
                    start: start_slider,
                    end: self.end,
                });
                dbg!(&result);

                start_slider = start_slider
                    .with_hour(0)
                    .unwrap()
                    .checked_add_days(Days::new(1))
                    .unwrap();
                dbg!(start_slider);
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
                dbg!(&result);

                start_slider = start_slider
                    .with_hour(0)
                    .unwrap()
                    .checked_add_days(Days::new(1))
                    .unwrap();
                dbg!(start_slider);
            }
        }
        dbg!(&result);
        result
    }

    pub fn is_intersect_with_slot(&self, other: &Slot) -> bool {
        let overlap = min(self.end, other.end) - max(self.start, other.start);
        overlap.num_hours() > 0
    }
}
