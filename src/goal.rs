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
#[derive(Deserialize, Debug, Default)]
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

#[cfg(test)]
impl Goal {
	pub fn new(id: usize) -> Self {
		Self {
			id,
			title: String::from("Test"),
			..Default::default()
		}
	}

	pub fn duration(mut self, duration: usize) -> Self {
		self.duration = duration;
		self
	}

	pub fn repetition(mut self, repetition: Repetition) -> Self {
		self.repetition = Some(repetition);
		self
	}

	pub fn start(mut self, start: OffsetDateTime) -> Self {
		self.start = Some(start);
		self
	}

	pub fn deadline(mut self, deadline: OffsetDateTime) -> Self {
		self.deadline = Some(deadline);
		self
	}
}
