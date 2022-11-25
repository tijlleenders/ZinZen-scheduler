use crate::repetition;
use crate::repetition::Repetition;
use crate::slot::Slot;
use crate::util::MyDurationRound;
use chrono::prelude::*;
use chrono::Duration;

/// An iterator that returns slots that conform to a repetition,
/// within specified bounds.
/// e.g. all MONDAYS between 1st September 2022 and 30th September 2022.
/// e.g. all DAYS between 1st September 2022 and 30th September 2022.
/// e.g. all HOURS between 1st September 2022 and 30th September 2022.
pub(crate) struct TimeSlotIterator {
    start: NaiveDateTime,
    end: NaiveDateTime,
    repetition: Option<Repetition>,
    after_time: usize,
    before_time: usize,
}

impl TimeSlotIterator {
    pub fn new(
        start: NaiveDateTime,
        end: NaiveDateTime,
        repetition: Option<Repetition>,
        after_time: usize,
        before_time: usize,
    ) -> TimeSlotIterator {
        TimeSlotIterator {
            start: start + Duration::hours(after_time as i64),
            end,
            repetition,
            after_time,
            before_time,
        }
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
            Some(Repetition::WEEKLY(_)) => {
                if self.start >= self.end {
                    return None;
                }
                let start = self.start;
                let mut end = self.start;
                while end.weekday().to_string() != "Sun" && end <= self.end {
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
                while !hour_is_within_bounds(
                    self.after_time,
                    self.before_time,
                    self.start.hour() as usize,
                ) {
                    self.start += Duration::hours(1);
                    if self.start >= self.end {
                        return None;
                    }
                }
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
