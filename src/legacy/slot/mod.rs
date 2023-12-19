pub mod iterator;

use super::date::deserialize_normalized_date;
use chrono::{Datelike, NaiveDateTime, Timelike};
use serde::Deserialize;
use std::fmt::{self, Debug, Display};

// TODO 2023-04-26  | Slot rules as below:
// - A rule that slot.end must not be before slot.start
// - A suggestion to add rule to create 2 slots if slot.start is after slot.end

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Deserialize)]
pub struct Slot {
    #[serde(deserialize_with = "deserialize_normalized_date")]
    pub start: NaiveDateTime,
    #[serde(deserialize_with = "deserialize_normalized_date")]
    pub end: NaiveDateTime,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct SlotConflict {
    pub slot: Slot,
    pub num_conflicts: usize,
}

impl Debug for Slot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let start = self.start;
        let end = self.end;
        write!(
            f,
            "Slot {{ \n\tstart:\t {:02}-{:02}-{:02} {:02},\n\t end:\t {:02}-{:02}-{:02} {:02}, \n}}",
            start.year(),
            start.month(),
            start.day(),
            start.hour(),
            end.year(),
            end.month(),
            end.day(),
            end.hour(),
        )
    }
}

impl Display for Slot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let start = self.start;
        let end = self.end;
        write!(
            f,
            "Slot {{ start: {:02}-{:02}-{:02} {:02} - end: {:02}-{:02}-{:02} {:02} }}",
            start.year(),
            start.month(),
            start.day(),
            start.hour(),
            end.year(),
            end.month(),
            end.day(),
            end.hour(),
        )
    }
}
