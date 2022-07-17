#[cfg(target_arch = "wasm32")]
extern "C" {
	/// Quit's the current wasm execution, and returns an error code as well as some info as a string in the IPC
	pub(self) fn exit(exit_code: u8, ipc_offset: usize) -> !;
}

/// Various error codes
pub struct ErrorCodes;

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
impl ErrorCodes {
	pub(crate) const IPCDataOverflow: u8 = 2;
	pub(crate) const JSONError: u8 = 5;
	pub(crate) const UnwrapError: u8 = 6;
	pub(crate) const GoalTaskDurationOverflow: u8 = 7;
	pub(crate) const UnableToFindTaskSlot: u8 = 8;
}

#[allow(dead_code)]
#[derive(thiserror::Error, Debug)]
pub enum SchedulerError {
	/// Called unwrap on a None value
	#[error("Called `Option::unwrap(---)` on a None value")]
	UnwrapError,
	/// Too much data written to IPC
	#[error(
		"The maximum length of data in the IPC buffer is {0} bytes, instead found {} bytes which is excess",
		1
	)]
	IPCDataOverflow(usize),
	/// Unable to serialize data
	#[error("Unable to parse data as valid json: {0}")]
	JSONError(serde_json::error::Error),
	/// A goal with excessive time was found
	#[error("A goal (description = {0}) was found with a duration greater than the timeline duration, and therefore cannot fit")]
	GoalTaskDurationOverflow(String),
	/// The Scheduler was unable to find a slot for a given task
	#[error("The Scheduler was unable to find a slot for a given goal (description = {0})")]
	UnableToFindTaskSlot(String),
}

/// Internal error type used by crate
pub(crate) type SchedulerResult<T = ()> = Result<T, SchedulerError>;

// Safe equivalent to unwrap that works in the wasm context and natively seamlessly
// TODO use something like https://github.com/rustwasm/console_error_panic_hook
