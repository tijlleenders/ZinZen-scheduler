use crate::goal::Goal;
use time::Duration;

/// The [PreProcessor] takes in a user's goals, given a duration
pub struct PreProcessor;

impl PreProcessor {
	pub fn process_task_count(goals: &[Goal], duration: Duration) -> Vec<(usize, &Goal)> {
		let duration_in_hours = duration.whole_hours() as f64;

		goals
			.into_iter()
			.map(|goal| {
				let occurrences = duration / goal.interval;

				(occurrences.ceil(), goal)
			})
			.map(|(mut occurrences, goal)| {
				let task_time = goal.duration / occurrences;
				let in_minutes = task_time.as_seconds_f64() / 60.;

				// Smallest task time is 15 minutes
				if in_minutes < 15. {
					let goal_time = goal.duration.as_seconds_f64() / 60.;
					occurrences = (goal_time / 15.).ceil();
				}

				(occurrences as usize, goal)
			})
			.collect()
	}
}
