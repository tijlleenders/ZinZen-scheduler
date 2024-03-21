//! # ZinZen scheduler
//!
//! The ZinZen scheduler is a "calendar as a function".  
//! Input: A calendar start datetime and end datetime, plus a Directed Acyclical Graph of Goals/Budgets with time constraints.  
//! Output: A calendar that successfully allocates all Goals - or the maximum amount of Goals in that time period.  
//!
// TODO: fix DocTest
// ```
// use scheduler::scheduler;
//
//     let json_input: serde_json::Value = serde_json::json!({
//       "TODO_working_example"
//     });
//     let input: Input = serde_json::from_value(json_input)?;
//     let output = scheduler::run_scheduler(input);
// ```
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

use chrono::NaiveDateTime;
use models::{activity::Activity, calendar::Calendar, goal::Goal, task::FinalTasks};
use serde_wasm_bindgen::{from_value, to_value};
use services::activity_generator;
use services::activity_placer;
use technical::input_output::Input;
use wasm_bindgen::prelude::*;
pub mod models;
pub mod services;
/// The data structures
pub mod technical;

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
    // JsError implements From<Error>, so we can just use `?` on any Error
    let input: Input = from_value(input.clone())?;
    let final_tasks = run_scheduler(input.start_date, input.end_date, &input.goals);
    Ok(to_value(&final_tasks)?)
}

pub fn run_scheduler(
    start_date: NaiveDateTime,
    end_date: NaiveDateTime,
    goals: &[Goal],
) -> FinalTasks {
    let mut calendar = Calendar::new(start_date, end_date);
    dbg!(&calendar);

    calendar.add_budgets_from(goals);

    //generate and place simple goal activities
    let simple_goal_activities =
        activity_generator::generate_simple_goal_activities(&calendar, goals);
    dbg!(&simple_goal_activities);

    //generate and place budget goal activities
    let budget_goal_activities: Vec<Activity> =
        activity_generator::generate_budget_goal_activities(&calendar, goals);
    dbg!(&budget_goal_activities);
    dbg!(&calendar);

    let activities: Vec<Activity> = [simple_goal_activities, budget_goal_activities].concat();
    activity_placer::place(&mut calendar, activities);

    calendar.log_impossible_min_day_budgets();

    if let Some(get_to_week_min_budget_activities) =
        activity_generator::generate_get_to_week_min_budget_activities(&calendar, goals)
    {
        activity_placer::place(&mut calendar, get_to_week_min_budget_activities);
    }
    //TODO: Test that day stays below min when week min being reached so other goals can get to the week min too

    calendar.log_impossible_min_week_budgets();

    let top_up_week_budget_activities =
        activity_generator::generate_top_up_week_budget_activities(&calendar, goals);
    activity_placer::place(&mut calendar, top_up_week_budget_activities);
    //TODO: Test that day stays below min or max when week max being reachd

    calendar.print()
}
