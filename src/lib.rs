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

use crate::models::date::{inc_span, is_date_between};
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

    // new scheduler
    let mut flexibilities = input
        .goals
        .iter()
        .map(|(_, goal)| {
            let mut slots = generate_slots(&input);
            apply_goal_to_slots(&mut slots, goal);
            let flexibility = slots.iter().fold(0_usize, |num, slot| match slot {
                Slot::Goal(_, _) => num + 1,
                _ => num,
            });
            Flexibility { flexibility, slots }
        })
        .collect::<Vec<_>>();

    flexibilities.sort_by(|a, b| a.flexibility.cmp(&b.flexibility));
    // flexibilities.iter().for_each(|f| println!("{f:?}"));

    let mut slots = generate_slots(&input);
    for flexibility in flexibilities.iter() {
        apply_flexibility_to_slots(&mut slots, flexibility);
    }

    let mut tasks = vec![];
    gather_tasks(&mut tasks, &slots);

    let out = FinalTasks {
        scheduled: vec![DayTasks {
            day: tasks[0].start.date(),
            tasks: tasks.clone(),
        }],
        impossible: vec![DayTasks {
            day: tasks[0].start.date(),
            tasks: vec![],
        }],
    };
    // new scheduler - end

    out
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

use crate::new_models::flexibility::Flexibility;
use crate::new_models::slot::Slot;
fn generate_slots(input: &Input) -> Vec<Slot> {
    let mut out = vec![Slot::Empty(input.calendar_start.clone())];

    while out
        .last()
        .unwrap()
        .date()
        .lt(&input.calendar_end.sub(Duration::hours(1)))
    {
        out.push(Slot::Empty(inc_span(out.last().unwrap().date())))
    }

    out
}

fn apply_goal_to_slots<'a, 'b: 'a>(slots: &'a mut Vec<Slot<'b>>, goal: &'b Goal) {
    for slot in &mut *slots {
        if !is_in_filter(slot, &goal.filters) {
            continue;
        }
        // continue if it doesn't fit into the slot
        match (goal.start, goal.deadline) {
            (Some(ref start), Some(ref end)) => {
                if !is_date_between(slot.date(), start, end) {
                    continue;
                }
            }
            (Some(ref start), _) => {
                if slot.date().lt(start) {
                    continue;
                }
            }
            (_, Some(ref end)) => {
                if slot.date().ge(end) {
                    continue;
                }
            }
            _ => {}
        }
        *slot = Slot::Goal(slot.date().clone(), goal);
    }
}

fn apply_flexibility_to_slots<'a, 'b: 'a>(
    slots: &'a mut Vec<Slot<'b>>,
    flexibility: &'b Flexibility<'b>,
) {
    let filtered_slots = flexibility
        .slots
        .iter()
        .filter(|&slot| {
            if let Slot::Goal(_, _) = slot {
                true
            } else {
                false
            }
        })
        .collect::<Vec<_>>();

    let mut span = 0_usize;
    'a_loop: for slot in slots {
        if let Slot::Goal(_, _) = slot {
            continue;
        }
        for &f_slot in &filtered_slots {
            if f_slot.date().eq(slot.date()) {
                let goal = f_slot.goal().unwrap();
                span += 1;
                if span <= goal.min_duration.unwrap_or(1) {
                    *slot = f_slot.clone();
                    // println!("{slot:?}");
                } else {
                    break 'a_loop;
                }
            }
        }
    }
}

fn is_in_filter(slot: &mut Slot, filter: &Option<TimeFilter>) -> bool {
    // println!("{filter:?}");
    if let Some(filter) = filter {
        is_date_between(
            slot.date(),
            &get_date_by_hour(slot.date(), filter.after_time.unwrap()),
            &get_date_by_hour(slot.date(), filter.before_time.unwrap()),
        )
    } else {
        false
    }
}
fn get_date_by_hour(date: &NaiveDateTime, hour: usize) -> NaiveDateTime {
    NaiveDateTime::new(
        NaiveDate::from_ymd_opt(date.year(), date.month(), date.day()).unwrap(),
        NaiveTime::from_hms_opt(hour as u32, 0, 0).unwrap(),
    )
}

fn gather_tasks<'a, 'b: 'a>(tasks: &'a mut Vec<Task>, slots: &'b Vec<Slot<'b>>) {
    if slots.is_empty() {
        return;
    }

    let mut taskid = 0;
    let mut task = {
        let slot = &slots[0];
        if let Some(goal) = slot.goal() {
            create_task(taskid, goal.id.clone(), goal.title.clone(), slot.date())
        } else {
            create_task(taskid, "free".to_owned(), "free".to_owned(), slot.date())
        }
    };

    for idx in 1..slots.len() {
        let slot = &slots[idx];
        if let Some(goal) = slot.goal() {
            if task.goalid.eq("free") {
                tasks.push(task.clone());
                taskid += 1;
                task = create_task(taskid, goal.id.clone(), goal.title.clone(), slot.date());
            } else if task.goalid.ne(&goal.id) {
                tasks.push(task.clone());
                taskid += 1;
                task = create_task(taskid, goal.id.clone(), goal.title.clone(), slot.date());
            } else {
                task.duration += 1;
                task.deadline = inc_span(&task.deadline);
            }
        } else {
            if task.goalid.eq("free") {
                task.duration += 1;
                task.deadline = inc_span(&task.deadline);
            } else {
                tasks.push(task.clone());
                taskid += 1;
                task = create_task(taskid, "free".to_owned(), "free".to_owned(), slot.date());
            }
        }
    }

    tasks.push(task);
}

fn create_task(id: usize, goalid: String, title: String, start: &NaiveDateTime) -> Task {
    Task {
        taskid: id,
        goalid,
        title,
        duration: 1,
        start: start.clone(),
        deadline: inc_span(start),
        tags: vec![],
        impossible: false,
    }
}
