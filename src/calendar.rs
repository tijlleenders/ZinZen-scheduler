use serde::{Deserialize, Serialize}; // consider https://crates.io/crates/serde-wasm-bindgen
use std::{fmt, usize};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use web_sys::console;

use crate::{
	slot::Slot,
	tasks::{CutOffType, Task, TaskStatus},
};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Calendar {
	pub max_time_units: usize,
	pub time_unit_qualifier: String,
	pub tasks: Vec<Task>,
	pub slots: Vec<Slot>,
}

impl Calendar {
	#[cfg(target_arch = "wasm32")]
	#[wasm_bindgen]
	pub fn load(val: &JsValue) -> JsValue {
		let mut calendar: Calendar = val.into_serde().unwrap();
		console::log_2(&"Called load_calendar with:".into(), &val);
		calendar.schedule();
		JsValue::from_serde(&calendar).unwrap()
	}

	pub fn new(max_time_units: usize, time_unit_qualifier: String) -> Calendar {
		Calendar {
			max_time_units,
			time_unit_qualifier,
			tasks: Vec::new(),
			slots: Vec::new(),
		}
	}

	pub fn schedule(&mut self) {
		#[cfg(not(target_arch = "wasm32"))]
		log::info!("Calendar after loading:{:#?}\n", self);

		loop {
			let unscheduled_task_index_with_highest_scheduling_possibilities: Option<usize> =
				self.find_unscheduled_task_index_with_highest_scheduling_possibilities();
			match unscheduled_task_index_with_highest_scheduling_possibilities {
				Some(task_index_to_schedule) => {
					let least_overlap_interval: Option<(usize, usize)> =
						self.find_least_requested_slot_for_task(&self.tasks[task_index_to_schedule]);
					//log::info!(
					//     "least overlap for task_id {}:{}-{}\n",
					//     task_id_to_schedule,
					//     least_overlap_interval.0,
					//     least_overlap_interval.1
					// );
					match least_overlap_interval {
						None => {
							self.slots.retain(|slot| {
								let delete = { slot.task_id == self.tasks[task_index_to_schedule].task_id };
								!delete
							});
							self.tasks[task_index_to_schedule].task_status = TaskStatus::Impossible;
						}
						Some(interval) => {
							self.schedule_task(task_index_to_schedule, interval.0, interval.1);
						}
					}
				}
				None => break,
			}
		}
		self.slots.sort_by(|a, b| a.begin.cmp(&b.begin));
		#[cfg(not(target_arch = "wasm32"))]
		log::info!("Calendar after scheduling:{:#?}\n", self);
	}

	fn schedule_task(&mut self, task_index: usize, begin: usize, end: usize) {
		let task_id = self.tasks[task_index].task_id;
		#[cfg(not(target_arch = "wasm32"))]
		log::info!("Scheduling task_id {}.\n", task_id);

		#[cfg(not(target_arch = "wasm32"))]
		log::info!("task {:#?}.\n", self.tasks[task_index]);

		//Todo: only remove all slots if duration to be scheduled has been exhausted
		self.slots.retain(|slot| {
			let delete = { slot.task_id == task_id };
			!delete
		});
		let scheduled_slot = Slot { task_id, begin, end };
		let mut new_slots: Vec<Slot> = Vec::with_capacity(self.slots.capacity());
		for slot in self.slots.iter() {
			let cut_off_type = Calendar::find_cut_off_type(slot, begin, end);
			match cut_off_type {
				CutOffType::CutStart => {
					if end != slot.end {
						new_slots.push(Slot {
							task_id: slot.task_id,
							begin: end,
							end: slot.end,
						});
					}
				}
				CutOffType::CutMiddle => {
					if slot.begin != begin {
						new_slots.push(Slot {
							task_id: slot.task_id,
							begin: slot.begin,
							end: begin,
						});
					}
					if end != slot.end {
						new_slots.push(Slot {
							task_id: slot.task_id,
							begin: end,
							end: slot.end,
						});
					}
				}
				CutOffType::CutEnd => {
					if slot.begin != begin {
						new_slots.push(Slot {
							task_id: slot.task_id,
							begin: slot.begin,
							end: begin,
						});
					}
				}
				CutOffType::NoCut => {
					new_slots.push(Slot {
						task_id: slot.task_id,
						begin: slot.begin,
						end: slot.end,
					});
				}
				CutOffType::CutWhole => {}
			}
		}
		self.slots = new_slots;
		self.slots.push(scheduled_slot);
		self.tasks[task_index].task_status = TaskStatus::Scheduled;
		//Todo: if scheduled and 'before' some other task, remove TaskStatus::WAITING from the other tasks that have 'following' attribute
		//log::info!(
		//     "Calendar right after scheduling task_id {}:{:#?}\n",
		//     task_id,
		//     self
		// );
	}

	pub fn find_cut_off_type(slot: &Slot, begin: usize, end: usize) -> CutOffType {
		if slot.begin >= begin && slot.begin < end {
			return CutOffType::CutStart;
		}
		if slot.begin < begin && slot.end > end {
			return CutOffType::CutMiddle;
		}
		if slot.begin < begin && slot.end > begin {
			return CutOffType::CutEnd;
		}
		if slot.begin >= begin && slot.end <= end {
			return CutOffType::CutWhole;
		}
		CutOffType::NoCut
	}

