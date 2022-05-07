use crate::{console, error::ErrorCode, IPC_BUFFER};
use serde::{Deserialize, Serialize};

/// Loads [`Goal`] inserted into IPC by JavaScript
pub unsafe fn load_goals_from_ipc(ipc_offset: usize) -> Vec<Goal> {
	let slice = &IPC_BUFFER[..ipc_offset];

	match serde_json::from_slice(slice) {
		Ok(ok) => ok,
		Err(err) => console::log_err(ErrorCode::DeserializationError, err),
	}
}

/// A [Goal] is what one wants to do, it is used in conjunction with a span of time to generate a [Schedule]
#[derive(Serialize, Deserialize, Debug)]
#[non_exhaustive]
pub struct Goal {
	/// Every goal has a unique ID
	pub id: usize,
	/// A goal's description
	pub description: String,
	/// How much total time should a user put into their goal, eg "I want to learn how to code, and I want to dedicate 72 hours of work"
	pub duration: time::Duration,
	/// The interval between a Goal's tasks, this can be used to repeat a Goal daily, weekly, etc
	pub interval: time::Duration,
	/// Allows the user to set exact times for when a task should be done, given as a time of day and an interval in hours
	/// Here `interval` is fundamentally always divisible by 24
	pub time_constraint: Option<time::Time>,
	/// Where each task should be committed to, eg "I want to cook for my dog at home".
	/// This is useful to make sure a schedule makes sense, since people can't teleport from place to place in minutes
	pub location_constraint: Option<usize>,
}

impl Default for Goal {
	fn default() -> Self {
		Self {
			id: 0,
			description: "[NO DESCRIPTION]".to_string(),
			duration: time::Duration::ZERO,
			interval: time::Duration::DAY,
			time_constraint: (None),
			location_constraint: None,
		}
	}
}

impl Goal {
	pub(crate) fn intersects(
		&self,
		other: &Goal,
		task_allocation_a: time::Duration,
		task_allocation_b: time::Duration,
	) -> bool {
		// No constraint, no intersection
		if let (Some(time_a), Some(time_b)) = (self.time_constraint, other.time_constraint) {
			// A intersects B, if time_alloc of A enters into interval of B
			let a_intersects_b = (time_a - time_b).abs() < task_allocation_a;

			// B intersects A, if time_alloc of B enters into interval of A
			let b_intersects_a = time_b + task_allocation_b > time_a + self.interval;

			a_intersects_b || b_intersects_a
		} else {
			false
		}
	}
}
