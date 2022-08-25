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

    let calendar_start = input.calendar_start;
    let calendar_end = input.calendar_end;
    let tasks = task_generator(input);
    let scheduled_tasks = task_placer(tasks,calendar_start,calendar_end);
    let output = output_formatter(scheduled_tasks).unwrap();
    Ok(JsValue::from_serde(&output)?)

}
