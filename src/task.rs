use chrono::{Duration, NaiveDateTime};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

/// One or many created from a Goal by the preprocessor.
/// To be scheduled in order by the scheduler.
#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Task {
	pub id: usize,
	pub goal_id: usize,
	pub title: String,
	pub duration: usize,
	pub status: TaskStatus,
	pub flexibility: usize,
	pub start: NaiveDateTime,
	pub deadline: NaiveDateTime,
	pub slots: Vec<(NaiveDateTime, NaiveDateTime)>,
	pub confirmed_start: Option<NaiveDateTime>,
	pub confirmed_deadline: Option<NaiveDateTime>,
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
	pub fn new(
		id: usize,
		goal_id: usize,
		title: String,
		duration: usize,
		start: NaiveDateTime,
		deadline: NaiveDateTime,
	) -> Self {
		Self {
			id,
			goal_id,
			title,
			duration,
			status: TaskStatus::UNSCHEDULED,
			flexibility: 0,
			start,
			deadline,
			slots: Vec::new(),
			confirmed_start: None,
			confirmed_deadline: None,
		}
	}

	pub fn calculate_flexibility(&mut self) {
		let duration_available = self.slots[self.slots.len() - 1].1 - self.slots[0].0;
		let hours_available = duration_available.num_hours() as usize;
		self.flexibility = hours_available - self.duration + 1;
	}

	pub fn set_confirmed_start(&mut self, start: NaiveDateTime) {
		self.confirmed_start = Some(start);
	}

	pub fn set_confirmed_deadline(&mut self, deadline: NaiveDateTime) {
		self.confirmed_deadline = Some(deadline);
	}

    pub fn get_slots(&self) -> Vec<(NaiveDateTime,NaiveDateTime)> {
        self.slots.clone()
    }

    pub fn remove_slot(&mut self, slot: &(NaiveDateTime,NaiveDateTime)) {
        for i in 0..self.slots.len() {
            if &self.slots[i] == slot {
                self.slots.remove(i);
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum TaskStatus {
	UNSCHEDULED,
	SCHEDULED,
	IMPOSSIBLE,
	WAITING,
}
