use crate::{console, error::ErrorCode, IPC_BUFFER};
use nanoserde::{DeJson, SerJson};

/// Loads [Goal]s inserted into IPC by JavaScript
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

/// An internal ID that is auto-incremented for each goal declared
static mut AUTO_INCREMENTING_ID: usize = 0;

/// A [Goal] is what one wants to do, it is used in conjunction with a [Timeline] to generate a [Schedule]
#[derive(DeJson, SerJson, Debug)]
#[non_exhaustive]
pub struct Goal {
	pub id: usize,
	pub description: String,
	pub duration: usize,
	pub repetition: Repetition,
	pub time_constraint: Option<usize>,
	pub location_constraint: Option<usize>,
}

impl Default for Goal {
	fn default() -> Self {
		let new_id = unsafe {
			AUTO_INCREMENTING_ID += 1;
			AUTO_INCREMENTING_ID
		};

		Self {
			id: new_id,
			description: "[NO DESCRIPTION]".to_string(),
			duration: 0,
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
