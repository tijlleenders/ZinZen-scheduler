use crate::task::{Slot, Task};
use crate::task_placer::TaskPlacer;
use crate::Input;

pub fn task_generator(Input { start, end, goals }: Input) -> TaskPlacer {
	let mut tasks = vec![];
	let mut slots = vec![];

	let mut id = 0;
	for goal in goals.into_iter() {
		// Default dates for goals if start/deadline is not set
		let goal_start = goal.start.unwrap_or(start);
		let goal_end = goal.deadline.unwrap_or(end);

		// If there's repetition, create multiple tasks
		if let Some(repetition) = goal.repetition {
			let repeat_interval = repetition.into_hours();

			let mut task_start = (goal_start - start).whole_hours();
			let mut task_end = (goal_start - start).whole_hours() + repeat_interval;

			let goal_end_hours = (goal_end - start).whole_hours();
			while task_start < goal_end_hours {
				tasks.push(Task::new(id, goal.id, goal.duration));
				slots.push(Slot::new(id, task_start as usize, task_end as usize));

				task_start += repeat_interval;
				task_end += repeat_interval;
				id += 1;
			}
			continue;
		}

		tasks.push(Task::new(id, goal.id, goal.duration));
		slots.push(Slot::new(
			id,
			(goal_start - start).whole_hours() as usize,
			(goal_end - start).whole_hours() as usize,
		));
		id += 1;
	}

	TaskPlacer::new(tasks, slots)
}

#[cfg(test)]
mod tests {
	use crate::task_generator::task_generator;
	use crate::Input;

	use super::*;

	#[test]
	fn repeat() {
		let input: Input = serde_json::from_str(
			r#"
{
	"startDate": "2022-01-01T00:00:00Z",
	"endDate": "2022-01-04T00:00:00Z",
	"goals": [
		{
			"id": 1,
			"title": "walk",
			"duration": 1,
			"repetition": "daily"
		}
	]
}
        "#,
		)
		.unwrap();

		let scheduler = task_generator(input);
		assert_eq!(
			scheduler.tasks,
			vec![Task::new(0, 1, 1), Task::new(1, 1, 1), Task::new(2, 1, 1)]
		);
		assert_eq!(
			scheduler.slots,
			vec![Slot::new(0, 0, 24), Slot::new(1, 24, 48), Slot::new(2, 48, 72)]
		)
	}
}
