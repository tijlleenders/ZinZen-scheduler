use super::slot::Slot;
use serde::Deserialize;

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
    fn remove_slots(&mut self, slots: Vec<Slot>);
    /// Get next slot of timeline based on index
    fn get_next_slot(&self, index: i32) -> Option<Slot>;
}

