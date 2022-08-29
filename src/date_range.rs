use crate::util::MyDurationRound;
use chrono::prelude::*;
use chrono::Duration;

/// A range of datetimes with an interval.
pub(crate) struct DateRange {
	pub(crate) start: NaiveDateTime,
	pub(crate) end: NaiveDateTime,
	pub(crate) interval: Option<Duration>,
}

impl Iterator for DateRange {
	type Item = (NaiveDateTime, NaiveDateTime);
	fn next(&mut self) -> Option<Self::Item> {
		if self.interval.is_none() {
			return Some((self.start, self.end));
		}
		if self.start < self.end {
			let start = self.start;
			let mut end = self.start + self.interval.unwrap(); //It's okay to unwrap coz we've
												   //handled case where is_none() above
			if end > self.end {
				end = self.end;
			} else {
				end = end.duration_round(self.interval.unwrap()).ok()?;
			}
			self.start = end;
			Some((start, end))
		} else {
			None
		}
	}
}
