use crate::{
	goal::{Goal, Repetition},
	task::Task,
};

/// The [PreProcessor] takes in a user's goals, given a duration
pub struct PreProcessor;

impl PreProcessor {
	pub fn generate_tasks(goals: &[Goal], time_in_hours: usize) -> Vec<Task> {
		let mut tasks = vec![];

		goals
			.into_iter()
			.map(|goal| {
				let occurrences = match goal.repetition {
					Repetition::Exact(exact) => exact,

					Repetition::Once => 1,
					Repetition::Daily => {
						if time_in_hours < 24 {
							1
						} else {
							time_in_hours / 24
						}
					}
					Repetition::Weekly => {
						if time_in_hours < 168 {
							1
						} else {
							time_in_hours / 168
						}
					}
					Repetition::Monthly => {
						if time_in_hours < 672 {
							1
						} else {
							time_in_hours / 672
						}
					}
					Repetition::Annually => {
						if time_in_hours < 8064 {
							1
						} else {
							time_in_hours / 8064
						}
					}
				};

				(occurrences, goal)
			})
			.for_each(|(occurrences, goal)| {
				for _ in 0..occurrences {
					tasks.push(Task::new(goal))
				}
			});

		tasks
	}
}
