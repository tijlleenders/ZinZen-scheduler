mod filter_timing;

use crate::models::{
    goal::{Day, TimeFilter},
    slot::Slot,
    timeline::Timeline,
};

impl Timeline {
    pub fn apply_filter(&mut self, _filter: &Option<TimeFilter>) -> Timeline {
        todo!("apply_filter not implemented");
    }
}

/// Filtering timeline based on on_days field in TimeFilter
fn _filter_on_days(_timeline: Timeline, _days_to_filter: &[Day]) {
    todo!("filter_on_days not implemented");
}

/// Filtering timeline based on not_on field in TimeFilter
fn _filter_not_on(_timeline: Timeline, _slots_to_filter: &[Slot]) {
    todo!("filter_not_on not implemented");
}

/// Validate that a given value is valid time number which must be between 0 and 24
fn _validate_time(time: Option<usize>, time_name: &str) {
    if let Some(time) = time {
        if time > 24 {
            panic!("{} must be between 0 and 24", time_name);
        }
    }
}

#[cfg(test)]
mod tests {
    // ==
}
