use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;

use crate::scheduler::Schedule;

/// A [Task] is an item a user is expected to accomplish, it is simply a time-slice in a user's schedule.
/// Through many tasks can a user achieve a
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
	/// What goal originally described this task
	pub(crate) goal_id: usize,
	/// When this task starts
	pub(crate) start: PrimitiveDateTime,
	/// When this task ends
	pub(crate) finish: PrimitiveDateTime,
	/// A Tasks flexibility is how flexible it is within it's allocated time frame.
	pub(crate) flexibility: f64,
}

impl Task {
	pub(crate) fn fill(schedule: &Schedule) -> Self {
		let max_seconds = (schedule.timeline.1 - schedule.timeline.0).as_seconds_f64().abs();

		Self {
			goal_id: 0,
			start: schedule.timeline.0,
			finish: schedule.timeline.1,
			flexibility: max_seconds / (3600.0),
		}
	}
}
