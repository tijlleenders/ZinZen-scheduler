use std::{collections::BTreeSet, ops::Sub};

use super::slot::Slot;
use chrono::NaiveDateTime;
use serde::Deserialize;

//TODO 2023-04-21
// - Implement Display for Timeline
// - If possible to develop divide timeline into hours, days, weeks, months, years

/// Timeline controlling passing list of slots in the system
/// Provide 2 public functionalities:
/// 1. remove timeline which is a list of slots
/// 2. get next slot of timeline
#[derive(Debug, Deserialize, PartialEq)]
pub struct Timeline {
    pub slots: BTreeSet<Slot>,
}

impl Timeline {
    /// Initialize a new timeline
    pub fn initialize(start: NaiveDateTime, end: NaiveDateTime) -> Option<Timeline> {
        let init_slot: Slot = Slot {
            start: start,
            end: end,
        };
        let mut collection: BTreeSet<Slot> = BTreeSet::new();

        if collection.insert(init_slot) {
            Some(Timeline { slots: collection })
        } else {
            None
        }
    }
}
pub trait TimelineOperations {
    /// Insert list of slots to the Timeline
    /// - It guerantees that slots are sorted, and not duplicated
    /// - If it the timeline is empty, will initialize it then insert
    fn insert(new_slots: Vec<Slot>);
    /// Remove list of slots
    /// - ? Should remove similar slots, or remove from each slot in timeline!!
    /// - ? Remove from each slot in the timeline means remove one slot from many
    fn remove_slots(&mut self, slots: Vec<Slot>) -> Option<Vec<Slot>>;
    /// Get next slot of timeline based on index
    fn get_next_slot(&self, index: usize) -> Option<Slot>;
}

impl TimelineOperations for Timeline {
    fn remove_slots(&mut self, _slots: Vec<Slot>) -> Option<Vec<Slot>> {
        todo!("Implement remove_slots")
    }

    fn get_next_slot(&self, _index: usize) -> Option<Slot> {
        todo!("Implement get_next_slot")
        // if index < self.slots.len() {
        //     Some(self.slots[index as usize].clone())
        // } else {
        //     None
        // }
    }

    fn insert(_new_slots: Vec<Slot>) {
        todo!("Implement insert")
    }
}

/// Subtracting two timelines from each other
impl Sub for Timeline {
    type Output = Timeline;

    fn sub(self, _other: Self) -> Self::Output {
        todo!("Implement sub for Timeline")
    }
}
