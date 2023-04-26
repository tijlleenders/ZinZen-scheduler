use std::collections::BTreeSet;

use super::slot::Slot;
use chrono::NaiveDateTime;
use serde::Deserialize;

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
    pub fn new() -> Timeline {
        let collection: TimelineSlotsType = BTreeSet::new();
        Timeline { slots: collection }
    }

    /// Initialize a new timeline
    pub fn initialize(start: NaiveDateTime, end: NaiveDateTime) -> Option<Timeline> {
        let init_slot: Slot = Slot { start, end };
        let mut collection: TimelineSlotsType = BTreeSet::new();

        if collection.insert(init_slot) {
            Some(Timeline { slots: collection })
        } else {
            None
        }
    }

    /// Split timeline into slots with 1 day interval
    pub fn split_into_days(&self) -> Timeline {
        // TODO 2023-04-25 | test scenario:
        //  - when slots in timeline are not full days!!!! Is the split
        // will return full day or will respect the tha slot not full day!!
        let mut new_slots: TimelineSlotsType = BTreeSet::new();
        for slot in self.slots.iter() {
            new_slots.extend(slot.divide_into_days());
        }
        Timeline { slots: new_slots }
    }

    /// Remove list of slots from timeline
    pub fn remove_slots(&mut self, slots_to_remove: Vec<Slot>) {
        let mut to_remove: TimelineSlotsType = BTreeSet::new();
        to_remove.extend(slots_to_remove);

        // Remove similar slots from Timeline
        for slot in &to_remove {
            self.slots.remove(slot);
        }

        // Remove from each slot in the timeline
        // Alogritm:
        // - Subtract each slot in timeline from each slot in to_remove, results in subtracted_slots
        // - Remove all items in timeline and insert sutracted_slots
        let mut subtracted_slots: TimelineSlotsType = BTreeSet::new();
        for current_slot in self.slots.iter() {
            for slot_to_remove in &to_remove {
                subtracted_slots.extend(*current_slot - *slot_to_remove);
            }
        }
        self.slots = subtracted_slots.clone();
    }

    /// Get a slot of timeline based on index
    /// - If index is out of range, return None
    pub fn get_slot(&self, index: usize) -> Option<Slot> {
        // TODO 2023-04-22 | change return type to Result to show error index out of range when happened

        if index < self.slots.len() {
            let slot = self.slots.iter().nth(index).unwrap();
            Some(*slot)
        } else {
            None
        }
    }
}
