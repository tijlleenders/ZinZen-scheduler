use crate::{error::Explode, IPC_BUFFER};
use serde::{Deserialize, Serialize};
use std::num::NonZeroUsize;
use time::PrimitiveDateTime;

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
	pub title: String,
	/// How much total time (default assumes number of hours) should a user put into their goal per repetition
	pub duration: Option<usize>,
	/// Allows the user to set the earliest schedulable datetime for tasks to achieve this goal
	pub start: Option<time::PrimitiveDateTime>,
	/// When this Goal's tasks should end
	pub deadline: Option<time::PrimitiveDateTime>,
}

impl Default for Goal {
	fn default() -> Self {
		Self {
			id: unsafe { NonZeroUsize::new_unchecked(0) },
			title: "[NO DESCRIPTION]".to_string(),
			duration: None,
			start: None,
			deadline: None,
		}
	}
}