	pub fn print_slots_for_range(&self, start: usize, finish: usize) {
		for slot in self.slots.iter() {
			if slot.end > start && slot.begin < finish {
				// log::info!["found for {}..{}: {:#?}\n", start, finish, slot];
			}
		}
	}

	fn find_least_requested_slot_for_task(&self, task: &Task) -> Option<(usize, usize)> {
		#[cfg(not(target_arch = "wasm32"))]
		log::info!(
			"Finding number of requests for possible slot for task_id:{}\n",
			task.task_id
		);
		let mut slot_with_least_requests: Option<(usize, usize)> = None;
		let mut lowest_number_of_requests_for_slot: Option<usize> = None;
		for slot in self.slots.iter() {
			if slot.task_id == task.task_id {
				let num_windows_in_slot = (slot.end - slot.begin + 1).checked_sub(task.duration_to_schedule);
				#[cfg(not(target_arch = "wasm32"))]
				log::info!("num_windows_in_slot:{:#?}\n", num_windows_in_slot);
				match num_windows_in_slot {
					None => continue,
					Some(_) => {}
				}

				for slot_offset in 0..num_windows_in_slot.unwrap() {
					let overlap = self.find_overlap_number_for(
						slot.begin + slot_offset,
						slot.begin + slot_offset + task.duration_to_schedule,
					);
					#[cfg(not(target_arch = "wasm32"))]
					log::info!(
						"# overlaps for:{}-{}:{}\n",
						slot.begin + slot_offset,
						slot.begin + slot_offset + task.duration_to_schedule,
						overlap
					);
					match lowest_number_of_requests_for_slot {
						None => {
							lowest_number_of_requests_for_slot = Some(overlap);
							slot_with_least_requests = Some((
								slot.begin + slot_offset,
								slot.begin + slot_offset + task.duration_to_schedule,
							))
						}
						Some(lowest_overlap) => {
							if overlap == 1 {
								return slot_with_least_requests;
							}
							if overlap < lowest_overlap {
								slot_with_least_requests = Some((
									slot.begin + slot_offset,
									slot.begin + slot_offset + task.duration_to_schedule,
								));
								lowest_number_of_requests_for_slot = Some(overlap);
							}
						}
					}
				}
			}
		}
		slot_with_least_requests
	}

	fn find_overlap_number_for(&self, begin: usize, end: usize) -> usize {
		let mut result: usize = 0;
		for slot in self.slots.iter() {
			if slot.begin < end && slot.end > begin {
				result += 1;
			}
		}
		result
	}

	fn find_unscheduled_task_index_with_highest_scheduling_possibilities(&self) -> Option<usize> {
		#[cfg(not(target_arch = "wasm32"))]
		log::info!("Searching for new task to process...\n");

		let mut task_id_highest_scheduling_possibilities_prio: Option<usize> = None;
		let mut highest_scheduling_possibilities_so_far: usize = 0;
		for (task_index, task) in self.tasks.iter().enumerate() {
			match task.task_status {
				TaskStatus::Impossible => {
					continue;
				}
				TaskStatus::Scheduled => {
					continue;
				}
				TaskStatus::Waiting => {
					continue;
				}
				TaskStatus::Unscheduled => {
					//Todo: skip if following another task
					let mut scheduling_possibilities: usize = 0;
					for slot in self.slots.iter() {
						if slot.task_id == task.task_id {
							let range: usize = slot.end - slot.begin;

							#[cfg(not(target_arch = "wasm32"))]
							log::info!("scheduling_possibilities before:{}\n", scheduling_possibilities);

							let num_to_add_option = range.checked_sub(task.duration_to_schedule);
							match num_to_add_option {
								Some(num) => scheduling_possibilities += num,
								None => {
									#[cfg(not(target_arch = "wasm32"))]
									log::info!("task duration cannot be subtracted from slot.end-slot.begin")
								}
							}

							#[cfg(not(target_arch = "wasm32"))]
							log::info!("scheduling_possibilities after:{}\n", scheduling_possibilities);
						}
					}

					match task_id_highest_scheduling_possibilities_prio {
						Some(highest_num) => {
							if scheduling_possibilities > highest_num {
								#[cfg(not(target_arch = "wasm32"))]
								log::info![
                                "Found task {} with scheduling_possibilities {}...higher than previous task {:#?} with {:#?}\n",
                                task_index, scheduling_possibilities, task_id_highest_scheduling_possibilities_prio, highest_scheduling_possibilities_so_far
                                ];
								task_id_highest_scheduling_possibilities_prio = Some(task_index);
								highest_scheduling_possibilities_so_far = scheduling_possibilities;
							}
						}
						None => {
							task_id_highest_scheduling_possibilities_prio = Some(task_index);
							highest_scheduling_possibilities_so_far = scheduling_possibilities;
						}
					}
				}
			}
		}
		task_id_highest_scheduling_possibilities_prio
	}
}

impl fmt::Display for Calendar {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Calendar:\nTasks:{:#?}\nSlots:{:#?}\n", self.tasks, self.slots)
	}
}
