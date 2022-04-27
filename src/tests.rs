#![cfg(test)]

use super::{calendar::*, slot::*, tasks::*};
fn init_env_logger() {
	#[cfg(not(target_arch = "wasm32"))]
	let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn calendar_with_daily_goal() {
	init_env_logger();

	let mut calendar = Calendar {
		max_time_units: 720,
		time_unit_qualifier: String::from("h"),
		tasks: Vec::new(),
		slots: Vec::new(),
	};

	let task1 = Task {
		duration_scheduled: 0,
		duration_to_schedule: 1,
		task_id: 1,
		task_status: TaskStatus::UNSCHEDULED,
		goal_id: "goal1".to_string(),
	};
	let task2 = Task {
		task_id: 2,
		duration_to_schedule: 1,
		duration_scheduled: 0,
		task_status: TaskStatus::UNSCHEDULED,
		goal_id: "goal1".to_string(),
	};
	let task3 = Task {
		task_id: 3,
		duration_to_schedule: 1,
		duration_scheduled: 0,
		task_status: TaskStatus::UNSCHEDULED,
		goal_id: "goal1".to_string(),
	};
	let slot1 = Slot {
		task_id: 1,
		begin: 6,
		end: 31,
	};
	let slot2 = Slot {
		task_id: 2,
		begin: 24,
		end: 49,
	};
	let slot3 = Slot {
		task_id: 3,
		begin: 48,
		end: 73,
	};

	calendar.tasks.push(task1);
	calendar.tasks.push(task2);
	calendar.tasks.push(task3);

	calendar.slots.push(slot1);
	calendar.slots.push(slot2);
	calendar.slots.push(slot3);

	#[cfg(not(target_arch = "wasm32"))]
	log::info!("Calendar:{:#?}\n", calendar);

	calendar.schedule();

	#[cfg(not(target_arch = "wasm32"))]
	log::info!("Calendar:{:#?}\n", calendar);

	assert_eq!(720, calendar.max_time_units);
	assert_eq!("h", calendar.time_unit_qualifier);
	let mut s_vec: Vec<Slot> = Vec::new();
	let expected_slot1 = Slot {
		task_id: 1,
		begin: 4,
		end: 5,
	};
	let expected_slot2 = Slot {
		task_id: 2,
		begin: 24,
		end: 25,
	};
	let expected_slot3 = Slot {
		task_id: 3,
		begin: 48,
		end: 49,
	};
	s_vec.push(expected_slot1);
	s_vec.push(expected_slot2);
	s_vec.push(expected_slot3);
	assert_eq!(s_vec, calendar.slots);
}

#[test]
fn two_goals() {
	init_env_logger();

	let mut calendar = Calendar {
		max_time_units: 720,
		time_unit_qualifier: String::from("h"),
		tasks: Vec::new(),
		slots: Vec::new(),
	};

	let task1 = Task {
		duration_scheduled: 0,
		duration_to_schedule: 1,
		task_id: 1,
		task_status: TaskStatus::UNSCHEDULED,
		goal_id: "5f39a726-641c-4c1a-aa54-2f28a3847ee8".to_string(),
	};
	let task2 = Task {
		task_id: 2,
		duration_to_schedule: 1,
		duration_scheduled: 0,
		task_status: TaskStatus::UNSCHEDULED,
		goal_id: "".to_string(),
	};

	let slot1 = Slot {
		task_id: 1,
		begin: 7,
		end: 720,
	};
	let slot2 = Slot {
		task_id: 2,
		begin: 7,
		end: 720,
	};

	calendar.tasks.push(task1);
	calendar.tasks.push(task2);

	calendar.slots.push(slot1);
	calendar.slots.push(slot2);

	#[cfg(not(target_arch = "wasm32"))]
	log::info!("Calendar:{:#?}\n", calendar);

	calendar.schedule();

	#[cfg(not(target_arch = "wasm32"))]
	log::info!("Calendar:{:#?}\n", calendar);

	assert_eq!(720, calendar.max_time_units);
	assert_eq!("h", calendar.time_unit_qualifier);
	let mut s_vec: Vec<Slot> = Vec::new();
	let expected_slot1 = Slot {
		task_id: 1,
		begin: 4,
		end: 5,
	};
	let expected_slot2 = Slot {
		task_id: 2,
		begin: 24,
		end: 25,
	};
	let expected_slot3 = Slot {
		task_id: 3,
		begin: 48,
		end: 49,
	};
	s_vec.push(expected_slot1);
	s_vec.push(expected_slot2);
	s_vec.push(expected_slot3);
	assert_eq!(s_vec, calendar.slots);
}

#[test]
fn at_subtract_overflow_panic() {
	init_env_logger();

	let mut calendar = Calendar {
		max_time_units: 168,
		time_unit_qualifier: String::from("h"),
		tasks: Vec::new(),
		slots: Vec::new(),
	};

	let task1 = Task {
		duration_scheduled: 0,
		duration_to_schedule: 1,
		task_id: 1,
		task_status: TaskStatus::UNSCHEDULED,
		goal_id: "5f39a726-641c-4c1a-aa54-2f28a3847ee8".to_string(),
	};

	let slot1 = Slot {
		task_id: 1,
		begin: 11,
		end: 12,
	};
	let slot2 = Slot {
		task_id: 1,
		begin: 35,
		end: 36,
	};

	calendar.tasks.push(task1);

	calendar.slots.push(slot1);
	calendar.slots.push(slot2);

	#[cfg(not(target_arch = "wasm32"))]
	log::info!("Calendar:{:#?}\n", calendar);

	calendar.schedule();

	#[cfg(not(target_arch = "wasm32"))]
	log::info!("Calendar:{:#?}\n", calendar);

	assert_eq!(168, calendar.max_time_units);
	assert_eq!("h", calendar.time_unit_qualifier);
	let mut s_vec: Vec<Slot> = Vec::new();
	let expected_slot1 = Slot {
		task_id: 1,
		begin: 11,
		end: 12,
	};
	s_vec.push(expected_slot1);
	assert_eq!(s_vec, calendar.slots);
}

#[test]
fn test_find_cut_off_type() {
	init_env_logger();
	let slot = Slot {
		task_id: 0,
		begin: 10,
		end: 15,
	};
	let cut_off = Calendar::find_cut_off_type(&slot, 0, 12);
	assert_eq!(cut_off, CutOffType::CUTSTART);

	let cut_off = Calendar::find_cut_off_type(&slot, 10, 12);
	assert_eq!(cut_off, CutOffType::CUTSTART);

	let cut_off = Calendar::find_cut_off_type(&slot, 11, 12);
	assert_eq!(cut_off, CutOffType::CUTMIDDLE);

	let cut_off = Calendar::find_cut_off_type(&slot, 11, 15);
	assert_eq!(cut_off, CutOffType::CUTEND);

	let cut_off = Calendar::find_cut_off_type(&slot, 11, 20);
	assert_eq!(cut_off, CutOffType::CUTEND);

	let cut_off = Calendar::find_cut_off_type(&slot, 20, 22);
	assert_eq!(cut_off, CutOffType::NOCUT);
}
