use std::num::NonZeroUsize;

use serde::{Deserialize, Serialize};
use time::{Duration, PrimitiveDateTime};

use crate::{error::Explode, IPC_BUFFER};

/// Loads [`Goal`] inserted into IPC by JavaScript
pub unsafe fn load_goals_from_ipc(ipc_offset: usize) -> (Vec<Goal>, (PrimitiveDateTime, PrimitiveDateTime)) {
	let slice = &IPC_BUFFER[..ipc_offset];
	serde_json::from_slice(slice).explode()
}

/// A [Goal] is what one wants to do, it is used in conjunction with a span of time to generate a [Schedule]
#[derive(Serialize, Deserialize, Debug)]
pub struct Goal {
	/// Every goal has a unique ID
	pub id: NonZeroUsize,
	/// A goal's description
	pub description: String,
	/// How much total time should a user put into their goal, eg "I want to learn how to code, and I want to code 6 hours per day"
	pub duration: Duration,
	/// Earliest start datetime for this Goal's Tasks
	pub start: Option<PrimitiveDateTime>,
	/// Deadline for this Goal's Tasks
	pub deadline: Option<PrimitiveDateTime>,
}

impl Default for Goal {
	fn default() -> Self {
		Self {
			id: unsafe { NonZeroUsize::new_unchecked(0) },
			description: "[NO DESCRIPTION]".to_string(),
			duration: time::Duration::ZERO,
			deadline: None,
			start: None,
		}
	}
}
