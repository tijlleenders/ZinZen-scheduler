//! Console function implementations for `.wasm`

#![allow(dead_code)]
use crate::{error::Explode, write_to_ipc};

extern "C" {
	/// Imported logging facade from JavaScript
	fn console_log(is_string: bool, ipc_offset: usize);
}

/// Log a string to the console
pub fn log_str<S: AsRef<str>>(msg: S) {
	unsafe { console_log(true, write_to_ipc(msg.as_ref()).explode()) }
}

/// Log a string to the console
pub fn log_buf<S: AsRef<[u8]>>(data: S) {
	unsafe { console_log(false, write_to_ipc(data).explode()) }
}
