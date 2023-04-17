use chrono::NaiveDateTime;
use serde::Deserialize;

use super::{repetition::Repetition, goal::TimeFilter};

pub mod filter;
pub mod iterator;
pub mod utils;

// derive Debug for TimeSlotsIterator
#[derive(Debug)]
/// An iterator that returns slots that conform to a repetition,
/// with optional filters (after/before/Mondays/Weekdays/Weekends/Bank holidays...)
/// e.g. iterate over all MONDAYS between 1st September 2022 and 30th September 2022.
/// e.g. iterate over all DAYS between 1st September 2022 and 30th September 2022.
/// e.g. iterate over all HOURS between 1st September 2022 and 30th September 2022.
/// e.g. iterate over all 'every two hours' slots between times 10-21 for time period X-Y
pub(crate) struct TimeSlotsIterator {
    timeline: Vec<Slot>,
    repetition: Option<Repetition>,
    filters: Option<TimeFilter>,
    current_start_position: NaiveDateTime,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Deserialize)]
pub struct Slot {
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
}

#[derive(PartialEq, Eq, Clone)]
pub struct SlotConflict {
    pub slot: Slot,
    pub num_conflicts: usize,
}
