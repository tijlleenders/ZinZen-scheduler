/// crate docz
use errors::Error;
use models::input::Input;
use models::output::FinalOutput;
use serde_wasm_bindgen::{from_value, to_value};
use services::output::output_formatter;
use services::{task_generator::task_generator, task_placer::task_placer};
use wasm_bindgen::prelude::*;

mod errors;
/// The data structures
pub mod models;
/// The services handling the data structures
pub mod services;
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
/// The main wasm function to call
#[wasm_bindgen]
pub fn schedule(input: &JsValue) -> Result<JsValue, JsError> {
    // JsError implements From<Error>, so we can just use `?` on any Error
    let input: Input = from_value(input.clone()).unwrap();
    let tasks = task_generator(input);
    let placed_tasks = task_placer(tasks);
    let output = match output_formatter(placed_tasks) {
        Err(Error::NoConfirmedDate(title, id)) => {
            panic!("Error with task {title}:{id}. Tasks passed to output formatter should always have a confirmed_start/deadline.")
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
        Ok(output) => output,
    };

    Ok(to_value(&output)?)
}

//Todo why is there a schedule function and a run_scheduler function?
/// The main binary function to call
pub fn run_scheduler(input: Input) -> FinalOutput {
    let tasks = task_generator(input);
    let placed_tasks = task_placer(tasks);
    match output_formatter(placed_tasks) {
        Err(Error::NoConfirmedDate(title, id)) => {
            panic!("Error with task {title}:{id}. Tasks passed to output formatter should always have a confirmed_start/deadline.");
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
        Ok(output) => output,
    }
}
