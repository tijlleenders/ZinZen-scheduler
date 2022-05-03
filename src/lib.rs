use std::io::Read;

use goal::load_goals_from_ipc;
use preprocessor::PreProcessor;
use task::Tasks;

/// API modules
mod console;
mod error;

/// Tests
mod tests;

/// Project details
mod goal;
mod preprocessor;
mod task;

/// A buffer where IPC Data is written to by Rust and read from by Javascript
/// At any one moment, only one read and write is done to this buffer, `WASM` is a single-threaded runtime anyway
pub const IPC_BUFFER_SIZE: usize = 1024 * 64;
pub static mut IPC_BUFFER: [u8; IPC_BUFFER_SIZE] = [0; IPC_BUFFER_SIZE];

#[no_mangle]
pub unsafe extern "C" fn getDataPointer() -> *const u8 {
	IPC_BUFFER.as_ptr()
}

/// This writes some data to the IPC buffer, then returns a pointer and an offset to the data
pub(crate) fn write_to_ipc<R: Read>(mut source: R) {
	unsafe {
		if source.read(&mut IPC_BUFFER).is_err() {
			let data = b"Fatal Error! Unable to write to IPC";
			IPC_BUFFER[..data.len()].copy_from_slice(data);

			error::exit(error::ErrorCode::UnableToWriteToIPC, data.len());
		}
	}
}

#[no_mangle]
unsafe extern "C" fn processGoals(bytes: usize, time_in_hours: usize) {
	let goals = load_goals_from_ipc(bytes);

	let processed = PreProcessor::generate_tasks(&goals, time_in_hours);
	let tasks: Tasks = processed.as_slice().into();
	console::log_str(tasks.serialize_json())
}
