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

/// A 64 KiB buffer for communication between Rust and JavaScript
/// At any one moment, only one read and write is done to this buffer, `WASM` is a single-threaded runtime anyway
pub const IPC_BUFFER_SIZE: usize = 1024 * 64;
pub static mut IPC_BUFFER: [u8; IPC_BUFFER_SIZE] = [0; IPC_BUFFER_SIZE];

#[no_mangle]
pub unsafe extern "C" fn getDataPointer() -> *const u8 {
	IPC_BUFFER.as_ptr()
}

/// This writes some data to the IPC buffer, then returns a pointer and an offset to the data
pub(crate) fn write_to_ipc<S: AsRef<[u8]>>(source: S) -> usize {
	let data = source.as_ref();

	unsafe {
		if data.len() >= IPC_BUFFER_SIZE {
			let error_msg: &[u8] = b"The length of data to be logged to the console exceeds the size of the IPC_BUFFER";

			IPC_BUFFER[..error_msg.len()].copy_from_slice(error_msg);
			error::exit(error::ErrorCode::IPCDataOverflow, error_msg.len())
		};

		IPC_BUFFER[..data.len()].copy_from_slice(data);
		data.len()
	}
}

#[no_mangle]
unsafe extern "C" fn processGoals(bytes: usize, time_in_hours: usize) {
	let goals = load_goals_from_ipc(bytes);

	let processed = PreProcessor::generate_tasks(&goals, time_in_hours);
	let tasks: Tasks = processed.as_slice().into();
	console::log_str(tasks.serialize_json())
}
