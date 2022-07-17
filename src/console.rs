//! Console function implementations for `.wasm`
#![allow(dead_code)]

extern "C" {
	/// Imported logging facade from JavaScript
	fn console_log(is_string: bool, ipc_offset: usize);
}
