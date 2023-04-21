use errors::Error;
use models::input::Input;
use models::output::FinalOutput;
use models::task::Task;
use serde_wasm_bindgen::{from_value, to_value};
use services::output::output_formatter;
use services::task_generator::generate_tasks_to_place;
use services::task_placer::task_placer;
use wasm_bindgen::prelude::*;

mod errors;
/// API modules
pub mod models;
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
#[wasm_bindgen]
pub fn schedule(input: &JsValue) -> Result<JsValue, JsError> {
    // JsError implements From<Error>, so we can just use `?` on any Error
    let input: Input = from_value(input.clone()).unwrap();
    let tasks = generate_tasks_to_place(input);
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
pub fn run_scheduler(input: Input) -> FinalOutput {
    let tasks = generate_tasks_to_place(input);
    print_tasks(&tasks.tasks);
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

fn print_tasks(tasks: &Vec<Task>) {
    let mut timings: Vec<String> = vec![];
    for task in tasks.iter() {
        dbg!(task.id.clone());
        for slot in task.slots.iter() {
            let slot_timing: String = format!(" [ Start: {} , End: {} ] ", slot.start, slot.end);
            timings.push(slot_timing);
        }

        dbg!(&timings);
    }
    dbg!(&timings);
}
