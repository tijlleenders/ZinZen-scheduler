#![allow(unused)]

use crate::{goal::Goal, preprocessor::PreProcessor, task::Task};
use linked_list::LinkedList;
use time::{Duration, PrimitiveDateTime};

/// A schedule is just a list of tasks which satisfy the user's time and location constraints
#[derive(Debug)]
pub struct Schedule {
	pub(crate) slots: LinkedList<Task>,
	pub(crate) timeline: (PrimitiveDateTime, PrimitiveDateTime),
}

impl Schedule {
	pub fn generate_schedule(
		goals: &[Goal],
		timeline: (PrimitiveDateTime, PrimitiveDateTime),
	) -> Result<Schedule, String> {
		let max_free_time = timeline.1 - timeline.0;

		// Slots initially begin with full free time
		let mut schedule = Schedule {
			slots: LinkedList::new(),
			timeline,
		};

		// Insert a task that spans the whole schedule, considered free time
		let free_task = Task::fill(&mut schedule);
		schedule.slots.push_front(free_task);

		// ======================= TIMELINE & GOAL CHECKS =================================
		// Make sure no Goal exceeds the user's free time
		goals.iter().try_for_each(|g| {
			if g.task_duration >= max_free_time {
				Err(format!(
					"A goal (description = {})was found with a duration greater than the timeline duration",
					g.description
				))
			} else {
				Ok(())
			}
		})?;

		// Make sure the user's free time is enough to accommodate all goal's durations
		let total_goal_duration = goals.iter().map(|g| g.task_duration).reduce(|a, b| a + b);

		if let Some(total) = total_goal_duration {
			if total >= max_free_time {
				return Err("There isn't enough time in the user's schedule to accommodate all Goal's, either increase your expected timeline or reduce your individual Goal's allocated time".into());
			}

			// If the user allocates no time to any Goal, then all time is free time :)
			if total == Duration::ZERO {
				return Ok(schedule);
			}
		} else {
			// If the user has no goals then they have all free time
			return Ok(schedule);
		};

		// ========================= CHECK AND VALIDATE TIME_CONSTRAINT BOUNDS =================
		// Produce a tuple containing task count and goal
		let goals_occurrences = PreProcessor::process_task_count(goals, timeline.1 - timeline.0);

		// =================== INSERT TASKS INTO TIME SLOTS ===================
		goals_occurrences
			.iter()
			.for_each(|(task_count, goal)| insert_tasks(goal, *task_count, &mut schedule));

		Ok(schedule)
	}
}

pub(self) fn insert_tasks(goal: &Goal, task_count: f64, schedule: &mut Schedule) {
	// The first compatible slot
	let mut current_time_hint = match goal.time_constraint {
		Some(goal_start) => goal_start,
		None => schedule.timeline.0,
	};

	// Insert the relevant number of tasks into the time slot
	for _ in 0..(task_count as u32) {
		// Get's the first compatible slot
		let slot = if goal.time_constraint.is_some() {
			compatible_slot(
				schedule,
				goal.task_duration,
				Hint::Exact(current_time_hint),
				goal.interval,
			)
		} else {
			compatible_slot(
				schedule,
				goal.task_duration,
				Hint::Loose(current_time_hint),
				goal.interval,
			)
		};

		// Get mutable reference to the Task allocated in the schedule
		let task_allocated = get(&mut schedule.slots, slot).unwrap();

		// Get time expected for task a and b
		let task_allocated_time = task_allocated.finish - task_allocated.start;
		let task_allocated_duration = task_allocated_time / task_allocated.flexibility;

		// Store end_time copy
		let end_time = task_allocated.finish;

		// The allocated time now ends here
		task_allocated.finish = current_time_hint;
		task_allocated.flexibility = (task_allocated.finish - task_allocated.start) / task_allocated_duration;

		// Remove allocated task if no time is allocated
		if task_allocated.finish - task_allocated.start <= Duration::ZERO {
			schedule.slots.remove(slot);
		}

		// Create new splinter free slot
		let new_allocated = Task {
			goal_id: goal.id.get(),
			start: current_time_hint,
			finish: end_time,
			flexibility: (end_time - current_time_hint) / goal.task_duration,
		};

		// Insert newly allocated task
		schedule.slots.insert(slot, new_allocated);

		// Increment time_hint
		if let Some(interval) = goal.interval {
			current_time_hint += interval
		};
	}
}

#[derive(Copy, Clone, Debug)]
pub(self) enum Hint {
	Loose(time::PrimitiveDateTime),
	/// Means the time has to start exactly at the given time
	Exact(time::PrimitiveDateTime),
}

fn get<T>(list: &mut LinkedList<T>, index: usize) -> Option<&mut T> {
	for (cmp, item) in list.iter_mut().enumerate() {
		if cmp == index {
			return Some(item);
		}
	}

	None
}

/// Returns an index to the first slot compatible with the given constraints
fn compatible_slot(
	schedule: &Schedule,
	task_duration: Duration,
	start_hint: Hint,
	maximum_delta: Option<Duration>,
) -> usize {
	schedule
		.slots
		.iter()
		.enumerate()
		.find(|(_, task)| {
			let space = task.finish - task.start;

			// Can we fit into the this slot?
			let can_fit = space >= task_duration;

			// Are we in range of the time hint?
			let in_range = match start_hint {
				// If there is no max delta, means the Task can be placed anywhere
				Hint::Loose(start_time) => match maximum_delta {
					Some(delta) => (start_time - task.start <= delta) || (task.finish - start_time <= delta),
					None => true,
				},

				Hint::Exact(start_time) => task.finish >= start_time && start_time >= task.start,
			};

			// Can we append ourselves into this tasks? Slots?
			let can_append = match start_hint {
				Hint::Exact(start_time) => {
					start_time - task.start >= space / task.flexibility && task.finish - start_time >= task_duration
				}
				Hint::Loose(_) => true,
			};

			can_fit && can_append && in_range
		})
		.map(|d| d.0)
		.unwrap()
}
