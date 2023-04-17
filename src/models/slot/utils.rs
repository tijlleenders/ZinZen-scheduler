use super::{Slot, TimeSlotsIterator};
use crate::models::{goal::TimeFilter, repetition::Repetition};
use chrono::{Datelike, Days, NaiveDateTime, Timelike, Weekday, Duration};
use std::{
    cmp::{max, min},
    fmt::Display,
    ops::{Add, Sub},
};

impl TimeSlotsIterator {
    pub fn new(
        start: NaiveDateTime,
        end: NaiveDateTime,
        repetition: Option<Repetition>,
        filters: Option<TimeFilter>,
    ) -> TimeSlotsIterator {
        let mut result = TimeSlotsIterator {
            timeline: vec![Slot { start, end }],
            repetition,
            filters,
            current_start_position: start, //override after applying filters
        };
        result.apply_filters();
        result.current_start_position = result.timeline[0].start;
        result
    }
}

impl Display for Slot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Slot - start: {} - end: {}", self.start, self.end)
    }
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

pub fn get_start_of_repeat_step(
    current_date_time: &NaiveDateTime,
    repeat: Repetition,
) -> NaiveDateTime {
    let mut result = *current_date_time;
    match repeat {
        Repetition::DAILY(_) => result
            .checked_add_days(Days::new(1))
            .unwrap()
            .with_hour(0)
            .unwrap()
            .with_minute(0)
            .unwrap()
            .with_second(0)
            .unwrap(),
        Repetition::HOURLY => result.checked_add_signed(Duration::hours(1)).unwrap(),
        Repetition::Weekly(_) => {
            if result.weekday() == Weekday::Mon {
                return result
                    .checked_add_days(Days::new(7))
                    .unwrap()
                    .with_hour(0)
                    .unwrap()
                    .with_minute(0)
                    .unwrap()
                    .with_second(0)
                    .unwrap();
            }
            for _days_to_add in 1..=6 {
                result = result.checked_add_days(Days::new(1)).unwrap();
                if result.weekday() == Weekday::Mon {
                    return result
                        .with_hour(0)
                        .unwrap()
                        .with_minute(0)
                        .unwrap()
                        .with_second(0)
                        .unwrap();
                }
            }
            panic!("shouldn't reach")
        }
        Repetition::WEEKDAYS => {
            match result.weekday() {
                Weekday::Sat => result
                    .checked_add_days(Days::new(2))
                    .unwrap()
                    .with_hour(0)
                    .unwrap()
                    .with_minute(0)
                    .unwrap()
                    .with_second(0)
                    .unwrap(),
                Weekday::Sun => result
                    .checked_add_days(Days::new(1))
                    .unwrap()
                    .with_hour(0)
                    .unwrap()
                    .with_minute(0)
                    .unwrap()
                    .with_second(0)
                    .unwrap(),
                _ => result
                    .checked_add_days(Days::new(1))
                    .unwrap()
                    .with_hour(0)
                    .unwrap()
                    .with_minute(0)
                    .unwrap()
                    .with_second(0)
                    .unwrap(),
            }

            // if result.weekday() == Weekday::Sat {
            //     return result
            //         .checked_add_days(Days::new(2))
            //         .unwrap()
            //         .with_hour(0)
            //         .unwrap()
            //         .with_minute(0)
            //         .unwrap()
            //         .with_second(0)
            //         .unwrap();
            // } else if result.weekday() == Weekday::Sun {
            //     return result
            //         .checked_add_days(Days::new(1))
            //         .unwrap()
            //         .with_hour(0)
            //         .unwrap()
            //         .with_minute(0)
            //         .unwrap()
            //         .with_second(0)
            //         .unwrap();
            // } else {
            //     return result
            //         .checked_add_days(Days::new(1))
            //         .unwrap()
            //         .with_hour(0)
            //         .unwrap()
            //         .with_minute(0)
            //         .unwrap()
            //         .with_second(0)
            //         .unwrap();
            // }
        }
        Repetition::WEEKENDS => {
            if result.weekday() == Weekday::Sat {
                return result
                    .checked_add_days(Days::new(7))
                    .unwrap()
                    .with_hour(0)
                    .unwrap()
                    .with_minute(0)
                    .unwrap()
                    .with_second(0)
                    .unwrap();
            }
            if result.weekday() == Weekday::Sun {
                return result
                    .checked_add_days(Days::new(6))
                    .unwrap()
                    .with_hour(0)
                    .unwrap()
                    .with_minute(0)
                    .unwrap()
                    .with_second(0)
                    .unwrap();
            }
            for _days_to_add in 1..=6 {
                result = result.checked_add_days(Days::new(1)).unwrap();
                if result.weekday() == Weekday::Sat {
                    return result
                        .with_hour(0)
                        .unwrap()
                        .with_minute(0)
                        .unwrap()
                        .with_second(0)
                        .unwrap();
                }
            }
            panic!("Shouldn't reach this");
        }
        Repetition::EveryXdays(day_interval) => result
            .checked_add_days(Days::new(day_interval.try_into().unwrap()))
            .unwrap()
            .with_hour(0)
            .unwrap()
            .with_minute(0)
            .unwrap()
            .with_second(0)
            .unwrap(),
        Repetition::EveryXhours(hour_interval) => result
            .checked_add_signed(Duration::hours(hour_interval.try_into().unwrap()))
            .unwrap(),
        Repetition::MONDAYS => {
            if result.weekday() == Weekday::Mon {
                return result
                    .checked_add_days(Days::new(7))
                    .unwrap()
                    .with_hour(0)
                    .unwrap()
                    .with_minute(0)
                    .unwrap()
                    .with_second(0)
                    .unwrap();
            }
            for _days_to_add in 1..=6 {
                result = result.checked_add_days(Days::new(1)).unwrap();
                if result.weekday() == Weekday::Mon {
                    return result
                        .with_hour(0)
                        .unwrap()
                        .with_minute(0)
                        .unwrap()
                        .with_second(0)
                        .unwrap();
                }
            }
            panic!("Shouldn't reach this");
        }
        Repetition::TUESDAYS => {
            if result.weekday() == Weekday::Tue {
                return result
                    .checked_add_days(Days::new(7))
                    .unwrap()
                    .with_hour(0)
                    .unwrap()
                    .with_minute(0)
                    .unwrap()
                    .with_second(0)
                    .unwrap();
            }
            for _days_to_add in 1..=6 {
                result = result.checked_add_days(Days::new(1)).unwrap();
                if result.weekday() == Weekday::Tue {
                    return result
                        .with_hour(0)
                        .unwrap()
                        .with_minute(0)
                        .unwrap()
                        .with_second(0)
                        .unwrap();
                }
            }
            panic!("Shouldn't reach this");
        }
        Repetition::WEDNESDAYS => {
            if result.weekday() == Weekday::Wed {
                return result
                    .checked_add_days(Days::new(7))
                    .unwrap()
                    .with_hour(0)
                    .unwrap()
                    .with_minute(0)
                    .unwrap()
                    .with_second(0)
                    .unwrap();
            }
            for _days_to_add in 1..=6 {
                result = result.checked_add_days(Days::new(1)).unwrap();
                if result.weekday() == Weekday::Wed {
                    return result
                        .with_hour(0)
                        .unwrap()
                        .with_minute(0)
                        .unwrap()
                        .with_second(0)
                        .unwrap();
                }
            }
            panic!("Shouldn't reach this");
        }
        Repetition::THURSDAYS => {
            if result.weekday() == Weekday::Thu {
                return result
                    .checked_add_days(Days::new(7))
                    .unwrap()
                    .with_hour(0)
                    .unwrap()
                    .with_minute(0)
                    .unwrap()
                    .with_second(0)
                    .unwrap();
            }
            for _days_to_add in 1..=6 {
                result = result.checked_add_days(Days::new(1)).unwrap();
                if result.weekday() == Weekday::Thu {
                    return result
                        .with_hour(0)
                        .unwrap()
                        .with_minute(0)
                        .unwrap()
                        .with_second(0)
                        .unwrap();
                }
            }
            panic!("Shouldn't reach this");
        }
        Repetition::FRIDAYS => {
            if result.weekday() == Weekday::Fri {
                return result
                    .checked_add_days(Days::new(7))
                    .unwrap()
                    .with_hour(0)
                    .unwrap()
                    .with_minute(0)
                    .unwrap()
                    .with_second(0)
                    .unwrap();
            }
            for _days_to_add in 1..=6 {
                result = result.checked_add_days(Days::new(1)).unwrap();
                if result.weekday() == Weekday::Fri {
                    return result
                        .with_hour(0)
                        .unwrap()
                        .with_minute(0)
                        .unwrap()
                        .with_second(0)
                        .unwrap();
                }
            }
            panic!("Shouldn't reach this");
        }
        Repetition::SATURDAYS => {
            if result.weekday() == Weekday::Sat {
                return result
                    .checked_add_days(Days::new(7))
                    .unwrap()
                    .with_hour(0)
                    .unwrap()
                    .with_minute(0)
                    .unwrap()
                    .with_second(0)
                    .unwrap();
            }
            for _days_to_add in 1..=6 {
                result = result.checked_add_days(Days::new(1)).unwrap();
                if result.weekday() == Weekday::Sat {
                    return result
                        .with_hour(0)
                        .unwrap()
                        .with_minute(0)
                        .unwrap()
                        .with_second(0)
                        .unwrap();
                }
            }
            panic!("Shouldn't reach this");
        }
        Repetition::SUNDAYS => {
            if result.weekday() == Weekday::Sun {
                return result
                    .checked_add_days(Days::new(7))
                    .unwrap()
                    .with_hour(0)
                    .unwrap()
                    .with_minute(0)
                    .unwrap()
                    .with_second(0)
                    .unwrap();
            }
            for _days_to_add in 1..=6 {
                result = result.checked_add_days(Days::new(1)).unwrap();
                if result.weekday() == Weekday::Sun {
                    return result
                        .with_hour(0)
                        .unwrap()
                        .with_minute(0)
                        .unwrap()
                        .with_second(0)
                        .unwrap();
                }
            }
            panic!("Shouldn't reach this");
        }
        Repetition::FlexDaily(_, _) => todo!(),
        Repetition::FlexWeekly(_, _) => todo!(),
    }
}
