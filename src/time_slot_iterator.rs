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
    pub(crate) start: NaiveDateTime,
    pub(crate) end: NaiveDateTime,
    pub(crate) repetition: Option<Repetition>,
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
                    } else {
                        end = end.duration_round(Duration::days(1)).ok()?;
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
                return Some(Slot { start, end });
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
                return None;
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
                return None;
            }
            Some(Repetition::EveryXdays(days)) => {
                if self.start < self.end {
                    let start = self.start;
                    let mut end = self.start + Duration::days(1);
                    if end > self.end {
                        end = self.end;
                    } else {
                        end = end.duration_round(Duration::days(1)).ok()?;
                    }
                    self.start = end + Duration::days((days - 1) as i64);
                    Some(Slot { start, end })
                } else {
                    None
                }
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
                return None;
            }
        }
    }
}
