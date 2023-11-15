use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime};
use lazy_static::lazy_static;

lazy_static!{
    static ref SLOT_DURATION: Duration = Duration::hours(1);
}

#[derive(Debug, Clone)]
pub struct DateTime {
    naive: NaiveDateTime,
}
impl DateTime {
    pub fn from_naive_date_time(date_time: &NaiveDateTime) -> Self {
        Self { naive: normalize_date(date_time) }
    }
    pub fn from_naive_date(date: &NaiveDate) -> Self {
        Self { naive: NaiveDateTime::new(date.clone(), NaiveTime::default()) }
    }
    pub fn from_naive_time(time: &NaiveTime) -> Self {
        Self { naive: normalize_date(&NaiveDateTime::new(NaiveDate::default(), time.clone())) }
    }
    pub fn make_range(&self, span: usize) -> DateTimeRange {
        let mut current = self.naive.clone();
        for _ in 0..span {
            current += *SLOT_DURATION;
        }
        DateTimeRange::new(self.clone(), Self::from_naive_date_time(&current))
    }
    pub fn inc(&self) -> Self {
        self.inc_by(1)
    }
    pub fn inc_by(&self, span: usize) -> Self {
        let mut out = self.clone();
        for _ in 0..span {
            out.naive += *SLOT_DURATION;
        }
        out
    }
    pub fn dec(&self) -> Self {
        self.dec_by(1)
    }
    pub fn dec_by(&self, span: usize) -> Self {
        let mut out = self.clone();
        for _ in 0..span {
            out.naive -= *SLOT_DURATION;
        }
        out
    }
}
impl PartialEq<Self> for DateTime {
    fn eq(&self, other: &Self) -> bool {
        self.naive.eq(&other.naive)
    }
}
impl PartialOrd<Self> for DateTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.naive.partial_cmp(&other.naive)
    }
}
impl Eq for DateTime {}
impl Ord for DateTime {
    fn cmp(&self, other: &Self) -> Ordering {
        self.naive.cmp(&other.naive)
    }
}
impl Display for DateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.naive.fmt(f)
    }
}

#[derive(Debug)]
pub struct DateTimeRange {
    start: DateTime,
    end: DateTime,
}
impl DateTimeRange {
    pub fn new(start: DateTime, end: DateTime) -> Self {
        if start < end {
            Self { start, end }
        } else {
            Self { end, start }
        }
    }

    pub fn start(&self) -> &DateTime {
        &self.start
    }
    pub fn end(&self) -> &DateTime {
        &self.end
    }

    /// The span in slot durations in between the range
    pub fn span(&self) -> usize {
        let mut out = 0;

        let mut current = self.start.clone();
        while current <= self.end {
            out += 1;
            current.naive += *SLOT_DURATION;
        }

        out
    }

    pub fn contains_date_time(&self, date: &DateTime) -> bool {
        println!("{:?}<={:?}<{:?}", self.start, date, self.end);
        self.start.naive.le(&date.naive) && date.naive.lt(&self.end.naive)
    }

    pub fn contains(&self, range: &DateTimeRange) -> bool {
        self.start <= range.start && range.end <= self.end
    }

    pub fn shift(&self, span: i16) -> DateTimeRange {
        if span.is_positive() {
            Self {
                start: self.start.inc_by(span as usize),
                end: self.end.inc_by(span as usize),
            }
        } else {
            Self {
                start: self.start.dec_by(span as usize),
                end: self.end.dec_by(span.abs() as usize),
            }
        }
    }
}

fn normalize_date(date_time: &NaiveDateTime) -> NaiveDateTime {
    let mut out = NaiveDateTime::new(date_time.date(), NaiveTime::default());

    loop {
        out += *SLOT_DURATION;
        if &out > date_time {
            out -= *SLOT_DURATION;
            break;
        }
    }

    out
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;
    use crate::models::date::create_date_by_hour;

    #[test]
    fn test_date_time_range() {
        let date_start = DateTime::from_naive_date_time(&create_date("2022-01-01T00:00:00"));
        let date_end = date_start.inc_by(3);
        assert!(date_start < date_end);
        assert!(!(date_start >= date_end));

        let range = date_start.make_range(3);
        assert_eq!(range.start(), &date_start);
        assert_eq!(range.end(), &date_end);

        let range = range.shift(2);
        assert_eq!(range.start(), &date_start.inc_by(2));
        assert_eq!(range.end(), &date_end.inc_by(2));

        let date_between = DateTime::from_naive_date_time(&create_date("2022-01-01T04:00:00"));
        assert!(range.contains_date_time(&date_between));
        let date_between = DateTime::from_naive_date_time(&create_date("2022-01-01T03:00:00"));
        assert!(range.contains_date_time(&date_between));
        let date_between = DateTime::from_naive_date_time(&create_date("2022-01-01T01:00:00"));
        assert!(!range.contains_date_time(&date_between));
        let date_between = DateTime::from_naive_date_time(&create_date("2022-01-01T05:00:00"));
        assert!(!range.contains_date_time(&date_between));
    }

    #[test]
    fn test_date_time_range_contains() {
        let date_start = DateTime::from_naive_date_time(&create_date("2022-01-01T03:00:00"));
        let range = date_start.make_range(3);

        let range1 = DateTime::from_naive_date_time(&create_date("2022-01-01T04:00:00"))
            .make_range(1);
        assert!(range.contains(&range1));

        let range1 = DateTime::from_naive_date_time(&create_date("2022-01-01T02:00:00"))
            .make_range(6);
        assert!(!range.contains(&range1));

        let range1 = DateTime::from_naive_date_time(&create_date("2022-01-01T02:00:00"))
            .make_range(4);
        assert!(!range.contains(&range1));

        let range1 = DateTime::from_naive_date_time(&create_date("2022-01-01T03:00:00"))
            .make_range(4);
        assert!(!range.contains(&range1));

    }

    fn create_date(date_str: &str) -> NaiveDateTime {
        NaiveDateTime::from_str(date_str).unwrap()
    }
}
