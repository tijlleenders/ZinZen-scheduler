use serde::Deserialize;
use time::macros::datetime;
use time::PrimitiveDateTime;
use time::{Duration, OffsetDateTime};

use crate::goal::Goal;
use crate::scheduler_core::CoreScheduler;
use crate::task::{Slot, Task};
use crate::Input;

pub fn preprocess(Input { start, end, goals }: Input) -> CoreScheduler {
	let tasks = goals
		.iter()
		.enumerate()
		.map(|(id, goal)| Task::new(id, goal.id, goal.duration))
		.collect::<Vec<_>>();

	// TODO: goal/task may have multiple slots
	let slots = goals
		.iter()
		.enumerate()
		.map(|(id, goal)| Slot {
			task_id: id,
			start: (goal.start - start).whole_hours() as usize,
			end: (goal.deadline - start).whole_hours() as usize,
		})
		.collect::<Vec<_>>();

	CoreScheduler::new(tasks, slots)
}
