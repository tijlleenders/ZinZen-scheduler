use std::fmt::Formatter;
use std::num::NonZeroUsize;

use serde::{Deserialize, Serialize};
use time::serde::iso8601;
use time::{Duration, OffsetDateTime, PrimitiveDateTime};

/// A [Goal] is what one wants to do, it is used in conjunction with a span of time to generate a [Schedule]
#[derive(Serialize, Deserialize, Debug)]
pub struct Goal {
	/// Every goal has a unique ID
	pub id: usize,
	/// A goal's description
	pub title: String,
	/// How much total time should a user put into their goal, eg "I want to learn how to code, and I want to code 6 hours per day"
	pub duration: usize,
	/// Earliest start datetime for this Goal's Tasks
	// TODO: should be optional (default to start of today)
	#[serde(with = "iso8601")]
	pub start: OffsetDateTime,
	/// Deadline for this Goal's Tasks
	// TODO: should be optional (will default to end of calendar))
	#[serde(with = "iso8601")]
	pub deadline: OffsetDateTime,
}
