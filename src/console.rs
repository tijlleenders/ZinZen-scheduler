//! Console function implementations for `.wasm`

#![allow(dead_code)]
use crate::{error, write_to_ipc};

extern "C" {
	/// Imported logging facade from JavaScript
	fn console_log(is_string: bool, ipc_offset: usize);
}

/// Log a string to the console
pub fn log_str<S: AsRef<str>>(msg: S) {
	let data = msg.as_ref();

	unsafe {
		console_log(true, write_to_ipc(data));
	}
}

/// Log a string to the console
pub fn log_buf<S: AsRef<[u8]>>(data: S) {
	unsafe {
		console_log(false, write_to_ipc(data));
	}
}

/// Log a Rust error to JS console and exit
pub fn log_err<E: std::error::Error>(error_code: u8, err: E) -> ! {
	let data = err.to_string();

	unsafe {
		console_log(true, write_to_ipc(data.as_bytes()));
		error::exit(error_code, data.len())
	}
}
