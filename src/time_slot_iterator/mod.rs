use self::time_filter::TimeFilter;
use crate::repetition::Repetition;
use crate::slot::Slot;
use chrono::prelude::*;
use chrono::Duration;

pub mod time_filter;

/// An iterator that returns slots that conform to a repetition,
/// with optional filters (after/before/Mondays/Weekdays/Weekends/Bank holidays...)
/// e.g. iterate over all MONDAYS between 1st September 2022 and 30th September 2022.
/// e.g. iterate over all DAYS between 1st September 2022 and 30th September 2022.
/// e.g. iterate over all HOURS between 1st September 2022 and 30th September 2022.
/// e.g. iterate over all 'every two hours' slots between times 10-21 for time period X-Y
pub(crate) struct TimeSlotIterator {
    timeline: Vec<Slot>,
    repetition: Option<Repetition>,
    filters: Vec<TimeFilter>,
}

impl TimeSlotIterator {
    pub fn new(
        start: NaiveDateTime,
        end: NaiveDateTime,
        repetition: Option<Repetition>,
        filters: Vec<TimeFilter>,
    ) -> TimeSlotIterator {
        let result = TimeSlotIterator {
            timeline: vec![Slot {
                start: start,
                end: end,
            }],
            repetition,
            filters,
        };
        result.apply_filters();
        result
    }

    fn apply_filters(&self) {
        todo!()
    }
}

impl Iterator for TimeSlotIterator {
    type Item = Slot;
    fn next(&mut self) -> Option<Self::Item> {
        match self.repetition {
            None => {
                if self.start < self.end {
                    let start = self.start;
                    let end = self.end;
                    self.start = end;
                    Some(Slot { start, end })
                } else {
                    None
                }
            }
            Some(Repetition::DAILY(_)) => {
                if self.start < self.end {
                    let start = self.start;
                    let mut end = self.start + Duration::days(1);
                    if end > self.end {
                        end = self.end;
                    }
                    self.start = end;
                    Some(Slot { start, end })
                } else {
                    None
                }
            }
            Some(Repetition::HOURLY) => {
                if self.start < self.end {
                    let start = self.start;
                    let mut end = self.start + Duration::hours(1);
                    if end > self.end {
                        end = self.end;
                    } else {
                        end = end.duration_round(Duration::hours(1)).ok()?;
                    }
                    self.start = end;
                    Some(Slot { start, end })
                } else {
                    None
                }
            }
            Some(Repetition::Weekly(_)) => {
                if self.start >= self.end {
                    return None;
                }
                let start = self.start;
                let mut end = self.start;
                while end.weekday().to_string() != "Sun" && end <= self.end {
                    // while end <= self.end {
                    end += Duration::days(1);
                }
                if end >= self.end {
                    end = self.end;
                } else {
                    end += Duration::days(1);
                }
                self.start = end;
                Some(Slot { start, end })
            }
            Some(Repetition::WEEKDAYS) => {
                if self.start >= self.end {
                    return None;
                }
                while self.start.weekday().to_string() == "Sat"
                    || self.start.weekday().to_string() == "Sun" && self.start < self.end
                {
                    self.start += Duration::days(1);
                }
                if self.start.weekday().to_string() != "Sat"
                    && self.start.weekday().to_string() != "Sun"
                    && self.start < self.end
                {
                    let start = self.start;
                    let end = self.start + Duration::days(1);
                    self.start = end;
                    return Some(Slot { start, end });
                }
                None
            }
            Some(Repetition::WEEKENDS) => {
                if self.start >= self.end {
                    return None;
                }
                while self.start.weekday().to_string() != "Sat"
                    && self.start.weekday().to_string() != "Sun"
                    && self.start < self.end
                {
                    self.start += Duration::days(1);
                }
                if (self.start.weekday().to_string() == "Sat"
                    || self.start.weekday().to_string() == "Sun")
                    && self.start < self.end
                {
                    let start = self.start;
                    let end = self.start + Duration::days(1);
                    self.start = end;
                    return Some(Slot { start, end });
                }
                None
            }
            Some(Repetition::EveryXdays(days)) => {
                if self.start >= self.end {
                    return None;
                }
                let start = self.start;
                let mut end = self.start + Duration::days(1);
                if end > self.end {
                    end = self.end;
                }
                self.start = end + Duration::days((days - 1) as i64);
                Some(Slot { start, end })
            }
            Some(Repetition::EveryXhours(hours)) => {
                if self.start >= self.end {
                    return None;
                }
                // while !hour_is_within_bounds(
                //     self.after_time,
                //     self.before_time,
                //     self.start.hour() as usize,
                // ) {
                //     self.start += Duration::hours(1);
                //     if self.start >= self.end {
                //         return None;
                //     }
                // }
                //we are now at an hour that is within the time bounds of the task
                let start = self.start;
                let end = self.start + Duration::hours(1);
                self.start = end + Duration::hours((hours - 1) as i64);
                Some(Slot { start, end })
            }
            _ => {
                //it's a day of the week e.g. MONDAYS or TUESDAYS
                if self.start >= self.end {
                    return None;
                }
                while self.start.weekday().to_string() != self.repetition.unwrap().to_string()
                    && self.start < self.end
                {
                    self.start += Duration::days(1);
                }
                if self.start.weekday().to_string() == self.repetition.unwrap().to_string() {
                    let start = self.start;
                    let end = self.start + Duration::days(1);
                    self.start = end;
                    return Some(Slot { start, end });
                }
                None
            }
        }
    }
}

fn hour_is_within_bounds(after_time: usize, before_time: usize, hour: usize) -> bool {
    if before_time < after_time {
        hour < before_time || hour >= after_time
    } else {
        hour >= after_time && hour < before_time
    }
}

fn skip_to_after_time(mut original_time: NaiveDateTime, after_time: usize) -> NaiveDateTime {
    while original_time.hour() < after_time as u32 {
        original_time += Duration::hours(1);
    }
    original_time
}
