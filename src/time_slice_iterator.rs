use crate::util::MyDurationRound;
use chrono::prelude::*;
use chrono::Duration;
use serde::de::{self, Visitor};
use serde::Deserialize;
use serde::*;
use std::fmt;

/// How often can a task repeat
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Repetition {
    DAILY,
    HOURLY,
    #[serde(rename = "weekly")]
    WEEKLY,
    WEEKDAYS,
    WEEKENDS,
    EveryXdays(usize),
    MONDAYS,
    TUESDAYS,
    WEDNESDAYS,
    THURSDAYS,
    FRIDAYS,
    SATURDAYS,
    SUNDAYS,
}

//How to implement serde deserialize: https://serde.rs/impl-deserialize.html
struct RepetitionVisitor;

impl<'de> Visitor<'de> for RepetitionVisitor {
    type Value = Repetition;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "a string of either daily, hourly, weekly, mondays etc."
        )
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let split = s.split(" ").collect::<Vec<&str>>();
        if split.len() == 3 {
            //e.g. 'every 5 days' == 3 words
            let number = split[1];
            match number.parse::<usize>() {
                Ok(num) => Ok(Repetition::EveryXdays(num)),
                Err(e) => Err(E::custom(format!("Error deserializing goal: {}", e))),
            }
        } else {
            match s {
                "daily" => Ok(Repetition::DAILY),
                "hourly" => Ok(Repetition::HOURLY),
                "weekdays" => Ok(Repetition::WEEKDAYS),
                "weekends" => Ok(Repetition::WEEKENDS),
                "mondays" => Ok(Repetition::MONDAYS),
                "tuesdays" => Ok(Repetition::TUESDAYS),
                "wednesdays" => Ok(Repetition::WEDNESDAYS),
                "thursdays" => Ok(Repetition::THURSDAYS),
                "fridays" => Ok(Repetition::FRIDAYS),
                "saturdays" => Ok(Repetition::SATURDAYS),
                "sundays" => Ok(Repetition::SUNDAYS),
                _ => Err(E::custom(format!("Error deserializing goal"))),
            }
        }
    }
}

impl<'de> Deserialize<'de> for Repetition {
    fn deserialize<D>(deserializer: D) -> Result<Repetition, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(RepetitionVisitor)
    }
}

#[derive(PartialEq)]
pub struct EveryXdays(pub usize);

impl fmt::Display for EveryXdays {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "every {} days", self.0)
    }
}

//The reason Display is being implemented for repetition
// is so that the string representation of MONDAYS-SUNDAYS matches the
//string representation of chrono::Weekday(). This makes it easy to match
//when generating date ranges, e.g. by doing self.start.weekday().to_string() == "Sat".
//see: https://docs.rs/chrono/latest/src/chrono/weekday.rs.html#141
impl fmt::Display for Repetition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            Repetition::DAILY => "DAILY",
            Repetition::HOURLY => "HOURLY",
            Repetition::WEEKLY => "WEEKLY",
            Repetition::WEEKDAYS => "WEEKDAYS",
            Repetition::WEEKENDS => "WEEKENDS",
            Repetition::EveryXdays(usize) => "EveryXdays",
            Repetition::MONDAYS => "Mon",
            Repetition::TUESDAYS => "Tue",
            Repetition::WEDNESDAYS => "Wed",
            Repetition::THURSDAYS => "Thu",
            Repetition::FRIDAYS => "Fri",
            Repetition::SATURDAYS => "Sat",
            Repetition::SUNDAYS => "Sun",
        })
    }
}

/// An iterator that returns (NaiveDateTime,NaiveDateTime) time slices based on a specified repetition,
/// within specified bounds.
/// e.g. all MONDAYS between 1st September 2022 and 30th September 2022.
/// e.g. all DAYS between 1st September 2022 and 30th September 2022.
/// e.g. all HOURS between 1st September 2022 and 30th September 2022.
pub(crate) struct TimeSliceIterator {
    pub(crate) start: NaiveDateTime,
    pub(crate) end: NaiveDateTime,
    pub(crate) repetition: Repetition,
}

impl Iterator for TimeSliceIterator {
    type Item = (NaiveDateTime, NaiveDateTime);
    fn next(&mut self) -> Option<Self::Item> {
        match self.repetition {
            Repetition::DAILY => {
                if self.start < self.end {
                    let start = self.start;
                    let mut end = self.start + Duration::days(1);
                    if end > self.end {
                        end = self.end;
                    } else {
                        end = end.duration_round(Duration::days(1)).ok()?;
                    }
                    self.start = end;
                    Some((start, end))
                } else {
                    None
                }
            }
            Repetition::HOURLY => {
                if self.start < self.end {
                    let start = self.start;
                    let mut end = self.start + Duration::hours(1);
                    if end > self.end {
                        end = self.end;
                    } else {
                        end = end.duration_round(Duration::hours(1)).ok()?;
                    }
                    self.start = end;
                    Some((start, end))
                } else {
                    None
                }
            }
            Repetition::WEEKLY => {
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
                return Some((start, end));
            }
            Repetition::WEEKDAYS => {
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
                    return Some((start, end));
                }
                return None;
            }
            Repetition::WEEKENDS => {
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
                    return Some((start, end));
                }
                return None;
            }
            Repetition::EveryXdays(days) => {
                if self.start < self.end {
                    let start = self.start;
                    let mut end = self.start + Duration::days(1);
                    if end > self.end {
                        end = self.end;
                    } else {
                        end = end.duration_round(Duration::days(1)).ok()?;
                    }
                    self.start = end + Duration::days((days - 1) as i64);
                    Some((start, end))
                } else {
                    None
                }
            }
            _ => {
                if self.start >= self.end {
                    return None;
                }
                while self.start.weekday().to_string() != self.repetition.to_string()
                    && self.start < self.end
                {
                    self.start += Duration::days(1);
                }
                if self.start.weekday().to_string() == self.repetition.to_string() {
                    let start = self.start;
                    let end = self.start + Duration::days(1);
                    self.start = end;
                    return Some((start, end));
                }
                return None;
            }
        }
    }
}
