use crate::console;

pub struct ErrorCode;

pub trait ExplodeOption<T> {
	fn explode(self) -> T;
}

impl<T> ExplodeOption<T> for Option<T> {
	fn explode(self) -> T {
		self.unwrap_or_else(|| {
			console::log_err(
				ErrorCode::UnwrapError,
				format!("Call to .unwrap() panicked @ line: {}", line!()),
			)
		})
	}
}

extern "C" {
	pub fn exit(exit_code: u8, ipc_offset: usize) -> !;
}

#[allow(non_upper_case_globals)]
impl ErrorCode {
	pub(crate) const IPCDataOverflow: u8 = 2;
	pub(crate) const DeserializationError: u8 = 4;
	pub(crate) const SerializationError: u8 = 5;
	pub(crate) const UnwrapError: u8 = 6;
}
