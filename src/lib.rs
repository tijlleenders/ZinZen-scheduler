use serde::Deserialize;
use time::OffsetDateTime;
use wasm_bindgen::prelude::*;

use crate::task_generator::task_generator;

/// Project details
mod goal;
mod task;
mod task_generator;
mod task_placer;

#[wasm_bindgen]
extern "C" {
	// Import console.log from JavaScript
	#[wasm_bindgen(js_namespace = console)]
	fn log(s: &str);
}

/// A little logging function to make it easier to debug, calls console.log
pub fn console_log<S: AsRef<str>>(input: S) {
	log(input.as_ref());
}

/// Not necessarily an entry point, just initializes the console error hook
#[wasm_bindgen(start)]
pub fn init() -> Result<(), JsError> {
	std::panic::set_hook(Box::new(console_error_panic_hook::hook));
	log("Successfully initialized scheduler, ready to go!");

	Ok(())
}
#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
interface Input {
    startDate: string;
    endDate: string;
    goals: number
}
"#;

#[derive(Deserialize, Debug)]
/// Just a deserialization target
pub struct Input {
	#[serde(rename = "startDate")]
	#[serde(with = "time::serde::iso8601")]
	start: OffsetDateTime,
	#[serde(rename = "endDate")]
	#[serde(with = "time::serde::iso8601")]
	_end: OffsetDateTime,
	goals: Vec<goal::Goal>,
}

/// Generates a task and slots list from the provided parameters
#[wasm_bindgen]
pub fn schedule(input: JsValue) -> Result<JsValue, JsError> {
	// JsError implements From<Error>, so we can just use `?` on any Error
	let input = input.into_serde()?;

	let placer = task_generator(input);
	let result = match placer.task_placer() {
		Ok(res) => res,
		// How tedious, these error types are somehow incomaptible
		Err(err) => return Err(JsError::new(&err.to_string())),
	};

	Ok(JsValue::from_serde(&result)?)
}
