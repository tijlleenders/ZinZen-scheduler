pub mod impls;
pub mod utils;

use chrono::NaiveDateTime;
use serde::Deserialize;

// TODO 2023-04-26  | Slot rules as below:
// - A rule that slot.end must not be before slot.start
// - A suggestion to add rule to create 2 slots if slot.start is after slot.end

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Deserialize)]
pub struct Slot {
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
}

#[derive(PartialEq, Eq, Clone)]
pub struct SlotConflict {
    pub slot: Slot,
    pub num_conflicts: usize,
}