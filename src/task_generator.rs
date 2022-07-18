use crate::task::{Slot, Task};
use crate::task_placer::TaskPlacer;
use crate::Input;

pub fn task_generator(Input { start, _end: _, goals }: Input) -> TaskPlacer {
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

	TaskPlacer::new(tasks, slots)
}
