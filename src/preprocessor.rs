use crate::{goal::Goal, task::Task};
use time::PrimitiveDateTime;

/// The preprocessor formats the input so the core-processor can focus on scheduling.
/// It takes the goal-tree defined in the UI and turns it into a flat list of Tasks.
/// Each Task has 0-* corresponding time-constrained Slots.
pub struct PreProcessor;

impl PreProcessor {
	pub fn generate_tasks_to_schedule(
		goals: &[Goal],
		timeline: (PrimitiveDateTime, PrimitiveDateTime),
	) -> impl Iterator<Item = (&Task)> {
		goals.iter().map(move |goal| {
			
		})
	}
}
