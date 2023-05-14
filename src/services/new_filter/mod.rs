mod filter_not_on;
mod filter_on_days;
mod filter_timing;

use crate::models::{goal::TimeFilter, timeline::Timeline};

impl Timeline {
    pub fn apply_filter(&mut self, _filter: &Option<TimeFilter>) -> Timeline {
        todo!("apply_filter not implemented");
    }
}

#[cfg(test)]
mod tests {
    // ==
}
