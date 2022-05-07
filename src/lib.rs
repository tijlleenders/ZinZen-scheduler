use error::ErrorCode;
use goal::load_goals_from_ipc;
use preprocessor::PreProcessor;
use time::Duration;

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
pub(crate) fn write_to_ipc<S: AsRef<[u8]>>(buf: S) -> usize {
	let data = buf.as_ref();

	unsafe {
		if data.len() >= _IPC_BUFFER_SIZE {
			let error_msg: &[u8] = b"The length of data to be logged to the console exceeds the size of the IPC_BUFFER";

			IPC_BUFFER[..error_msg.len()].copy_from_slice(error_msg);
			error::exit(error::ErrorCode::IPCDataOverflow, error_msg.len())
		};

		IPC_BUFFER[..data.len()].copy_from_slice(data);
		data.len()
	}
}

#[no_mangle]
unsafe extern fn preProcessGoals(bytes: usize, time_in_hours: i64) {
	let goals = load_goals_from_ipc(bytes);

	let processed = PreProcessor::process_task_count(&goals, Duration::hours(time_in_hours));
	let string =
		serde_json::to_string(&processed).unwrap_or_else(|err| console::log_err(ErrorCode::SerializationError, err));

	console::log_str(string)
}
