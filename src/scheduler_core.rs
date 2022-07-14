//! The core-scheduler focusses on scheduling. It is referred to as 'core' to avoid confusion
//! with the scheduler (which is pre-processor + core).
//! It takes a flat list of tasks with 0-* corresponding time-constrained slots.
//! For each tasks it decides which of the possible positions within the corresponding slots is best.
//! The scheduler optimizes for the minimum amount of IMPOSSIBLE tasks.
//! https://github.com/tijlleenders/ZinZen-scheduler/wiki/Core

use serde::{Deserialize, Serialize};

use crate::task::TaskStatus::{IMPOSSIBLE, SCHEDULED, UNSCHEDULED};
use crate::task::{Slot, Task, TaskResult, TaskStatus};

#[derive(Debug)]
pub struct CoreScheduler {
	pub tasks: Vec<Task>,
	pub slots: Vec<Slot>,
	/// Tasks that have been processed. Initially empty
	processed_tasks: Vec<Task>,
}

#[derive(Serialize, Debug)]
pub struct SchedulerResult {
	pub tasks: Vec<TaskResult>,
	pub slots: Vec<Slot>,
}

impl CoreScheduler {
	pub fn new(tasks: Vec<Task>, slots: Vec<Slot>) -> Self {
		Self {
			tasks,
			slots,
			processed_tasks: vec![],
		}
	}

	fn get_slots(&self) -> &[Slot] {
		self.slots.as_ref()
	}

	fn calculate_flexibility(&mut self) {
		for task in self.tasks.iter_mut() {
			let id = task.id();
			for slot in self.slots.iter_mut().filter(|slot| slot.task_id == id) {
				task.flexibility += slot.end - slot.start;
			}
		}
	}

	fn find_overlap_number_for(&self, begin: usize, end: usize) -> usize {
		let mut result: usize = 0;
		for slot in self.slots.iter() {
			if slot.start < end && slot.end > begin {
				result += 1;
			}
		}
		result
	}

	fn find_least_requested_slot_for_task(&self, task: &Task) -> (usize, usize) {
		// lifted from old code, don't understand the impl yet
		let mut slot_with_least_requests: Option<(usize, usize)> = None;
		let mut lowest_number_of_requests_for_slot: Option<usize> = None;
		for slot in self.slots.iter() {
			if slot.task_id == task.id() {
				let num_windows_in_slot = (slot.end - slot.start + 1).checked_sub(task.get_duration_to_schedule());

				match num_windows_in_slot {
					None => continue,
					Some(_) => {}
				}

				for slot_offset in 0..num_windows_in_slot.unwrap() {
					let overlap = self.find_overlap_number_for(
						slot.start + slot_offset,
						slot.start + slot_offset + task.get_duration_to_schedule(),
					);

					match lowest_number_of_requests_for_slot {
						None => {
							lowest_number_of_requests_for_slot = Some(overlap);
							slot_with_least_requests = Some((
								slot.start + slot_offset,
								slot.start + slot_offset + task.get_duration_to_schedule(),
							))
						}
						Some(lowest_overlap) => {
							if overlap == 1 {
								return slot_with_least_requests.unwrap();
							}
							if overlap < lowest_overlap {
								slot_with_least_requests = Some((
									slot.start + slot_offset,
									slot.start + slot_offset + task.get_duration_to_schedule(),
								));
								lowest_number_of_requests_for_slot = Some(overlap);
							}
						}
					}
				}
			}
		}
		slot_with_least_requests.unwrap()
	}

	/// Schedule the given task to this slot,
	/// updating the other tasks and their slots as needed.
	fn do_schedule(&mut self, mut task: Task, scheduled_slot: Slot) {
		task.status = SCHEDULED;
		let task_id = task.id();

		self.processed_tasks.push(task);
		self.slots.retain(|slot| slot.task_id != task_id);

		// Remove the parts of slots from other tasks that overlap with scheduled slot
		let (cut_start, cut_end) = (scheduled_slot.start, scheduled_slot.end);

		self.slots = self.slots.iter().fold(vec![], |mut acc, slot| {
			if slot.start >= cut_start && slot.start < cut_end {
				// start
				acc.push(Slot {
					task_id: slot.task_id,
					start: cut_end,
					end: slot.end,
				});
			} else if scheduled_slot.start < cut_start && scheduled_slot.end > cut_end {
				// middle
				acc.push(Slot {
					task_id: slot.task_id,
					start: slot.start,
					end: cut_start,
				});
				acc.push(Slot {
					task_id: slot.task_id,
					start: cut_end,
					end: slot.end,
				});
			} else if scheduled_slot.start < cut_start && scheduled_slot.end > cut_start {
				// end
				acc.push(Slot {
					task_id: slot.task_id,
					start: slot.start,
					end: cut_start,
				});
			} else {
				// no cutoff, keep same slot
				// XXX cannot move so cloning
				acc.push(slot.clone());
			}
			acc
		});
		// Add the newly scheduled slot
		self.slots.push(scheduled_slot);
	}

