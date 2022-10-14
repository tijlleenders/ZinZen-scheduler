use wasm_bindgen::prelude::*;

pub use goal::Goal;
pub use input::Input;
pub use output_formatter::{output_formatter, Output};
pub use time_slot_iterator::Repetition;

mod errors;
/// API modules
mod goal;
pub mod input;
pub mod output_formatter;
mod task;
mod task_generator;
mod task_placer;
mod time_slot_iterator;
mod util;

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
pub fn schedule(input: &JsValue) -> Result<JsValue, JsError> {
    use errors::Error;
    use output_formatter::*;
    use task_generator::task_generator;
    use task_placer::*;
    // JsError implements From<Error>, so we can just use `?` on any Error
    let input: Input = input.into_serde()?;

    let calendar_start = input.calendar_start;
    let calendar_end = input.calendar_end;
    let tasks = task_generator(input);
    let scheduled_tasks = task_placer(tasks, calendar_start, calendar_end);
    let output = match output_formatter(scheduled_tasks) {
        Err(Error::NoConfirmedDate(title, id)) => {
            panic!("Error with task {title}:{id}. Tasks passed to output formatter should always have a confirmed_start/deadline.")
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
        Ok(output) => output,
    };

    Ok(JsValue::from_serde(&output)?)
}

pub fn run_scheduler(input: Input) -> Vec<Output> {
    use errors::Error;
    use output_formatter::*;
    use task_generator::task_generator;
    use task_placer::*;
    let calendar_start = input.calendar_start;
    let calendar_end = input.calendar_end;
    let tasks = task_generator(input);
    let scheduled_tasks = task_placer(tasks, calendar_start, calendar_end);
    match output_formatter(scheduled_tasks) {
        Err(Error::NoConfirmedDate(title, id)) => {
            panic!("Error with task {title}:{id}. Tasks passed to output formatter should always have a confirmed_start/deadline.");
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
        Ok(output) => output,
    }
}
