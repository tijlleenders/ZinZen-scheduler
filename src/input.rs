use chrono::prelude::*;
use serde::Deserialize;

use crate::goal::Goal;

#[derive(Deserialize, Debug)]
/// Just a deserialization target
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
