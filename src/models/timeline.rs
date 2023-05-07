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

    /// Get splitted timeline into slots with 1 day interval
    pub fn get_split_into_days(&self) -> Timeline {
        // TODO 2023-04-25 | test scenario:
        //  - when slots in timeline are not full days!!!! Is the split
        // will return full day or will respect the tha slot not full day!!
        let mut new_slots: TimelineSlotsType = BTreeSet::new();
        for slot in self.slots.iter() {
            new_slots.extend(slot.divide_into_days());
        }
        Timeline { slots: new_slots }
    }

    /// Get splitted timeline into slots with 1 hour interval
    pub fn get_split_into_hours(&self) -> Timeline {
        //TODO 2023-04-30 | Create a generic function to split slots into custom interval (1 hour, 10 mins, 1 day, etc)
        let mut new_slots: TimelineSlotsType = BTreeSet::new();
        for slot in self.slots.iter() {
            new_slots.extend(slot.divide_into_1h_slots());
        }
        Timeline { slots: new_slots }
    }

    /// Get merged consequent Timeline slots
    pub fn get_merged_slots(&self) -> Timeline {
        /*
        Algorithm:
        - get sorted list of slots incoming_slots (done by BTreeSet)
        - init new list for merged_slots (BTreeSet)
        - loop over slots in incoming_slots iter
            - get last item in merged_slots list to merge it with new slot
            - add last item in merged_slots with new slot
            - if addition done:
                - replace last item in the merged_slots list with
                new merged_slot
            - if addition not done:
                - add incoming_slot to the merged_slots list
        */
        let slots_len = self.slots.len();

        dbg!(&self);
        dbg!(&slots_len);

        if self.slots.is_empty() {
            return Timeline::new();
        }

        let incoming_slots = &self.slots;
        let mut merged_slots: TimelineSlotsType = BTreeSet::new();
        let mut incoming_slots_iter = incoming_slots.iter();
        merged_slots.insert(*incoming_slots_iter.next().unwrap());

        for incoming_slot in incoming_slots_iter {
            let last_merged_slot = *merged_slots.last().unwrap();

            match last_merged_slot + *incoming_slot {
                Some(merged_slot) => {
                    if merged_slots.pop_last().is_some() {
                        merged_slots.insert(merged_slot);
                    }
                }
                None => {
                    merged_slots.insert(*incoming_slot);
                }
            }
        }

        Timeline {
            slots: merged_slots,
        }
    }
    
    pub fn remove_slots_by_timing(){

    }

    /// Remove list of slots from timeline
    pub fn remove_slots(&mut self, slots_to_remove: Vec<Slot>) {
        // TODO 2023-04-30 | Apply `retain` to remove slots after splitting into 1 hour slots
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
                let subtracted_slot = *current_slot - *slot_to_remove;
                subtracted_slots.extend(subtracted_slot);
            }
        }

        if !subtracted_slots.is_empty() {
            self.slots = subtracted_slots;
        }
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
