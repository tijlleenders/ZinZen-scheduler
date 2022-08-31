use chrono::{Duration, NaiveDateTime};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

/// One or many created from a Goal.
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
    pub after_time: usize,
    pub before_time: usize,
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
        after_time: usize,
        before_time: usize,
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
            after_time,
            before_time,
			slots: Vec::new(),
			confirmed_start: None,
			confirmed_deadline: None,
		}
	}

    //TODO: The current way this is done may not be entirely accurate for tasks that can be done on
    //multiple days within certain time bounds.
	pub fn calculate_flexibility(&mut self) {
        let hours_available = self.slots.len(); 
		self.flexibility = hours_available - self.duration + 1;
	}

	pub fn set_confirmed_start(&mut self, start: NaiveDateTime) {
		self.confirmed_start = Some(start);
	}

	pub fn set_confirmed_deadline(&mut self, deadline: NaiveDateTime) {
		self.confirmed_deadline = Some(deadline);
	}

	pub fn get_slots(&self) -> Vec<(NaiveDateTime, NaiveDateTime)> {
		self.slots.clone()
	}

	pub fn remove_slot(&mut self, slot: &(NaiveDateTime, NaiveDateTime)) {
		let mut index = 0;
		for i in 0..self.slots.len() {
			if &self.slots[i] == slot {
				index = i;
			}
		}
		self.slots.remove(index);
	}
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum TaskStatus {
	UNSCHEDULED,
	SCHEDULED,
	IMPOSSIBLE,
	WAITING,
}
