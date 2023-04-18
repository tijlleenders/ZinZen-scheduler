use super::{utils::get_start_of_repeat_step, Slot, TimeSlotsIterator};

impl Iterator for TimeSlotsIterator {
    type Item = Vec<Slot>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.timeline.is_empty() {
            return None;
        }
        match self.repetition {
            Some(repetition) => {
                let mut result = vec![];

                let next_start_position =
                    get_start_of_repeat_step(&self.current_start_position, repetition);
                let mut indexes_to_delete_count: usize = 0;
                for slot in self.timeline.iter_mut() {
                    if next_start_position.le(&slot.end) && next_start_position.gt(&slot.start) {
                        //next_start_position is 'on' the current slot
                        result.push(Slot {
                            start: slot.start,
                            end: next_start_position,
                        });
                        if next_start_position.eq(&slot.end) {
                            indexes_to_delete_count += 1;
                        } else {
                            slot.start = next_start_position;
                        }
                        self.current_start_position = next_start_position;
                        continue;
                    } else if next_start_position.gt(&slot.end) {
                        //next_start_position is 'past' the current slot
                        indexes_to_delete_count += 1;
                        result.push(*slot);
                    } else {
                        //next_start_position is 'before' the current slot
                        self.current_start_position = next_start_position;
                        break;
                    }
                }
                for _i in 1..=indexes_to_delete_count {
                    self.timeline.remove(0);
                }
                Some(result)
            }
            None => {
                let result = self.timeline.clone();
                self.timeline.clear();
                Some(result)
            }
        }
    }
}
