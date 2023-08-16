pub mod impls;
pub mod iterator;

use chrono::{Datelike, NaiveDateTime, Timelike};
use serde::Deserialize;
use std::fmt::{self, Debug, Display};

// TODO 2023-04-26  | Slot rules as below:
// - A rule that slot.end must not be before slot.start
// - A suggestion to add rule to create 2 slots if slot.start is after slot.end

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Deserialize)]
pub struct Slot {
    pub start: NaiveDateTime,
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

#[cfg(test)]
mod tests {
    use chrono::Duration;

    use super::Slot;

    #[test]
    fn diplay_slot() {
        let slot = Slot::mock(Duration::hours(12), 2023, 6, 1, 9, 0);
        let debug_output = format!("{}", slot);
        assert_eq!(
            debug_output,
            "Slot { start: 2023-06-01 09 - end: 2023-06-01 21 }"
        );
    }

    #[test]
    fn debug_slot() {
        let slot = Slot::mock(Duration::hours(12), 2023, 6, 1, 9, 0);
        let debug_output = format!("{:?}", slot);
        let expected =
            "Slot { \n\tstart:\t 2023-06-01 09,\n\t end:\t 2023-06-01 21, \n}".to_string();

        assert_eq!(debug_output, expected);
    }

    #[test]
    fn debug_slot_formatted() {
        let slot = Slot::mock(Duration::hours(12), 2023, 6, 1, 9, 0);
        let debug_output = format!("{:?}", slot);
        let debug_output_fomratted = format!("{:#?}", slot);

        let expected =
            "Slot { \n\tstart:\t 2023-06-01 09,\n\t end:\t 2023-06-01 21, \n}".to_string();

        assert_eq!(debug_output, expected);
        assert_eq!(debug_output_fomratted, expected);
    }
}
