use chrono::{NaiveDateTime, Duration};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

/// One or many created from a Goal by the preprocessor.
/// To be scheduled in order by the scheduler.
#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Task {
	pub id: usize,
	pub goal_id: usize,
    pub title: String,
	pub duration: usize,
	pub status: TaskStatus,
	pub flexibility: usize,
	pub start: NaiveDateTime,
	pub end: NaiveDateTime,
	pub slots: Vec<(NaiveDateTime, NaiveDateTime)>,
}

impl Ord for Task {
	fn cmp(&self, other: &Self) -> Ordering {
		self.flexibility.cmp(&other.flexibility)
	}
}

impl PartialOrd for Task {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Task {
	pub fn new(id: usize, goal_id: usize, title: String, duration: usize, start: NaiveDateTime, end: NaiveDateTime) -> Self {
		Self {
			id,
			goal_id,
            title,
			duration,
			status: TaskStatus::UNSCHEDULED,
			flexibility: 0,
			start,
			end,
			slots: Vec::new(),
		}
	}

    pub fn calculate_flexibility(&mut self) {
        let duration_available = self.slots[self.slots.len()-1].1 - self.slots[0].0;
        let hours_available = duration_available.num_hours() as usize;
        self.flexibility = hours_available - self.duration + 1;
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum TaskStatus {
	UNSCHEDULED,
	SCHEDULED,
	IMPOSSIBLE,
	WAITING,
}
