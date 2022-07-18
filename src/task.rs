

use serde::{Deserialize, Serialize};


/// One or many created from a Goal by the preprocessor.
/// To be scheduled in order by the scheduler.
#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Task {
	id: usize,
	goal_id: usize,
	pub duration_to_schedule: usize,
	// TODO: should split off the following fields into internal
	// scheduler implementation, but in a rush now
	pub duration_scheduled: usize,
	pub status: TaskStatus,
	pub flexibility: usize,
}

/// Serialization target to be returned at the end of scheduling
#[derive(Debug, Serialize)]
pub struct TaskResult {
	id: usize,
	goal_id: usize,
	duration_to_schedule: usize,
	pub duration_scheduled: usize,
	pub status: TaskStatus,
}

impl Task {
	pub fn new(id: usize, goal_id: usize, duration_to_schedule: usize) -> Self {
		Self {
			id,
			goal_id,
			duration_to_schedule,
			duration_scheduled: 0,
			status: TaskStatus::UNSCHEDULED,
			flexibility: 0,
		}
	}

	#[inline]
	pub fn id(&self) -> usize {
		self.id
	}

	pub fn into_task_result(self) -> TaskResult {
		TaskResult {
			id: self.id,
			goal_id: self.goal_id,
			duration_to_schedule: self.duration_to_schedule,
			duration_scheduled: self.duration_scheduled,
			status: self.status,
		}
	}
}

/// Period of time that a task can fit into.
#[derive(Serialize, Deserialize, Debug, Eq, Ord, PartialEq, PartialOrd, Clone)]
pub struct Slot {
	pub task_id: usize,
	/// in hours
	pub start: usize,
	/// in hours
	pub end: usize,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum TaskStatus {
	UNSCHEDULED,
	SCHEDULED,
	IMPOSSIBLE,
	WAITING,
}
