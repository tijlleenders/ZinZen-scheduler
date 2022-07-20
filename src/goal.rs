use std::option::Option;

use serde::Deserialize;
use time::OffsetDateTime;

/// How often can a task repeat
#[derive(Deserialize, Debug, Copy, Clone)]
pub enum Repetition {
	#[serde(rename = "daily")]
	DAILY,
}

impl Repetition {
	pub fn into_hours(self) -> i64 {
		match self {
			Self::DAILY => 24,
		}
	}
}

/// A [Goal] is what one wants to do, it is used in conjunction with a span of time to generate a [Schedule]
#[derive(Deserialize, Debug)]
pub struct Goal {
	/// Every goal has a unique ID
	pub id: usize,
	/// A goal's description
	pub title: String,
	/// How much total time should a user put into their goal, eg "I want to learn how to code, and I want to code 6 hours per day"
	pub duration: usize,
	pub repetition: Option<Repetition>,
	/// Earliest start datetime for this Goal's Tasks
	#[serde(default)]
	#[serde(with = "time::serde::iso8601::option")]
	pub start: Option<OffsetDateTime>,
	/// Deadline for this Goal's Tasks
	#[serde(default)]
	#[serde(with = "time::serde::iso8601::option")]
	pub deadline: Option<OffsetDateTime>,
}
