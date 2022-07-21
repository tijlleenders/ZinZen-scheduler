extern crate console_error_panic_hook;

use wasm_bindgen::prelude::*;

pub use goal::{Goal, Repetition};
pub use input::Input;

/// API modules
mod error;
mod goal;

/// Project details
mod input;
mod task;
mod task_generator;
mod task_placer;
mod util;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
interface Input {
    startDate: string;
    endDate: string;
    goals: number
}
"#;

// https://rustwasm.github.io/wasm-bindgen/reference/arbitrary-data-with-serde.html
#[wasm_bindgen]
pub fn schedule(input: JsValue) -> JsValue {
	use task_generator::task_generator;

	// Set console error hook, so we get console errors if this panic. This is only ran once
	console_error_panic_hook::set_once();

	// TODO serde error handling
	let input = input.into_serde().unwrap();

	let placer = task_generator(input);
	let result = placer.task_placer();

	// Any errors from unwrap() here is our fault
	JsValue::from_serde(&result).unwrap()
}
