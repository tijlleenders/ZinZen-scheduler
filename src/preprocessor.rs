use crate::goal::Goal;
use time::Duration;

/// The [PreProcessor] takes in a user's goals, given a duration
pub struct PreProcessor;

impl PreProcessor {
	pub fn process_task_count(goals: &[Goal], duration: Duration) -> impl Iterator<Item = (usize, &Goal)> {
		goals.iter().map(move |goal| match goal.interval {
			Some(interval) => ((duration / interval).ceil() as usize, goal),
			None => (1, goal),
		})
	}
}
