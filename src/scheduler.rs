use crate::{
	console,
	error::{exit, ErrorCode},
	goal::Goal,
	preprocessor::PreProcessor,
	task::Task,
};
use linked_list::{Cursor, LinkedList};
use serde::Serialize;
use time::{Duration, PrimitiveDateTime};

/// A slot in a user's schedule, can be free time or contains a tasks
#[derive(Debug)]
pub enum ScheduleSlot {
	Occupied(Task),
	Free((PrimitiveDateTime, PrimitiveDateTime)),
}

/// A schedule is just a list of tasks which satisfy the user's time and location constraints
pub struct Schedule {
	slots: LinkedList<ScheduleSlot>,
	timeline: (PrimitiveDateTime, PrimitiveDateTime),
}

impl Schedule {
	pub fn generate_schedule(
		goals: &mut [Goal],
		timeline: (PrimitiveDateTime, PrimitiveDateTime),
	) -> Result<Schedule, String> {
		let max_free_time = timeline.1 - timeline.0;

		// Slots initially begin with full free time
		let mut slots = LinkedList::new();
		slots.push_front(ScheduleSlot::Free(timeline));

		// ======================= TIMELINE & GOAL CHECKS =================================
		// Make sure no Goal exceeds the user's free time
		if goals.iter().any(|g| g.duration >= max_free_time) {
			return Err("A goal was found with a duration greater than the timeline duration".into());
		}

		// Make sure the user's free time is enough to accommodate all goal's durations
		let total_goal_duration = goals.iter().map(|g| g.duration).reduce(|a, b| a + b);

		if let Some(total) = total_goal_duration {
			if total >= max_free_time {
				return Err("There isn't enough time in the user's schedule to accommodate all Goal's, either increase your expected timeline or reduce your individual Goal's allocated time".into());
			}

			// If the user allocates no time to any Goal, then all time is free time :)
			if total == Duration::ZERO {
				return Ok(Schedule { slots, timeline });
			}
		} else {
			// If the user has no goals then they have all free time
			return Ok(Schedule { slots, timeline });
		};

		// ========================= CHECK AND VALIDATE TIME_CONSTRAINT BOUNDS =================
		// Produce a tuple containing task count and goal
		let goals_occurrences = PreProcessor::process_task_count(goals, timeline.0 - timeline.1);
		let goals_occurrences_copy = PreProcessor::process_task_count(goals, timeline.0 - timeline.1).clone();

		goals_occurrences
			.iter()
			.filter(|(_, g)| g.time_constraint.is_some())
			.enumerate()
			.try_for_each(|(idx, (task_count_a, goal_self))| -> Result<(), String> {
				let task_allocation_a = goal_self.duration / *task_count_a as f32;

				// Iterate
				goals_occurrences_copy
					.iter()
					.filter(|(_, g)| g.time_constraint.is_some())
					.enumerate()
					.try_for_each(|(t_idx, (task_count_b, goal_other))| {
						let task_allocation_b = goal_other.duration / *task_count_b as f32;

						// This prevents checking conflicts with self
						if t_idx == idx {
							return Ok(());
						}

						// TWO goals intersect if their time constraints are within range
						if goal_self.intersects(goal_other, task_allocation_a, task_allocation_b) {
							return Err(format!(
							"Two goals: (description = {}) and (description = {}) are conflicting as they intersect",
							goal_other.description, goal_self.description
						));
						};

						Ok(())
					})?;

				Ok(())
			})?;

		// =================== INSERT TASKS INTO TIME SLOTS ===================
		goals_occurrences
			.iter()
			.for_each(|(task_count, goal)| insert_tasks(goal, *task_count, &mut slots));

		Ok(Schedule { slots, timeline })
	}
}

pub(self) fn insert_tasks(goal: &Goal, task_count: usize, time_slots: &mut LinkedList<ScheduleSlot>) {
	// Amount of time allocated to each task
	let task_allocation = goal.duration / task_count as f32;

	// The first slot containing free time
	let mut current_start;
	let (_, (free_time_start, _)) = compatible_slot(time_slots, task_allocation, None);
	if let Some(time) = goal.time_constraint {
		current_start = free_time_start.replace_time(time);
	} else {
		current_start = free_time_start.clone();
	};

	let tasks = (0..task_count)
		.map(|_| {
			let task = Task {
				goal_id: goal.id,
				start: current_start,
				finish: current_start + task_allocation,
			};

			current_start += goal.interval;
			task
		})
		.collect::<Vec<_>>();

	for task in tasks {
		let (idx, (_, free_end)) = compatible_slot(time_slots, task_allocation, Some(task.start));
		let free_end_copy = free_end.clone();

		// This free time now ends here
		*free_end = task.start;

		// Create new splinter free slot
		let free_slot = ScheduleSlot::Free((task.finish, free_end_copy));

		// Insert occupied slot
		time_slots.insert(idx, ScheduleSlot::Occupied(task));

		// Insert free slot
		time_slots.insert(idx + 1, free_slot);
	}
}

fn compatible_slot(
	time_slots: &mut LinkedList<ScheduleSlot>,
	task_alloc: Duration,
	instant: Option<time::PrimitiveDateTime>,
) -> (usize, &mut (PrimitiveDateTime, PrimitiveDateTime)) {
	let slot = time_slots
		.iter_mut()
		.enumerate()
		.find(|(_, slot)| match slot {
			ScheduleSlot::Occupied(_) => false,
			ScheduleSlot::Free((lower, upper)) => {
				let can_fit = (*upper - *lower) >= task_alloc;
				let in_range = match instant {
					Some(instant) => *upper >= instant && instant >= *lower,
					None => true,
				};

				in_range && can_fit
			}
		})
		.unwrap();

	(
		slot.0,
		match slot.1 {
			ScheduleSlot::Free(time) => time,
			ScheduleSlot::Occupied(_) => unreachable!(),
		},
	)
}
