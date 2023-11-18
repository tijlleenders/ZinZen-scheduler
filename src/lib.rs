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

use models::input::Input;
use models::output::FinalTasks;
use std::ops::Sub;
use wasm_bindgen::prelude::*;

mod errors;
/// Mocking module to generate objects for testing
pub mod mocking;
/// The data structures
pub mod models;
pub mod new_models;
/// The services handling the data structures
pub mod services;

#[cfg(test)]
mod tests;

use crate::models::date::{create_date_by_hour, dec_span_by, inc_span, inc_span_by, is_date_between, normalize_date};
use crate::models::goal::{Goal, TimeFilter};
use crate::models::output::{DayTasks, Task};
use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, NaiveTime};
#[cfg(feature = "with-logging")]
use std::sync::Once;

// Static flag to ensure logger init happens only once
#[cfg(feature = "with-logging")]
static LOGGER_INITIALIZED: Once = Once::new();

#[cfg(feature = "with-logging")]
fn initialize_logger() {
    // Use the Once flag to ensure initialization happens only once
    LOGGER_INITIALIZED.call_once(|| {
        env_logger::init();
    });
}

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
    console_error_panic_hook::set_once();
    let input: Input = serde_wasm_bindgen::from_value(input.clone())?;
    let final_tasks = run_scheduler(input);
    Ok(serde_wasm_bindgen::to_value(&final_tasks)?)
}

/// The main binary function to call
#[cfg(not(feature = "new-scheduler"))]
pub fn run_scheduler(input: Input) -> FinalTasks {
    #[cfg(feature = "with-logging")]
    initialize_logger();

    let steps = generate_steps_to_place(input);

    log::debug!("{:#?}", &steps);

    let placed_steps = step_placer(steps);

    match output_formatter(placed_steps) {
        Err(Error::NoConfirmedDate(title, id)) => {
            panic!("Error with step {title}:{id}. Steps passed to output formatter should always have a confirmed_start/deadline.");
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
        Ok(final_tasks) => {
            log::debug!("{:#?}", &final_tasks);
            final_tasks
        }
    }
}

#[cfg(feature = "new-scheduler")]
pub fn run_scheduler(input: Input) -> FinalTasks {
    #[cfg(feature = "with-logging")]
    initialize_logger();

    // let mut slots = generate_slots(&input.calendar_start, &input.calendar_end);
    //
    // let mut goals = get_goals(&input);
    // while !goals.is_empty() {
    //     let flexibilities = get_flexibilities(&goals, &input.calendar_start, &input.calendar_end);
    //     let flexibilities = remove_flexibilities(flexibilities, &slots);
    //     let flexibility_of_1 = flexibilities.iter().filter(|f| f.flexibility == 1).collect::<Vec<_>>();
    //     if !flexibility_of_1.is_empty() {
    //         let active_flexibility = &flexibility_of_1[0];
    //         apply_flexibility_to_slots(&mut slots, active_flexibility);
    //         goals = remove_goal(&goals, active_flexibility.goal);
    //         continue;
    //     }
    //
    //     let active_flexibility = find_greatest_flexibility(&flexibilities);
    //     println!("{active_flexibility:?}");
    //     let ref_flexibilities = flexibilities.iter().filter(|f| f.flexibility != active_flexibility.flexibility).collect::<Vec<_>>();
    //
    //     let working_slots = find_least_conflicting_slots(active_flexibility, ref_flexibilities);
    //     apply_slots(&mut slots, &working_slots, active_flexibility.goal);
    //
    //     goals = remove_goal(&goals, active_flexibility.goal);
    // }
    //
    // let mut tasks = vec![];
    // gather_tasks(&mut tasks, &slots);
    //
    let out = FinalTasks {
        // scheduled: vec![DayTasks {
        //     day: tasks[0].start.date(),
        //     tasks: tasks.clone(),
        // }],
        // impossible: vec![DayTasks {
        //     day: tasks[0].start.date(),
        //     tasks: vec![],
        // }],
        scheduled: vec![DayTasks {
            day: NaiveDate::default(),
            tasks: vec![],
        }],
        impossible: vec![DayTasks {
            day: NaiveDate::default(),
            tasks: vec![],
        }],
    };
    // new scheduler - end

    out
}
