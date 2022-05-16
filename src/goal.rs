use std::num::NonZeroUsize;

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
	pub id: NonZeroUsize,
	/// A goal's description
	pub description: String,
	/// How much total time should a user put into their goal, eg "I want to learn how to code, and I want to code 6 hours per day"
	pub task_duration: time::Duration,
	/// The interval between a Goal's tasks, this can be used to repeat a Goal daily, weekly, etc
	/// Here `interval` is fundamentally always divisible by 24.
	/// NONE means it happens only once
	pub interval: Option<time::Duration>,
	/// Allows the user to set exact times for when a task should be start, given as a date and time
	pub time_constraint: Option<time::PrimitiveDateTime>,
	/// Where each task should be committed to, eg "I want to cook for my dog at home".
	/// This is useful to make sure a schedule makes sense, since people can't teleport from place to place in minutes
	pub location_constraint: Option<usize>,
}

impl Default for Goal {
	fn default() -> Self {
		Self {
			id: unsafe { NonZeroUsize::new_unchecked(255) },
			description: "[NO DESCRIPTION]".to_string(),
			task_duration: time::Duration::ZERO,
			interval: None,
			time_constraint: (None),
			location_constraint: None,
		}
	}
}
