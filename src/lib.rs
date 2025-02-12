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

use crate::models::activity::Activity;
use activity_generator::{
    add_budget_min_day_activities, add_budget_min_week_activities,
    add_budget_top_up_week_activities, add_simple_activities, add_tasks_completed_today,
};
use activity_placer::{place, place_postponed_as_best_effort};
use chrono::NaiveDateTime;
use models::task::TaskCompletedToday;
use models::{calendar::Calendar, goal::Goal, task::FinalTasks};
use serde_wasm_bindgen::{from_value, to_value};
use services::activity_generator;
use services::activity_placer;
use std::collections::BTreeMap;
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
    let final_tasks = run_scheduler(
        input.start_date,
        input.end_date,
        &input.goals,
        input.tasks_completed_today,
    );
    Ok(to_value(&final_tasks)?)
}

#[must_use]
pub fn run_scheduler(
    start_date: NaiveDateTime,
    end_date: NaiveDateTime,
    goals: &[Goal],
    tasks_completed_today: Vec<TaskCompletedToday>,
) -> FinalTasks {
    let mut calendar = Calendar::new(start_date, end_date);
    let mut activities: Vec<Activity> = vec![];
    let mut goal_map: BTreeMap<String, Goal> = BTreeMap::new(); //Don't use hashmap as that doesn't guarantee ordering - messing up determinacy of tests
    for goal in goals {
        //optimize this out if frontend already has a map? - probably won't have any significant effect => measure
        goal_map.insert(goal.id.clone(), goal.clone());
    }

    calendar.add_budgets_from(&mut goal_map);

    dbg!(&calendar); //before tasks_completed_today
    add_tasks_completed_today(
        &calendar,
        &goal_map,
        &tasks_completed_today,
        &mut activities,
    );
    place(&mut calendar, &mut activities);

    dbg!(&calendar); //before simple
    add_simple_activities(&mut calendar, &goal_map, &mut activities);
    add_budget_min_day_activities(&mut calendar, &goal_map, &mut activities);
    //Todo: do we need a different treatment of Activities if they are part of budget or not ?
    //If not, simplify the code! Also for generating activities?

    place(&mut calendar, &mut activities);

    dbg!(&calendar); //before get_budget_min
    add_budget_min_week_activities(&calendar, &goal_map, &mut activities);
    place(&mut calendar, &mut activities);

    dbg!(&calendar); //before get_budget_top_up_week
    add_budget_top_up_week_activities(&calendar, &goal_map, &mut activities);
    place(&mut calendar, &mut activities);

    dbg!(&calendar); //before BestEffort
    place_postponed_as_best_effort(&mut calendar, &mut activities);

    //TODO: Fit simple budget activities into scheduled budgets?
    //      No need, as simple budget activities will share the same overlay, but with less hours
    //      Thus, the flex will always be higher than (or equal to?) the MinDayBudget activities
    //      So MinDayBudget will get chosen last unless flex is equal and order happens to favor MinDayBudget
    //          => TODO: order activities before placing?
    dbg!(&calendar); //final result

    calendar.log_impossible_activities(&activities);
    calendar.print_new(&activities)
}
