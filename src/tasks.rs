use serde::{Deserialize, Serialize}; // consider https://crates.io/crates/serde-wasm-bindgen
use std::{fmt, usize};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use web_sys::console;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Task {
	pub(crate) task_id: usize,
	pub(crate) duration_to_schedule: usize,
	pub(crate) duration_scheduled: usize,
	pub(crate) task_status: TaskStatus,
	pub(crate) goal_id: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum TaskStatus {
	UNSCHEDULED,
	SCHEDULED,
	IMPOSSIBLE,
	WAITING,
}

#[derive(Debug, PartialEq)]
pub enum CutOffType {
	NOCUT,
	CUTSTART,
	CUTEND,
	CUTMIDDLE,
	CUTWHOLE,
}

pub struct ParseGoalError;

impl fmt::Display for ParseGoalError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str("Not a valid string to construct a Goal from\n")
	}
}
