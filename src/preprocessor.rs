use crate::goal::Goal;
use time::PrimitiveDateTime;

/// The [PreProcessor] takes in a user's goals, given a duration
pub struct PreProcessor;

impl PreProcessor {
	pub fn process_task_count(
		goals: &[Goal],
		timeline: (PrimitiveDateTime, PrimitiveDateTime),
	) -> impl Iterator<Item = (usize, &Goal)> {
		goals.iter().map(move |goal| match goal.interval {
			Some(interval) => match goal.deadline {
				Some(deadline) => (((deadline - timeline.0) / interval).ceil() as usize, goal),
				None => (((timeline.1 - timeline.0) / interval).ceil() as usize, goal),
			},
			None => (1, goal),
		})
	}
}
