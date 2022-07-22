use std::option::Option;

use chrono::prelude::*;
use chrono::Duration;
use serde::Deserialize;

/// How often can a task repeat
#[derive(Deserialize, Debug, Copy, Clone)]
pub enum Repetition {
	#[serde(rename = "daily")]
	DAILY,
}

impl From<Repetition> for Duration {
	fn from(a: Repetition) -> Self {
		match a {
			Repetition::DAILY => Duration::days(1),
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
	pub start: Option<NaiveDateTime>,
	/// Deadline for this Goal's Tasks
	#[serde(default)]
	pub deadline: Option<NaiveDateTime>,
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

	pub fn start(mut self, start: NaiveDateTime) -> Self {
		self.start = Some(start);
		self
	}

	pub fn deadline(mut self, deadline: NaiveDateTime) -> Self {
		self.deadline = Some(deadline);
		self
	}
}
