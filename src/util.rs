use std::cmp::Ordering;
use std::fmt;

use chrono::{Duration, NaiveDateTime};

const MAX_SECONDS_TIMESTAMP_FOR_NANOS: i64 = 9_223_372_036;

/// DurationRound is not defined for NaiveDateTime, so do this.
/// Vendored from https://docs.rs/chrono/latest/src/chrono/round.rs.html#149-200
impl MyDurationRound for NaiveDateTime {
	type Err = MyRoundingError;

	fn duration_round(self, duration: Duration) -> Result<Self, Self::Err> {
		if let Some(span) = duration.num_nanoseconds() {
			if self.timestamp().abs() > MAX_SECONDS_TIMESTAMP_FOR_NANOS {
				return Err(MyRoundingError::TimestampExceedsLimit);
			}
			let stamp = self.timestamp_nanos();
			if span > stamp.abs() {
				return Err(MyRoundingError::DurationExceedsTimestamp);
			}
			let delta_down = stamp % span;
			if delta_down == 0 {
				Ok(self)
			} else {
				let (delta_up, delta_down) = if delta_down < 0 {
					(delta_down.abs(), span - delta_down.abs())
				} else {
					(span - delta_down, delta_down)
				};
				if delta_up <= delta_down {
					Ok(self + Duration::nanoseconds(delta_up))
				} else {
					Ok(self - Duration::nanoseconds(delta_down))
				}
			}
		} else {
			Err(MyRoundingError::DurationExceedsLimit)
		}
	}

	fn duration_trunc(self, duration: Duration) -> Result<Self, Self::Err> {
		if let Some(span) = duration.num_nanoseconds() {
			if self.timestamp().abs() > MAX_SECONDS_TIMESTAMP_FOR_NANOS {
				return Err(MyRoundingError::TimestampExceedsLimit);
			}
			let stamp = self.timestamp_nanos();
			if span > stamp.abs() {
				return Err(MyRoundingError::DurationExceedsTimestamp);
			}
			let delta_down = stamp % span;
			match delta_down.cmp(&0) {
				Ordering::Equal => Ok(self),
				Ordering::Greater => Ok(self - Duration::nanoseconds(delta_down)),
				Ordering::Less => Ok(self - Duration::nanoseconds(span - delta_down.abs())),
			}
		} else {
			Err(MyRoundingError::DurationExceedsLimit)
		}
	}
}

/// An error from rounding by `Duration`
///
/// See: [`DurationRound`]
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum MyRoundingError {
	/// Error when the Duration exceeds the Duration from or until the Unix epoch.
	///
	/// ``` rust
	/// # use chrono::{DateTime, DurationRound, Duration, RoundingError, TimeZone, Utc};
	/// let dt = Utc.ymd(1970, 12, 12).and_hms(0, 0, 0);
	///
	/// assert_eq!(
	///     dt.duration_round(Duration::days(365)),
	///     Err(RoundingError::DurationExceedsTimestamp),
	/// );
	/// ```
	DurationExceedsTimestamp,

	/// Error when `Duration.num_nanoseconds` exceeds the limit.
	///
	/// ``` rust
	/// # use chrono::{DateTime, DurationRound, Duration, RoundingError, TimeZone, Utc};
	/// let dt = Utc.ymd(2260, 12, 31).and_hms_nano(23, 59, 59, 1_75_500_000);
	///
	/// assert_eq!(
	///     dt.duration_round(Duration::days(300 * 365)),
	///     Err(RoundingError::DurationExceedsLimit)
	/// );
	/// ```
	DurationExceedsLimit,

	/// Error when `DateTime.timestamp_nanos` exceeds the limit.
	///
	/// ``` rust
	/// # use chrono::{DateTime, DurationRound, Duration, RoundingError, TimeZone, Utc};
	/// let dt = Utc.ymd(2300, 12, 12).and_hms(0, 0, 0);
	///
	/// assert_eq!(dt.duration_round(Duration::days(1)), Err(RoundingError::TimestampExceedsLimit),);
	/// ```
	TimestampExceedsLimit,
}

impl fmt::Display for MyRoundingError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			MyRoundingError::DurationExceedsTimestamp => {
				write!(f, "duration in nanoseconds exceeds timestamp")
			}
			MyRoundingError::DurationExceedsLimit => {
				write!(f, "duration exceeds num_nanoseconds limit")
			}
			MyRoundingError::TimestampExceedsLimit => {
				write!(f, "timestamp exceeds num_nanoseconds limit")
			}
		}
	}
}

#[cfg(any(feature = "std", test))]
impl std::error::Error for MyRoundingError {
	#[allow(deprecated)]
	fn description(&self) -> &str {
		"error from rounding or truncating with DurationRound"
	}
}

/// Extension trait for rounding or truncating a DateTime by a Duration.
///
/// # Limitations
/// Both rounding and truncating are done via [`Duration::num_nanoseconds`] and
/// [`DateTime::timestamp_nanos`]. This means that they will fail if either the
/// `Duration` or the `DateTime` are too big to represented as nanoseconds. They
/// will also fail if the `Duration` is bigger than the timestamp.
pub trait MyDurationRound: Sized {
	/// Error that can occur in rounding or truncating
	#[cfg(any(feature = "std", test))]
	type Err: std::error::Error;

	/// Error that can occur in rounding or truncating
	#[cfg(not(any(feature = "std", test)))]
	type Err: fmt::Debug + fmt::Display;

	/// Return a copy rounded by Duration.
	///
	/// # Example
	/// ``` rust
	/// # use chrono::{DateTime, DurationRound, Duration, TimeZone, Utc};
	/// let dt = Utc.ymd(2018, 1, 11).and_hms_milli(12, 0, 0, 154);
	/// assert_eq!(
	///     dt.duration_round(Duration::milliseconds(10)).unwrap().to_string(),
	///     "2018-01-11 12:00:00.150 UTC"
	/// );
	/// assert_eq!(
	///     dt.duration_round(Duration::days(1)).unwrap().to_string(),
	///     "2018-01-12 00:00:00 UTC"
	/// );
	/// ```
	fn duration_round(self, duration: Duration) -> Result<Self, Self::Err>;

	/// Return a copy truncated by Duration.
	///
	/// # Example
	/// ``` rust
	/// # use chrono::{DateTime, DurationRound, Duration, TimeZone, Utc};
	/// let dt = Utc.ymd(2018, 1, 11).and_hms_milli(12, 0, 0, 154);
	/// assert_eq!(
	///     dt.duration_trunc(Duration::milliseconds(10)).unwrap().to_string(),
	///     "2018-01-11 12:00:00.150 UTC"
	/// );
	/// assert_eq!(
	///     dt.duration_trunc(Duration::days(1)).unwrap().to_string(),
	///     "2018-01-11 00:00:00 UTC"
	/// );
	/// ```
	fn duration_trunc(self, duration: Duration) -> Result<Self, Self::Err>;
}
