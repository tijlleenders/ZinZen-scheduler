pub mod iterator;

use super::slot::Slot;
use chrono::NaiveDateTime;
use serde::Deserialize;
use std::collections::BTreeSet;

pub type TimelineSlotsType = BTreeSet<Slot>;

//TODO 2023-04-21
// - Implement Display for Timeline
// - If possible to develop divide timeline into hours, days, weeks, months, years

/// Timeline controlling passing list of slots in the system
/// Provide 2 public functionalities:
/// 1. remove timeline which is a list of slots
/// 2. get next slot of timeline
#[derive(Debug, Deserialize, PartialEq, Clone, Default)]
pub struct Timeline {
    pub slots: TimelineSlotsType,
}

impl Timeline {
    /// Create new empty timeline
    #[allow(dead_code)]
    pub fn new() -> Timeline {
        let collection: TimelineSlotsType = BTreeSet::new();
        Timeline { slots: collection }
    }

    /// Initialize a new timeline
    #[allow(dead_code)]
    pub fn initialize(start: NaiveDateTime, end: NaiveDateTime) -> Option<Timeline> {
        let init_slot: Slot = Slot { start, end };
        let mut collection: TimelineSlotsType = BTreeSet::new();

        if collection.insert(init_slot) {
            Some(Timeline { slots: collection })
        } else {
            None
        }
    }
}
