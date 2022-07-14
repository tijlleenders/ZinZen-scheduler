use std::num::NonZeroUsize;

use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime, PrimitiveDateTime};

/// One or many created from a Goal by the preprocessor.
/// To be scheduled in order by the scheduler.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Task {
	task_id: usize,
	goal_id: usize,
	duration_to_schedule: usize,
	// TODO: should split off the following fields into internal
	// scheduler implementation, but in a rush now
	pub duration_scheduled: usize,
	pub task_status: TaskStatus,
	/// The slots that this task can fit into.
	pub slots: Vec<Slot>,
}

impl Task {
	pub fn new(task_id: usize, goal_id: usize, duration_to_schedule: usize) -> Self {
		Self {
			task_id,
			goal_id,
			duration_to_schedule,
			duration_scheduled: 0,
			task_status: TaskStatus::UNSCHEDULED,
			slots: vec![],
		}
	}
}

/// Period of time that a task can fit into.
#[derive(Serialize, Deserialize, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Slot {
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
