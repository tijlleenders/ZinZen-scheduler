use wasm_bindgen::prelude::*;

pub use goal::Goal;
pub use input::Input;
pub use output_formatter::{Output, output_formatter};
pub use time_slice_iterator::Repetition;

/// API modules
mod goal;
pub mod input;
pub mod output_formatter;
mod task;
mod task_generator;
mod task_placer;
mod util;
mod time_slice_iterator;

// Test
#[cfg(test)]
mod unit_tests;

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
pub fn schedule(input: JsValue) -> Result<JsValue, JsError> {
	use output_formatter::*;
	use task_generator::task_generator;
	use task_placer::*;
	// JsError implements From<Error>, so we can just use `?` on any Error
	let input: Input = input.into_serde()?;

	let calendar_start = input.calendar_start;
	let calendar_end = input.calendar_end;
	let tasks = task_generator(input);
	let scheduled_tasks = task_placer(tasks, calendar_start, calendar_end);
	let output = output_formatter(scheduled_tasks).unwrap();
	Ok(JsValue::from_serde(&output)?)
}

pub fn run_scheduler(input: Input) -> Vec<Output> {
    use output_formatter::*;
	use task_generator::task_generator;
	use task_placer::*;
    let calendar_start = input.calendar_start;
	let calendar_end = input.calendar_end;
	let tasks = task_generator(input);
	let scheduled_tasks = task_placer(tasks, calendar_start, calendar_end);
	let output = output_formatter(scheduled_tasks).unwrap();
    output
}