	/// Schedule all the tasks.
	pub fn schedule(mut self) -> SchedulerResult {
		while !self.tasks.is_empty() {
			self.calculate_flexibility();
			// Tasks with flex 0 are unscheduled
			if let Some(i) = self.tasks.iter().position(|task| task.flexibility == 0) {
				let mut task = self.tasks.remove(i);
				task.status = IMPOSSIBLE;
				self.processed_tasks.push(task);
				continue;
			}

			// Tasks with min flex should be scheduled now
			if let Some(i) = self.tasks.iter().position(|task| task.flexibility == 1) {
				let mut task = self.tasks.remove(i);
				let slot_index = self
					.slots
					.iter_mut()
					.position(|slot| slot.task_id == task.id())
					.expect(&*format!(
						"Expected 1 slot available for flex 1 task {:?}, none found, scheduler={:?}",
						task, self
					));
				let slot = self.slots.remove(slot_index);

				self.do_schedule(task, slot);
				continue;
			}

			// Get the max flex task
			// TODO: sort the Vec instead
			self.tasks.sort_by_key(|x| x.flexibility);
			let task = self.tasks.pop().unwrap();

			// Find slot with least overlap
			let (start, end) = self.find_least_requested_slot_for_task(&task);
			let task_id = task.id();
			self.do_schedule(task, Slot { start, end, task_id });
		}

		self.processed_tasks.sort_by_key(|x| x.id());
		self.slots.sort_by_key(|x| x.task_id);
		SchedulerResult {
			tasks: self.processed_tasks.into_iter().map(|t| t.into_task_result()).collect(),
			slots: self.slots,
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::preprocessor::preprocess;
	use crate::Input;

	use super::*;

	#[test]
	fn basic_test() {
		let input: Input = serde_json::from_str(
			r#"
{
    "startDate": "2022-01-01T00:00:00Z",
    "endDate": "2022-01-02T00:00:00Z",
    "goals": [
        {
          "id": 1,
          "title" : "shopping",
          "duration": 1,
          "start": "2022-01-01T10:00:00Z",
          "deadline": "2022-01-01T13:00:00Z"
        },
        {
          "id": 2,
          "title": "dentist",
          "duration": 1,
          "start": "2022-01-01T10:00:00Z",
          "deadline": "2022-01-01T11:00:00Z"
        },
        {
          "id": 3,
          "title" : "exercise",
          "duration": 1,
          "start": "2022-01-01T10:00:00Z",
          "deadline": "2022-01-01T18:00:00Z"
        }
    ]
}
        "#,
		)
		.unwrap();

		let mut scheduler = preprocess(input);
		let result = scheduler.schedule();

		let result_json = serde_json::to_string_pretty(&result).unwrap();

		//print!("{}", result_json);
		assert_eq!(
			result_json,
			r#"{
  "tasks": [
    {
      "id": 0,
      "goal_id": 1,
      "duration_to_schedule": 1,
      "duration_scheduled": 0,
      "status": "SCHEDULED"
    },
    {
      "id": 1,
      "goal_id": 2,
      "duration_to_schedule": 1,
      "duration_scheduled": 0,
      "status": "SCHEDULED"
    },
    {
      "id": 2,
      "goal_id": 3,
      "duration_to_schedule": 1,
      "duration_scheduled": 0,
      "status": "SCHEDULED"
    }
  ],
  "slots": [
    {
      "task_id": 0,
      "start": 12,
      "end": 13
    },
    {
      "task_id": 1,
      "start": 10,
      "end": 11
    },
    {
      "task_id": 2,
      "start": 11,
      "end": 12
    }
  ]
}"#
		);
	}
}
