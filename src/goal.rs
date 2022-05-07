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
	/// How often should this goal's tasks appear in the user's schedule, eg "I want to go to the book club weekly"
	pub repetition: Repetition,
	/// Allows the user to set exact times for when a task should be done, given as a time of day
	pub time_constraint: Option<time::PrimitiveDateTime>,
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
			repetition: Repetition::Once,
			time_constraint: None,
			location_constraint: None,
		}
	}
}

/// How often should a task be included in the schedule
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Repetition {
	/// If this value is `x`, then the user wants to do the task `x` times, irrespective of the timeline
	Exact(usize),
	/// Exactly once, eg you only need to go to buy a car once
	Once,
	/// Do the task each day in the schedule
	Daily,
	/// Do the task each week in the schedule
	Weekly,
	/// Do the task each month in the schedule
	Monthly,
	/// Do the task each year in the schedule
	Annually,
}
