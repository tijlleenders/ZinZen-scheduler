//! # ZinZen scheduler
//!
//! The ZinZen scheduler is a "calendar as a function".  
//! Input: A calendar start datetime and end datetime, plus some Goals with flexible time constraints.  
//! Output: A calendar that successfully allocates all Goals - or the maximum amount of Goals in that time period.  
//!
//! ```
//! use scheduler::models::input::Input;
//!
//!     let json_input: serde_json::Value = serde_json::json!({
//!       "startDate": "2022-01-01T00:00:00",
//!       "endDate": "2022-01-09T00:00:00",
//!       "goals": {
//!         "uuid1": {
//!           "id": "uuid1",
//!           "title": "sleep",
//!           "min_duration": 8,
//!           "repeat": "daily",
//!           "filters": {
//!             "after_time": 22,
//!             "before_time": 8
//!           }
//!         }
//!       }
//!     });
//!     let input: Input = serde_json::from_value(json_input).unwrap();
//!     let output = scheduler::run_scheduler(input);
//!     dbg!(output);
//! ```
//!
//! ## Getting Started
//! This project is hosted on [Github](https://github.com/tijlleenders/ZinZen-scheduler). The Docs.rs / Crates.io version is probably (far) behind.  
//! Please submit an issue there if you've found something we need to improve or have a question regarding how things work.
//!
//! For more explanation, see the crate documentation.
//! There are no features to configure.
//!
//!
//!
//! ## Special Considerations
//!
//! We're not at 1.0 major version yet.  
//! Expect breaking changes for every minor (y in 0.x.y) release!
//!
//! ## Contributing
//!
//! Read the standard [Contributor Covenant Code of Conduct](https://github.com/tijlleenders/ZinZen-scheduler/blob/main/CONTRIBUTING.md).  
//! **TL;DR : Be nice.**  
//! We also use the principles of Robert C. Martin's 'Clean Code' - nicely summarized [on this Gist](https://gist.github.com/wojteklu/73c6914cc446146b8b533c0988cf8d29).  
//! If you find documentation missing, this is considered a bug, so please submit a bug report!
//!
//! ## License and legal stuff
//!
//! AGPL 3.0 See [LICENSE](LICENSE) file.
//!
//! &copy;2020-now ZinZen&reg;
//!
//! This code is licensed under AGPLv3 but this license does not implicitly grant
//! permission to use the trade names, trademarks, service marks, or product names
//! of the licensor, except as required for reasonable and customary use in
//! describing the origin of the Work and the content of the notice/attribution
//! file.
//!
//! ZinZen&reg; supports an open and collaborative process. Registering the
//! ZinZen&reg; trademark is a tool to protect the ZinZen&reg; identity and the
//! quality perception of the ZinZen&reg; projects.

use errors::Error;
use models::input::Input;
use models::output::FinalOutput;
use serde_wasm_bindgen::{from_value, to_value};
use services::output::output_formatter;
use services::placer::step_placer;
use services::preprocess::generate_steps_to_place;
use wasm_bindgen::prelude::*;

mod errors;
/// Mocking module to generate objects for testing
pub mod mocking;
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
    let steps = generate_steps_to_place(input);
    let placed_steps = step_placer(steps);
    let output = match output_formatter(placed_steps) {
        Err(Error::NoConfirmedDate(title, id)) => {
            panic!("Error with step {title}:{id}. Steps passed to output formatter should always have a confirmed_start/deadline.")
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
    let steps = generate_steps_to_place(input);

    let placed_steps = step_placer(steps);
    dbg!(&placed_steps);
    match output_formatter(placed_steps) {
        Err(Error::NoConfirmedDate(title, id)) => {
            panic!("Error with step {title}:{id}. Steps passed to output formatter should always have a confirmed_start/deadline.");
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
        Ok(output) => output,
    }
}

/* TODO DEBUG NOTES

IDEA: This is to uniform location for notes related to similar bugs/concepts/issues/ideas in many files or modules.

# 2023-06-06
- Found many functions achieve the same concept "remove slots" as below samples:
    - Step::remove_slot(&mut self, s: Slot)
    - Step::remove_taken_slots(&mut self, s: Slot)
    - Timeline::remove_slots(&mut self, slots_to_remove: Vec<Slot>)

# 2023-06-07
- For filter_timing, "sleep" step in bug_215, on the last slot it consider few hours
more out of deadline than it should.

*/
