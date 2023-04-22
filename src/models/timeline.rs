use std::ops::Sub;

use super::slot::Slot;
use serde::Deserialize;

//TODO 2023-04-21
// - Implement Display for Timeline
// - If possible to develop divide timeline into hours, days, weeks, months, years

/// Timeline controlling pasasing list of slots in the system
/// Provide 2 public functionalities:
/// 1. remove timeline which is a list of slots
/// 2. get next slot of timeline
#[derive(Debug, Deserialize)]
pub struct Timeline {
    pub slots: Vec<Slot>,
}

pub trait TimelineOperations {
    /// Remove list of slots
    fn remove_slots(&mut self, slots: Vec<Slot>) -> Option<()>;
    /// Get next slot of timeline based on index
    fn get_next_slot(&self, index: usize) -> Option<Slot>;
}

impl TimelineOperations for Timeline {
    fn remove_slots(&mut self, _slots: Vec<Slot>) -> Option<()> {
        todo!("Implement remove_slots")
    }

    fn get_next_slot(&self, index: usize) -> Option<Slot> {
        if index < self.slots.len() {
            Some(self.slots[index as usize].clone())
        } else {
            None
        }
    }
}

/// Subtracting two timelines from each other
impl Sub for Timeline {
    type Output = Timeline;

    fn sub(self, _other: Self) -> Self::Output {
        todo!("Implement sub for Timeline")
    }
}
