use time::Duration;

use crate::{
	goal::{Goal, Repetition},
	task::Task,
};

/// The [PreProcessor] takes in a user's goals, given a duration
pub struct PreProcessor;

impl PreProcessor {
	pub fn process_task_count(goals: &[Goal], duration: Duration) -> Vec<(usize, &Goal)> {
		let duration_in_hours = duration.whole_hours() as f64;

		goals
			.into_iter()
			.map(|goal| {
				let occurrences = match goal.repetition {
					Repetition::Exact(exact) => exact as f64,
					Repetition::Once => 1.,
					Repetition::Daily => duration_in_hours / 24.,
					Repetition::Weekly => duration_in_hours / 168.,
					Repetition::Monthly => duration_in_hours / 672.,
					Repetition::Annually => duration_in_hours / 8064.,
				};

				(occurrences.ceil(), goal)
			})
			.map(|(mut occurrences, goal)| {
				let task_time = goal.duration / occurrences;
				let in_minutes = task_time.as_seconds_f64() / 60.;

				// Smallest task time is 15 minutes
				if in_minutes < 15. {
					occurrences = (in_minutes / 15.).ceil();
				}

				(occurrences as usize, goal)
			})
			.collect()
	}
}
