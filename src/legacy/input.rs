use super::budget::StepBudgets;
use super::date::deserialize_normalized_date;
use super::goal::GoalsMap;
use super::step::Step;
use chrono::prelude::*;
use serde::Deserialize;

/// The front end gets a Calendar by passing a JSON data into the scheduler, via an Input object.
/// It has the requested calendar start and end, and a collection of goals that the scheduler needs to schedule.
/// On the scheduler side, the JSON is received as a ['JSValue'](https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/struct.JsValue.html).
/// This allows it to be deserialized into this struct.
#[derive(Deserialize, Debug)]
pub struct Input {
    #[serde(rename = "startDate")]
    #[serde(deserialize_with = "deserialize_normalized_date")]
    pub calendar_start: NaiveDateTime,
    #[serde(rename = "endDate")]
    #[serde(deserialize_with = "deserialize_normalized_date")]
    pub calendar_end: NaiveDateTime,
    pub goals: GoalsMap,
}

#[derive(Debug, Deserialize)]
pub struct StepsToPlace {
    #[serde(deserialize_with = "deserialize_normalized_date")]
    pub calendar_start: NaiveDateTime,
    #[serde(deserialize_with = "deserialize_normalized_date")]
    pub calendar_end: NaiveDateTime,
    pub steps: Vec<Step>,
    pub step_budgets: StepBudgets,
}

#[derive(Deserialize, Debug)]
pub struct PlacedSteps {
    #[serde(deserialize_with = "deserialize_normalized_date")]
    pub calendar_start: NaiveDateTime,
    #[serde(deserialize_with = "deserialize_normalized_date")]
    pub calendar_end: NaiveDateTime,
    pub steps: Vec<Step>,
}
