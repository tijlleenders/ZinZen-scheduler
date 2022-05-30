use crate::{
	error::{Explode, SchedulerError, SchedulerResult},
	goal::Goal,
	preprocessor::PreProcessor,
	task::Task,
};
use linked_list::LinkedList;
use time::{Duration, PrimitiveDateTime};

/// A schedule is just a list of tasks which satisfy the user's time and location constraints
#[derive(Debug)]
pub struct Schedule {
	pub(crate) slots: LinkedList<Task>,
	pub(crate) timeline: (PrimitiveDateTime, PrimitiveDateTime),
}

impl Schedule {
	/// Generate a vector of tasks from a linked-list of slots
	pub(crate) fn into_tasks_vector(&self) -> Vec<Task> {
		self.slots.iter().cloned().collect::<Vec<_>>()
	}
}

pub fn generate_schedule(
	goals: &[Goal],
	timeline: (PrimitiveDateTime, PrimitiveDateTime),
) -> SchedulerResult<Schedule> {
	let max_free_time = timeline.1 - timeline.0;

	// Slots initially begin with full free time
	let mut schedule = Schedule {
		slots: LinkedList::new(),
		timeline,
	};

	// Insert a task that spans the whole schedule, considered free time
	let free_task = Task::fill(&schedule);
	schedule.slots.push_front(free_task);

	// ======================= TIMELINE & GOAL CHECKS =================================
	// Make sure no Goal exceeds the user's free time
	goals.iter().try_for_each(|g| {
		if g.task_duration >= max_free_time {
			return Err(SchedulerError::GoalTaskDurationOverflow(g.id.to_string()));
		}

		Ok(())
	})?;

	// Produce a tuple containing task count and goal, and insert into time slots
	let mut tasks = PreProcessor::process_task_count(goals, timeline);
	tasks.try_for_each(|(task_count, goal)| insert_tasks(goal, task_count, &mut schedule))?;

	Ok(schedule)
}

pub(self) fn insert_tasks(goal: &Goal, task_count: usize, schedule: &mut Schedule) -> SchedulerResult {
	// The first compatible slot
	let mut next_interval = match goal.start {
		Some(goal_start) => goal_start,
		None => schedule.timeline.0,
	};

	// Insert the relevant number of tasks into the time slot
	(0..task_count).try_for_each(|_| {
		// Get's the first compatible slot
		// Get mutable reference to the Task allocated in the schedule
		let (idx, task_allocated) = if goal.start.is_some() {
			compatible_slot(schedule, goal, Hint::Exact(next_interval))?
		} else {
			compatible_slot(schedule, goal, Hint::Range(next_interval, goal.interval))?
		};

		// Get time expected for task a and b
		let task_allocated_time = task_allocated.finish - task_allocated.start;
		let task_allocated_duration = task_allocated_time / task_allocated.flexibility;

		// Store end_time copy
		let end_time = task_allocated.finish;

		// MATHEMATICS!
		let divide_time = match goal.start {
			Some(_) => next_interval,
			None => {
				let denominator = task_allocated_duration + goal.task_duration;
				let ratio = task_allocated_duration / denominator;
				let task_allocated_new_time = task_allocated_time * ratio;

				task_allocated.start + task_allocated_new_time
			}
		};

		// The allocated time now ends here
		task_allocated.finish = divide_time;
		task_allocated.flexibility = (task_allocated.finish - task_allocated.start) / task_allocated_duration;

		// When does this task start
		let mut start = next_interval;

		// Remove free task
		if task_allocated.goal_id == 0 {
			start = task_allocated.start;
			schedule.slots.remove(idx).explode();
		}

		// Create new splinter free slot
		let new_allocated = Task {
			goal_id: goal.id.get(),
			start,
			finish: end_time,
			flexibility: (end_time - start) / goal.task_duration,
		};

		// Insert newly allocated task
		schedule.slots.insert(idx, new_allocated);

		// Increment time_hint
		if let Some(interval) = goal.interval {
			next_interval += interval
		};

		Ok(())
	})
}

#[derive(Copy, Clone, Debug)]
pub(self) enum Hint {
	Range(time::PrimitiveDateTime, Option<Duration>),
	/// Means the time has to start exactly at the given time
	Exact(time::PrimitiveDateTime),
}

/// Returns an index to the first slot compatible with the given constraints
fn compatible_slot<'a>(
	schedule: &'a mut Schedule,
	goal: &'a Goal,
	start_hint: Hint,
) -> SchedulerResult<(usize, &'a mut Task)> {
	#[allow(clippy::or_fun_call)]
	schedule
		.slots
		.iter_mut()
		.enumerate()
		.find(|(_, task)| {
			let task_space = task.finish - task.start;

			// Can we fit into the this slot?
			let can_fit = {
				let min_remainder = task_space - goal.task_duration;
				task_space / task.flexibility <= min_remainder
			};

			// Are we in range of the time hint?
			let in_range = match start_hint {
				Hint::Range(start_time, range) => match range {
					Some(delta) => {
						let lower = start_time >= task.start;
						let upper = start_time + delta >= task.start;
						lower && upper
					}
					// If there is no interval, it means this is a singular event and can thus be placed anywhere on the timeline
					None => start_time >= task.start, // BUG: Hmmm!
				},

				Hint::Exact(start_time) => task.finish >= start_time && start_time >= task.start,
			};

			// Can we append ourselves into this tasks slots?
			let can_append = match start_hint {
				Hint::Exact(start_time) => {
					start_time - task.start >= task_space / task.flexibility
						&& task.finish - start_time >= goal.task_duration
				}
				Hint::Range(_, _) => true,
			};

			can_fit && in_range && can_append
		})
		.ok_or(SchedulerError::UnableToFindTaskSlot(goal.description.to_string()))
}
