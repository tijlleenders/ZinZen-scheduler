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

    /// Remove list of slots from timeline
    pub fn remove_slots(&mut self, slots_to_remove: Vec<Slot>) {
        dbg!(&self);
        let mut to_remove: TimelineSlotsType = BTreeSet::new();
        to_remove.extend(slots_to_remove);
        dbg!(&to_remove);
        // Remove similar slots from Timeline
        self.slots.retain(|slot| !to_remove.contains(slot));
        dbg!(&self);

        /*
        =========
        Algorithm - V2 - 2023-05-22

        - for each slot_to_filter in slots_to_remove
            - iterate over timeline_slots and get first slot which overlaps with slot_to_filter
            - subtract ovelapped_timeline_slot from slot to remove and assign it to slots_after_subtraction
            - remove overlapped_timeline_slot from timeline
            - push slots_after_subtraction to the timeline_slots
        */
        for slot_to_filter in to_remove {
            dbg!(&slot_to_filter);
            let timeline_slots = self.slots.clone();
            match timeline_slots
                .iter()
                .find(|slot| slot.is_contains_slot(&slot_to_filter))
            {
                Some(overlapped_timeline_slot) => {
                    dbg!(&overlapped_timeline_slot);

                    let slots_after_subtraction = *overlapped_timeline_slot - slot_to_filter;
                    dbg!(&slots_after_subtraction);

                    self.slots.retain(|slot| slot != overlapped_timeline_slot);
                    self.slots.extend(slots_after_subtraction);
                    dbg!(&self);
                }
                None => continue,
            };
        }

        dbg!(&self);
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
