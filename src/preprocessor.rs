use time::Duration;

use crate::goal::Goal;
use time::PrimitiveDateTime;

/// The preprocessor formats the input so the core-processor can focus on scheduling.
/// It takes the goal-tree defined in the UI and turns it into a flat list of Tasks.
/// Each Task has 0-* corresponding time-constrained Slots.
pub struct PreProcessor;

impl PreProcessor {
	pub fn process_task_count(
		goals: &[Goal],
		timeline: (PrimitiveDateTime, PrimitiveDateTime),
	) -> impl Iterator<Item = (usize, &Goal)> {
		goals.iter().map(move |goal| {
			// Little nudge to prevent over-posting
			let start = goal.start.unwrap_or(timeline.0) + Duration::seconds(1);
			let finish = goal.deadline.unwrap_or(timeline.1);

			match goal.interval {
				Some(interval) => (((finish - start) / interval).floor() as usize + 1, goal),
				None => (1, goal),
			}
		})
	}
}
