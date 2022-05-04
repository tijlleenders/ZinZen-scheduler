use crate::{console, error::ErrorCode, IPC_BUFFER};
use nanoserde::{DeJson, SerJson};

/// Loads [`Goal`] inserted into IPC by JavaScript
pub unsafe fn load_goals_from_ipc(ipc_offset: usize) -> Vec<Goal> {
	let slice = &IPC_BUFFER[..ipc_offset];

	let string = match std::str::from_utf8(slice) {
		Ok(str) => str,
		Err(err) => console::log_err(ErrorCode::DataInIPCNotValidUTF8, err),
	};

	match DeJson::deserialize_json(string) {
		Ok(ok) => ok,
		Err(err) => console::log_err(ErrorCode::DeserializationError, err),
	}
}

/// A [Goal] is what one wants to do, it is used in conjunction with a span of time to generate a [Schedule]
#[derive(DeJson, SerJson, Debug)]
#[non_exhaustive]
pub struct Goal {
	/// Every goal has a unique ID
	pub id: usize,
	/// A goal's description
	pub description: String,
	/// How much total time should a user put into their goal, eg "I want to learn how to code, and I want to dedicate 72 hours of work"
	pub duration: f32,
	/// How often should this goal's tasks appear in the user's schedule, eg "I want to go to the book club weekly"
	pub repetition: Repetition,
	/// At what exact time of the day do ou want the tasks to start
	pub time_constraint: Option<usize>,
	/// Where each task should be committed to, eg "I want to cook for my dog at home".
	/// This is useful to make sure a schedule makes sense, since people can't teleport from place to place in minutes
	pub location_constraint: Option<usize>,
}

impl Default for Goal {
	fn default() -> Self {
		Self {
			id: 0,
			description: "[NO DESCRIPTION]".to_string(),
			duration: 0.0,
			repetition: Repetition::Once,
			time_constraint: None,
			location_constraint: None,
		}
	}
}

/// How often should a task be included in the schedule
#[derive(DeJson, SerJson, Debug)]
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
