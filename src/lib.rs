#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use web_sys::console;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn starts_as_soon_as_module_is_loaded() -> Result<(), JsValue> {
	// print pretty errors in wasm https://github.com/rustwasm/console_error_panic_hook
	// This is not needed for tracing_wasm to work, but it is a common tool for getting proper error line numbers for panics.
	console_error_panic_hook::set_once();

	tracing_wasm::set_as_global_default();

	//will have to debug using web-sys log statements... :/ https://rustwasm.github.io/book/reference/debugging.html#using-a-debugger

	Ok(())
}

pub mod calendar;
pub mod slot;
pub mod tasks;
pub mod tests;
