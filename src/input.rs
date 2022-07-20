use serde::Deserialize;
use time::OffsetDateTime;

use crate::goal::Goal;

#[derive(Deserialize, Debug)]
/// Just a deserialization target
pub struct Input {
	#[serde(rename = "startDate")]
	#[serde(with = "time::serde::iso8601")]
	pub start: OffsetDateTime,
	#[serde(rename = "endDate")]
	#[serde(with = "time::serde::iso8601")]
	pub end: OffsetDateTime,
	pub goals: Vec<Goal>,
}

#[cfg(test)]
impl Input {
	/// Create a new Input. Only useful for tests, otherwise input is
	/// deserialized as the input function.
	pub fn new(start: OffsetDateTime, end: OffsetDateTime, goals: Vec<Goal>) -> Self {
		Self { start, end, goals }
	}
}
