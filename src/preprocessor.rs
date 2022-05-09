use crate::goal::Goal;
use time::Duration;

/// The [PreProcessor] takes in a user's goals, given a duration
pub struct PreProcessor;

impl PreProcessor {
	pub fn process_task_count(goals: &[Goal], duration: Duration) -> Vec<(f64, &Goal)> {
		goals
			.into_iter()
			.map(|goal| match goal.interval {
				Some(interval) => ((duration / interval).ceil(), goal),
				None => (1f64, goal),
			})
			.collect()
	}
}
