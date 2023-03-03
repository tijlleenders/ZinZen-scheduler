use crate::goal::Goal;
use crate::task::Task;
use chrono::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
/// The front end passes data into the scheduler via an Input object.
/// It defines the calendar start and end, and a collection of goals that the scheduler needs to schedule.
/// On the scheduler side, the Input is passed as a ['JSValue'](https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/struct.JsValue.html).
/// This allows it to be deserialized into this struct.
pub struct Input {
    #[serde(rename = "startDate")]
    pub calendar_start: NaiveDateTime,
    #[serde(rename = "endDate")]
    pub calendar_end: NaiveDateTime,
    pub goals: Vec<Goal>,
}

#[cfg(test)]
impl Input {
    /// Create a new Input. Only useful for tests, otherwise input is
    /// deserialized as the input function.
    pub fn new(start: NaiveDateTime, end: NaiveDateTime, goals: Vec<Goal>) -> Self {
        Self {
            calendar_start: start,
            calendar_end: end,
            goals,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct TasksToPlace {
    pub calendar_start: NaiveDateTime,
    pub calendar_end: NaiveDateTime,
    pub tasks: Vec<Task>,
}

impl TasksToPlace {
    pub fn sort_on_flexibility(&mut self) {
        self.calculate_flexibilities();
        self.tasks.sort();
    }

    fn calculate_flexibilities(&mut self) {
        for task in self.tasks.iter_mut() {
            task.calculate_flexibility();
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct PlacedTasks {
    pub calendar_start: NaiveDateTime,
    pub calendar_end: NaiveDateTime,
    pub tasks: Vec<Task>,
}
