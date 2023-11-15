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

// fn get_goals(input: &Input) -> Vec<&Goal> {
//     input.goals.values().collect()
// }
//
// fn get_flexibilities<'a, 'b: 'a>(goals: &'a Vec<&'b Goal>, start: &'a NaiveDateTime, end: &'a NaiveDateTime) -> Vec<Flexibility<'b>> {
//     goals
//         .iter()
//         .map(|&goal| {
//             let slots = generate_slots(&start, &end).into_iter()
//                 .map(|s| if is_date_between(s.date(), &create_date_by_hour(start, goal.filters.as_ref().map(|f| f.after_time).unwrap_or(None).unwrap_or(0)), &create_date_by_hour(start, goal.filters.as_ref().map(|f| f.before_time).unwrap_or(None).unwrap_or(24))) {
//                     Slot::Goal(s.date().clone(), goal)
//                 } else {
//                     s
//                 })
//                 .collect::<Vec<_>>()
//                 ;
//             let flexibility = slots.iter().fold(0_usize, |num, slot| match slot {
//                 Slot::Goal(_, _) => num + 1,
//                 _ => num,
//             });
//             Flexibility { flexibility, goal, slots }
//         })
//         .collect::<Vec<_>>()
// }
//
// fn apply_slots<'a, 'b: 'a>(slots: &'a mut Vec<Slot<'b>>, working_slots: &'a Vec<Slot<'b>>, goal: &'b Goal) {
//     let dates = working_slots.iter().map(|s| s.date()).collect::<Vec<_>>();
//     for idx in 0..slots.len() {
//         let d = slots[idx].date();
//         if dates.contains(&d) {
//             slots[idx] = Slot::Goal(d.clone(), goal);
//         }
//     }
// }
//
// fn remove_flexibilities<'a, 'b: 'a>(mut flexibilities: Vec<Flexibility<'b>>, slots: &'a Vec<Slot<'b>>) -> Vec<Flexibility<'b>> {
//     for f in &mut flexibilities {
//         for idx in 0..f.slots.len() {
//             if let Some(Slot::Goal(_, _)) = slots.get(idx) {
//                 f.slots[idx] = Slot::Empty(f.slots[idx].date().clone())
//             }
//         }
//     }
//     flexibilities
// }
//
// fn remove_goal<'a>(goals: &Vec<&'a Goal>, goal: &Goal) -> Vec<&'a Goal> {
//     goals.iter().filter(|&& g| g.id.ne(&goal.id)).map(|&g| g).collect::<Vec<_>>()
// }
//
// fn find_greatest_flexibility<'a, 'b>(flexibilities: &'a Vec<Flexibility<'b>>) -> &'a Flexibility<'b> {
//     let init = &flexibilities[0];
//     flexibilities.iter().fold(init, |acc, f| if f.flexibility > acc.flexibility { f } else { acc })
// }
//
// fn find_least_conflicting_slots<'a, 'b: 'a>(active_flexibility: &'a Flexibility<'b>, ref_flexibilities: Vec<&'a Flexibility<'b>>) -> Vec<Slot<'b>> {
//     ref_flexibilities.iter()
//         .map(|&f| {
//             let (count, slot, duration) = find_least_conflicting_slot(active_flexibility, f);
//             (count, generate_slots(slot.date(), &inc_span_by(slot.date(), duration as i64)))
//         })
//         .fold((0_usize, vec![]), |acc, item| if acc.0 > item.0 {
//             item
//         } else {
//             acc
//         }).1
// }
// fn find_least_conflicting_slot<'a>(active_flexibility: &Flexibility<'a>, ref_flexibility: &'a Flexibility<'a>) -> (usize, Slot<'a>, usize) {
//     let start = find_start_date(active_flexibility);
//     let end = find_end_date(active_flexibility);
//     let duration = active_flexibility.goal.min_duration.unwrap_or(1);
//
//     let out = ref_flexibility.slots.iter()
//         .filter(|&s| s.date().ge(&start) || s.date().lt(&dec_span_by(&end, duration as i64)))
//         .map(|s| (count_slots(&ref_flexibility.slots, s.date(), &inc_span_by(s.date(), duration as i64)), s))
//         .fold((usize::MAX, ref_flexibility.slots.first().unwrap().clone()), |(acc_count, acc_slot), (count, slot)| if acc_count > count {
//             (count, slot.clone())
//         } else {
//             (acc_count, acc_slot)
//         });
//     (out.0, out.1, duration)
// }
//
// fn find_start_date(flexibility: &Flexibility<'_>) -> NaiveDateTime {
//     flexibility
//         .goal
//         .filters.clone()
//         .map(|f| f.after_time)
//         .unwrap_or(None)
//         .map(|hour| create_date_by_hour(flexibility.slots.first().unwrap().date(), hour))
//         .unwrap_or(flexibility.slots.first().unwrap().date().clone())
//         .clone()
// }
// fn find_end_date(flexibility: &Flexibility<'_>) -> NaiveDateTime {
//     flexibility
//         .goal
//         .filters.clone()
//         .map(|f| f.before_time)
//         .unwrap_or(None)
//         .map(|hour| create_date_by_hour(flexibility.slots.first().unwrap().date(), hour))
//         .unwrap_or(flexibility.slots.last().unwrap().date().clone())
// }
// fn count_slots(slots: &Vec<Slot>, start: &NaiveDateTime, end: &NaiveDateTime) -> usize {
//     slots.iter()
//         .filter(|&s| s.date().ge(start) || s.date().lt(end))
//         .fold(0_usize, |count, s| if let Slot::Goal(_, _) = s {
//             count + 1
//         } else {
//             count
//         })
// }
//
// fn generate_slots<'a, 'b>(start: &'a NaiveDateTime, end: &'a NaiveDateTime) -> Vec<Slot<'b>> {
//     let mut out = vec![Slot::Empty(start.clone())];
//
//     while out
//         .last()
//         .unwrap()
//         .date()
//         .lt(&end.sub(Duration::hours(1)))
//     {
//         out.push(Slot::Empty(inc_span(out.last().unwrap().date())))
//     }
//
//     out
// }
//
// fn apply_flexibility_to_slots<'a, 'b: 'a>(
//     slots: &'a mut Vec<Slot<'b>>,
//     flexibility: &'a Flexibility<'b>,
// ) {
//     let filtered_slots = flexibility
//         .slots
//         .iter()
//         .filter(|&slot| {
//             if let Slot::Goal(_, _) = slot {
//                 true
//             } else {
//                 false
//             }
//         })
//         .collect::<Vec<_>>();
//
//     let mut span = 0_usize;
//     'a_loop: for slot in slots {
//         if let Slot::Goal(_, _) = slot {
//             continue;
//         }
//         for &f_slot in &filtered_slots {
//             if f_slot.date().eq(slot.date()) {
//                 let goal = f_slot.goal().unwrap();
//                 span += 1;
//                 if span <= goal.min_duration.unwrap_or(1) {
//                     *slot = f_slot.clone();
//                     // println!("{slot:?}");
//                 } else {
//                     break 'a_loop;
//                 }
//             }
//         }
//     }
// }
//
//
//
//
//
//
//
//
//
//
//
// fn gather_tasks<'a, 'b: 'a>(tasks: &'a mut Vec<Task>, slots: &'b Vec<Slot<'b>>) {
//     if slots.is_empty() {
//         return;
//     }
//
//     let mut taskid = 0;
//     let mut task = {
//         let slot = &slots[0];
//         if let Some(goal) = slot.goal() {
//             create_task(taskid, goal.id.clone(), goal.title.clone(), slot.date())
//         } else {
//             create_task(taskid, "free".to_owned(), "free".to_owned(), slot.date())
//         }
//     };
//
//     for idx in 1..slots.len() {
//         let slot = &slots[idx];
//         if let Some(goal) = slot.goal() {
//             if task.goalid.eq("free") {
//                 tasks.push(task.clone());
//                 taskid += 1;
//                 task = create_task(taskid, goal.id.clone(), goal.title.clone(), slot.date());
//             } else if task.goalid.ne(&goal.id) {
//                 tasks.push(task.clone());
//                 taskid += 1;
//                 task = create_task(taskid, goal.id.clone(), goal.title.clone(), slot.date());
//             } else {
//                 task.duration += 1;
//                 task.deadline = inc_span(&task.deadline);
//             }
//         } else {
//             if task.goalid.eq("free") {
//                 task.duration += 1;
//                 task.deadline = inc_span(&task.deadline);
//             } else {
//                 tasks.push(task.clone());
//                 taskid += 1;
//                 task = create_task(taskid, "free".to_owned(), "free".to_owned(), slot.date());
//             }
//         }
//     }
//
//     tasks.push(task);
// }
//
// fn create_task(id: usize, goalid: String, title: String, start: &NaiveDateTime) -> Task {
//     Task {
//         taskid: id,
//         goalid,
//         title,
//         duration: 1,
//         start: start.clone(),
//         deadline: inc_span(start),
//         tags: vec![],
//         impossible: false,
//     }
// }
