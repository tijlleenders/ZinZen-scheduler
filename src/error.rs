use crate::write_to_ipc;

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
		crate::_IPC_BUFFER_SIZE
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

/// Safe equivalent to unwrap that works in the wasm context and natively seamlessly
pub trait Explode<T> {
	fn explode(self) -> T;
	fn into_explosion(self) -> SchedulerResult<T>;
}

impl<T> Explode<T> for Option<T> {
	fn explode(self) -> T {
		#[cfg(target_arch = "wasm32")]
		{
			self.unwrap_or_else(|| unsafe {
				exit(
					ErrorCodes::UnwrapError,
					write_to_ipc("Call to `Option::unwrap` panicked").explode(),
				)
			})
		}
		#[cfg(not(target_arch = "wasm32"))]
		{
			self.unwrap()
		}
	}

	fn into_explosion(self) -> SchedulerResult<T> {
		match self {
			Some(data) => Ok(data),
			None => Err(SchedulerError::UnwrapError),
		}
	}
}

impl<T> Explode<T> for SchedulerResult<T> {
	fn explode(self) -> T {
		#[cfg(target_arch = "wasm32")]
		{
			use crate::{console, write_to_ipc};

			unsafe {
				match self {
					Ok(data) => data,
					Err(err) => match err {
						SchedulerError::UnwrapError => {
							exit(ErrorCodes::UnwrapError, write_to_ipc(err.to_string()).explode())
						}
						SchedulerError::IPCDataOverflow(_) => {
							exit(ErrorCodes::IPCDataOverflow, write_to_ipc(err.to_string()).explode())
						}
						SchedulerError::JSONError(_) => {
							exit(ErrorCodes::JSONError, write_to_ipc(err.to_string()).explode())
						}
						SchedulerError::GoalTaskDurationOverflow(_) => exit(
							ErrorCodes::GoalTaskDurationOverflow,
							write_to_ipc(err.to_string()).explode(),
						),
						SchedulerError::UnableToFindTaskSlot(_) => exit(
							ErrorCodes::UnableToFindTaskSlot,
							write_to_ipc(err.to_string()).explode(),
						),
					},
				}
			}
		}
		#[cfg(not(target_arch = "wasm32"))]
		{
			self.unwrap()
		}
	}

	fn into_explosion(self) -> SchedulerResult<T> {
		self
	}
}

impl<T> Explode<T> for Result<T, serde_json::Error> {
	fn explode(self) -> T {
		self.into_explosion().explode()
	}

	fn into_explosion(self) -> SchedulerResult<T> {
		match self {
			Ok(data) => Ok(data),
			Err(err) => Err(SchedulerError::JSONError(err)),
		}
	}
}
