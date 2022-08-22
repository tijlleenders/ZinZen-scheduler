use wasm_bindgen::prelude::*;

pub use goal::{Goal, Repetition};
pub use input::Input;

/// API modules
mod goal;
mod input;
mod task;
mod task_generator;
mod task_placer;
mod output_formatter;
mod util;

// Tests
#[cfg(test)]
mod tests;

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
	use task_generator::task_generator;
    use task_placer::*;
    use output_formatter::*;
	// JsError implements From<Error>, so we can just use `?` on any Error
	let input:Input = input.into_serde()?;

/*
	// Generates a task and slots list from the provided parameters
	let placer = task_generator(input);
	let result = match placer.task_placer() {
		Ok(res) => res,
		// How tedious, these error types are somehow incomaptible
		Err(err) => return Err(JsError::new(&err.to_string())),
	};

	Ok(JsValue::from_serde(&result)?)
*/
    let calendar_start = input.calendar_start;
    let calendar_end = input.calendar_end;
    let mut tasks = task_generator(input);
    task_placer(&mut tasks,calendar_start,calendar_end);
    let output = output_formatter(tasks).unwrap();
    Ok(JsValue::from_serde(&output)?)

}
