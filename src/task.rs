use crate::goal::Goal;
use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;

/// A [Task] is an item a user is expected to accomplish, it is simply a time-slice in a user's schedule.
/// Through many tasks can a user achieve a
#[derive(Serialize, Deserialize)]
pub struct Task {
	/// What goal originally described this task
	pub(crate) goal_id: usize,
	/// When this task starts
	pub(crate) start: PrimitiveDateTime,
	/// When this task ends
	pub(crate) finish: PrimitiveDateTime,
}
