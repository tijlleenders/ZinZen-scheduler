use crate::{error::Explode, goal::Goal, preprocessor::PreProcessor, task::Task};
use linked_list::LinkedList;
use time::{Duration, PrimitiveDateTime};

/// A schedule is just a list of tasks which satisfy the user's time and location constraints
#[derive(Debug)]
pub struct Schedule {
	pub(crate) slots: LinkedList<Task>,
	pub(crate) timeline: (PrimitiveDateTime, PrimitiveDateTime),
}

impl Schedule {
	/// Generate a vector of slots from a list of slots
	pub(crate) fn slots_vector(&self) -> Vec<Task> {
		self.slots.iter().map(|t| t.clone()).collect::<Vec<_>>()
	}
}

pub fn generate_schedule(
	goals: &[Goal],
	timeline: (PrimitiveDateTime, PrimitiveDateTime),
) -> Result<Schedule, &'static str> {
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
	if goals.iter().any(|g| g.task_duration >= max_free_time) {
		return Err("A goal was found with a duration greater than the timeline duration");
	};

	// Make sure the user's free time is enough to accommodate all goal's durations
	let total_goal_duration = goals.iter().map(|g| g.task_duration).reduce(|a, b| a + b);

	if let Some(total) = total_goal_duration {
		if total >= max_free_time {
			return Err("There isn't enough time in the user's schedule to accommodate all Goal's, either increase your expected timeline or reduce your individual Goal's allocated time");
		}

		// If the user allocates no time to any Goal, then all time is free time :)
		if total == Duration::ZERO {
			return Ok(schedule);
		}
	} else {
		// If the user has no goals then they have all free time
		return Ok(schedule);
	};

	// Produce a tuple containing task count and goal, and insert into time slots
	PreProcessor::process_task_count(goals, timeline).for_each(|(task_count, goal)| {
		insert_tasks(goal, task_count, &mut schedule);
	});

	Ok(schedule)
}

pub(self) fn insert_tasks(goal: &Goal, task_count: usize, schedule: &mut Schedule) {
	// The first compatible slot
	let mut current_time_hint = match goal.start {
		Some(goal_start) => goal_start,
		None => schedule.timeline.0,
	};

	// Insert the relevant number of tasks into the time slot
	(0..task_count).for_each(|_| {
		// Get's the first compatible slot
		// Get mutable reference to the Task allocated in the schedule
		let (idx, task_allocated) = if goal.start.is_some() {
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

		// Get time expected for task a and b
		let task_allocated_time = task_allocated.finish - task_allocated.start;
		let task_allocated_duration = task_allocated_time / task_allocated.flexibility;

		// Store end_time copy
		let end_time = task_allocated.finish;

		// The allocated time now ends here
		task_allocated.finish = current_time_hint;
		task_allocated.flexibility = (task_allocated.finish - task_allocated.start) / task_allocated_duration;

		// When does this task start
		let mut start = current_time_hint;

		// Remove allocated task if no time is allocated
		if task_allocated.finish - task_allocated.start <= Duration::SECOND || task_allocated.goal_id == 0 {
			start = task_allocated.start;
			schedule.slots.remove(idx);
		}

		// Create new splinter free slot
		let new_allocated = Task {
			goal_id: goal.id.get(),
			start,
			finish: end_time,
			flexibility: (end_time - current_time_hint) / goal.task_duration,
		};

		// Insert newly allocated task
		schedule.slots.insert(idx, new_allocated);

		// Increment time_hint
		if let Some(interval) = goal.interval {
			current_time_hint += interval
		};
	});
}

#[derive(Copy, Clone, Debug)]
pub(self) enum Hint {
	Loose(time::PrimitiveDateTime),
	/// Means the time has to start exactly at the given time
	Exact(time::PrimitiveDateTime),
}

/// Returns an index to the first slot compatible with the given constraints
fn compatible_slot(
	schedule: &mut Schedule,
	task_duration: Duration,
	start_hint: Hint,
	maximum_delta: Option<Duration>,
) -> (usize, &mut Task) {
	schedule
		.slots
		.iter_mut()
		.enumerate()
		.find(|(_, task)| {
			let task_space = task.finish - task.start;

			// Can we fit into the this slot?
			let can_fit = task_space >= task_duration;

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
			// Goal ID 0 is considered free space
			let can_append = task.goal_id == 0
				|| match start_hint {
					Hint::Exact(start_time) => {
						start_time - task.start >= task_space / task.flexibility
							&& task.finish - start_time >= task_duration
					}
					Hint::Loose(_) => true,
				};

			can_fit && can_append && in_range
		})
		.ok_or("Unable to find slot for Task")
		.explode()
}
