pub struct ErrorCode;

pub trait Explode<T> {
	fn explode(self) -> T;
}

impl<T> Explode<T> for Option<T> {
	fn explode(self) -> T {
		#[cfg(target = "wasm32-unknown-unknown")]
		{
			use crate::console;
			self.unwrap_or_else(|| console::log_err(ErrorCode::UnwrapError, "Call to `Option::unwrap` panicked"))
		}
		#[cfg(not(target = "wasm32-unknown-unknown"))]
		{
			self.unwrap()
		}
	}
}

impl<T, E: std::fmt::Debug> Explode<T> for Result<T, E> {
	fn explode(self) -> T {
		#[cfg(target = "wasm32-unknown-unknown")]
		{
			use crate::console;
			self.unwrap_or_else(|err| console::log_err(ErrorCode::UnwrapError, err))
		}
		#[cfg(not(target = "wasm32-unknown-unknown"))]
		{
			self.unwrap()
		}
	}
}

extern "C" {
	pub fn exit(exit_code: u8, ipc_offset: usize) -> !;
}

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
impl ErrorCode {
	pub(crate) const IPCDataOverflow: u8 = 2;
	pub(crate) const DeserializationError: u8 = 4;
	pub(crate) const SerializationError: u8 = 5;
	pub(crate) const UnwrapError: u8 = 6;
}
