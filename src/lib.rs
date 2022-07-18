extern crate console_error_panic_hook;

use serde::Deserialize;
use time::OffsetDateTime;
use wasm_bindgen::prelude::*;

use crate::task_generator::task_generator;

/// API modules
mod console;
mod error;

/// Project details
mod goal;
mod task;
mod task_generator;
mod task_placer;

#[no_mangle]
unsafe extern "C" fn processTaskCount(_bytes: usize) -> usize {
	// let (goals, timeline) = load_goals_from_ipc(bytes);
	// let processed = PreProcessor::preprocess_old(&goals, timeline);
	//
	// let with_ids = processed.map(|(a, b)| (a, b.id)).collect::<Vec<_>>();
	// let string = serde_json::to_string(&with_ids).explode();
	//
	// write_to_ipc(string).explode()
	0 // XXX: stub
}

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"

export type Coords = { "latitude": number, "longitude": number, }; 

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

// https://rustwasm.github.io/wasm-bindgen/reference/arbitrary-data-with-serde.html
#[wasm_bindgen]
pub fn schedule(input: JsValue) -> JsValue {
	// Set console error hook, so we get console errors if this panic. This is only ran once
	console_error_panic_hook::set_once();

	// TODO serde error handling
	let input = input.into_serde().unwrap();

	let placer = task_generator(input);
	let result = placer.task_placer();

	// Any errors from unwrap() here is our fault
	JsValue::from_serde(&result).unwrap()
}
