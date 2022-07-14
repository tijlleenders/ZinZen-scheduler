use serde::Deserialize;
use time::serde::iso8601;
use time::{OffsetDateTime, PrimitiveDateTime};

use error::{Explode, SchedulerError, SchedulerResult};
use goal::load_goals_from_ipc;

/// API modules
mod console;
mod error;

/// Tests
mod tests;

/// Project details
mod goal;
mod preprocessor;
mod scheduler;
mod task;

// 64 Kib for the IPC
pub const _IPC_BUFFER_SIZE: usize = 1024 * 64;

/// A 64 KiB buffer for communication between Rust and JavaScrip
/// At any one moment, only one read and write is done to this buffer, `WASM` is a single-threaded runtime anyway
#[no_mangle]
pub static mut IPC_BUFFER: [u8; _IPC_BUFFER_SIZE] = [0; _IPC_BUFFER_SIZE];
#[no_mangle]
pub static IPC_BUFFER_SIZE: usize = _IPC_BUFFER_SIZE;

/// This writes some data to the IPC buffer, then returns a pointer and an offset to the data
pub(crate) fn write_to_ipc<S: AsRef<[u8]>>(buf: S) -> SchedulerResult<usize> {
	let data = buf.as_ref();

	unsafe {
		if data.len() >= _IPC_BUFFER_SIZE {
			return Err(SchedulerError::IPCDataOverflow(data.len()));
		};

		IPC_BUFFER[..data.len()].copy_from_slice(data);
		Ok(data.len())
	}
}

#[no_mangle]
unsafe extern "C" fn processTaskCount(bytes: usize) -> usize {
	// let (goals, timeline) = load_goals_from_ipc(bytes);
	// let processed = PreProcessor::preprocess_old(&goals, timeline);
	//
	// let with_ids = processed.map(|(a, b)| (a, b.id)).collect::<Vec<_>>();
	// let string = serde_json::to_string(&with_ids).explode();
	//
	// write_to_ipc(string).explode()
	0 // XXX: stub
}

#[derive(Deserialize, Debug)]
/// Just a deserialization target
pub struct Input {
	#[serde(rename = "startDate")]
	#[serde(with = "iso8601")]
	start: OffsetDateTime,
	#[serde(rename = "endDate")]
	#[serde(with = "iso8601")]
	end: OffsetDateTime,
	goals: Vec<goal::Goal>,
}

impl Input {
	unsafe fn load_plan_from_ipc(ipc_offset: usize) -> Self {
		let slice = &IPC_BUFFER[..ipc_offset];
		serde_json::from_slice(slice).explode()
	}
}

#[no_mangle]
unsafe extern "C" fn generateSchedule(bytes: usize) -> usize {
	// let Plan { goals, start, finish } = Plan::load_plan_from_ipc(bytes);
	//
	// // XXX: hack
	// //let schedule = scheduler::generate_schedule(&goals, (start, finish)).explode();
	// //let tasks = schedule.into_tasks_vector();
	// let string = serde_json::to_string(&vec!["a²"]).explode();
	//
	// write_to_ipc(string).explode()
	0 // XXX: stub
}
