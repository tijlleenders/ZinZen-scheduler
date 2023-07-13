use super::{utils::get_start_of_repeat_step, Slot, TimeSlotsIterator};
use crate::models::timeline::Timeline;
use core::panic;

impl Iterator for TimeSlotsIterator {
    type Item = Timeline;
    fn next(&mut self) -> Option<Self::Item> {
        if self.timeline.slots.is_empty() {
            return None;
        }
        match self.repetition {
            Some(repetition) => {
                let mut result = Timeline::new();

                let next_start_position =
                    get_start_of_repeat_step(&self.current_start_position, repetition);
                let mut indexes_to_delete_count: usize = 0;
                for mut slot in self.timeline.slots.clone().into_iter() {
                    if next_start_position.le(&slot.end) && next_start_position.gt(&slot.start) {
                        //next_start_position is 'on' the current slot
                        result.slots.insert(Slot {
                            start: slot.start,
                            end: next_start_position,
                        });

                        if next_start_position.eq(&slot.end) {
                            indexes_to_delete_count += 1;
                        } else {
                            // Becasue for is looping over a clone of timeline.slots,
                            //so we will replace slot inside timeline.slots by removing
                            //slot then add it after edit
                            if self.timeline.slots.remove(&slot) {
                                slot.start = next_start_position;
                                self.timeline.slots.insert(slot);
                            } else {
                                panic!("Timeline slots item should be changed after slot.start; but it can't");
                            }
                        }
                        self.current_start_position = next_start_position;
                        continue;
                    } else if next_start_position.gt(&slot.end) {
                        //next_start_position is 'past' the current slot
                        indexes_to_delete_count += 1;
                        result.slots.insert(slot);
                    } else {
                        //next_start_position is 'before' the current slot
                        self.current_start_position = next_start_position;

                        break;
                    }
                }
                for _i in 1..=indexes_to_delete_count {
                    self.timeline.slots.pop_first();
                }
                Some(result)
            }
            None => {
                let result = self.timeline.clone();
                self.timeline.slots.clear();
                Some(result)
            }
        }
    }
}
