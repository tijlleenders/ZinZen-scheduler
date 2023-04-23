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
        let init_slot: Slot = Slot { start, end };
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
    fn insert_slots(&mut self, new_slots: Vec<Slot>) -> Option<()>;
    /// Remove list of slots
    /// - ? Should remove similar slots, or remove from each slot in timeline!!
    /// - ? Remove from each slot in the timeline means remove one slot from many
    fn remove_slots(&mut self, slots_to_remove: Vec<Slot>) -> Option<Vec<Slot>>;
    /// Get a slot of timeline based on index
    /// - If index is out of range, return None
    fn get_slot(&self, index: usize) -> Option<Slot>;
}

impl TimelineOperations for Timeline {
    fn remove_slots(&mut self, slots_to_remove: Vec<Slot>) -> Option<Vec<Slot>> {
        if slots_to_remove.is_empty() {
            return None;
        }

        let mut to_remove: BTreeSet<Slot> = BTreeSet::new();
        to_remove.extend(slots_to_remove);

        // Remove similar slots from Timeline
        for slot in &to_remove {
            self.slots.remove(slot);
        }

        // Remove from each slot in the timeline
        // Alogritm:
        // - Subtract each slot in timeline from each slot in to_remove, results in subtracted_slots
        // - Remove all items in timeline and insert sutracted_slots
        let mut subtracted_slots: BTreeSet<Slot> = BTreeSet::new();
        for current_slot in self.slots.iter() {
            for slot_to_remove in &to_remove {
                subtracted_slots.extend(*current_slot - *slot_to_remove);
            }
        }
        self.slots = subtracted_slots.clone();
        Some(subtracted_slots.into_iter().collect())
    }

    fn get_slot(&self, index: usize) -> Option<Slot> {
        // TODO 2023-04-22 | chainsertnge return type to Result to show error index out of range when happened

        if index < self.slots.len() {
            let slot = self.slots.iter().nth(2).unwrap();
            Some(*slot)
        } else {
            None
        }
    }

    fn insert_slots(&mut self, new_slots: Vec<Slot>) -> Option<()> {
        for slot in new_slots {
            if !self.slots.insert(slot) {
                return None;
            }
        }
        Some(())
    }
}

/// Subtracting two timelines from each other
impl Sub for Timeline {
    type Output = Timeline;

    fn sub(self, _other: Self) -> Self::Output {
        todo!("Implement sub for Timeline")
    }
}