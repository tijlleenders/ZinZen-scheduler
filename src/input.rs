use chrono::prelude::*;
use serde::Deserialize;

use crate::goal::Goal;

#[derive(Deserialize, Debug)]
/// Just a deserialization target
pub struct Input {
	#[serde(rename = "startDate")]
	pub start: NaiveDate,
	#[serde(rename = "endDate")]
	pub end: NaiveDate,
	pub goals: Vec<Goal>,
}

#[cfg(test)]
impl Input {
	/// Create a new Input. Only useful for tests, otherwise input is
	/// deserialized as the input function.
	pub fn new(start: NaiveDate, end: NaiveDate, goals: Vec<Goal>) -> Self {
		Self { start, end, goals }
	}
}
