pub mod impls;
pub mod utils;

use chrono::NaiveDateTime;
use serde::Deserialize;

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
