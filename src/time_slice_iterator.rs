use crate::util::MyDurationRound;
use chrono::prelude::*;
use chrono::Duration;
use std::fmt;
use serde::Deserialize;

/// How often can a task repeat
#[derive(Deserialize, Debug, Copy, Clone)]
pub enum Repetition {
	#[serde(rename = "daily")]
	DAILY,
    HOURLY,
    MONDAYS,
    TUESDAYS,
    WEDNESDAYS,
    THURSDAYS,
    FRIDAYS,
    SATURDAYS,
    SUNDAYS,
}

//the reason Display is being implemented for repetition
// is so that the string representation of MONDAYS-SUNDAYS matches the 
//string representation of chrono::Weekday(). This makes it easy to match 
//when generating date ranges.
//see: https://docs.rs/chrono/latest/src/chrono/weekday.rs.html#141
impl fmt::Display for Repetition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            Repetition::DAILY => "DAILY",
            Repetition::HOURLY => "HOURLY",
            Repetition::MONDAYS => "Mon",
            Repetition::TUESDAYS => "Tue",
            Repetition::WEDNESDAYS => "Wed",
            Repetition::THURSDAYS => "Thur",
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
           },
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
           },
           _ => {
                while self.start.weekday().to_string() != self.repetition.to_string() && (self.start < self.end) {
                    self.start = self.start + Duration::days(1);
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
