use crate::new_models::calendar::Span;
use crate::new_models::date::DateTimeRangeContainerResult::{
    FitAtEnd, FitAtStart, FitInBetween, NoFit, PerfectFit,
};
use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime};
use lazy_static::lazy_static;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::ops::Add;
use std::str::FromStr;

lazy_static! {
    static ref SLOT_DURATION: Duration = Duration::hours(1);
}

#[derive(Debug, Clone)]
pub struct DateTime {
    naive: NaiveDateTime,
}
impl DateTime {
    pub fn from_naive_date_time(date_time: &NaiveDateTime) -> Self {
        Self {
            naive: normalize_date(date_time),
        }
    }
    pub fn from_naive_date(date: &NaiveDate) -> Self {
        Self {
            naive: NaiveDateTime::new(*date, NaiveTime::default()),
        }
    }
    pub fn from_naive_time(time: &NaiveTime) -> Self {
        Self {
            naive: normalize_date(&NaiveDateTime::new(NaiveDate::default(), *time)),
        }
    }
    fn from_str(date_str: &str) -> Option<Self> {
        NaiveDateTime::from_str(date_str)
            .map(|ndt| Self::from_naive_date_time(&ndt))
            .ok()
    }
    pub fn make_range(&self, span: usize) -> DateTimeRange {
        let mut current = self.naive;
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
    pub fn start_of_day(&self) -> DateTime {
        DateTime::from_naive_date_time(&NaiveDateTime::new(self.naive.date(), NaiveTime::default()))
    }
    pub fn end_of_day(&self) -> DateTime {
        let beginning = self.start_of_day();
        DateTime::from_naive_date_time(&beginning.naive.add(Duration::days(1)))
    }
    pub fn span_of_day(&self) -> Span {
        let end_of_day = self.end_of_day();
        let mut current = self.start_of_day();
        let mut count = 0;
        while current.lt(&end_of_day) {
            count += 1;
            current = current.inc();
        }
        count
    }
    pub fn span_by(&self, other: &DateTime) -> Span {
        let mut current = self.clone();
        let mut count = 0;
        while current.le(other) {
            current = current.inc();
            count += 1;
        }
        if count > 0 {
            count -= 1;
        }
        count
    }
    pub fn with_new_time(&self, time: &DateTime) -> DateTime {
        DateTime::from_naive_date_time(&NaiveDateTime::new(self.naive.date(), time.naive.time()))
    }
    pub fn time_after(&self, other: &Option<DateTime>) -> DateTime {
        other
            .as_ref()
            .map(|time| self.with_new_time(time))
            .unwrap_or(self.start_of_day())
    }
    pub fn time_before(&self, other: &Option<DateTime>) -> DateTime {
        other
            .as_ref()
            .map(|time| self.with_new_time(time))
            .unwrap_or(self.end_of_day())
    }
    pub fn naive_date(&self) -> NaiveDate {
        self.naive.date()
    }
    pub fn naive_date_time(&self) -> NaiveDateTime {
        self.naive
    }
}
impl PartialEq<Self> for DateTime {
    fn eq(&self, other: &Self) -> bool {
        self.naive.eq(&other.naive)
    }
}
impl PartialOrd<Self> for DateTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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
impl From<&DateTime> for NaiveDateTime {
    fn from(date: &DateTime) -> Self {
        date.naive
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DateTimeRangeContainerResult {
    NoFit,
    PerfectFit,
    FitAtStart(DateTimeRange, DateTimeRange),
    FitAtEnd(DateTimeRange, DateTimeRange),
    FitInBetween(DateTimeRange, DateTimeRange, DateTimeRange),
}

#[derive(Debug, Clone)]
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
                end: self.end.dec_by(span.unsigned_abs() as usize),
            }
        }
    }

    pub fn is_fitting(&self, range: &DateTimeRange) -> DateTimeRangeContainerResult {
        if self.contains(range) {
            match (&range.start, &range.end) {
                (start, end) if start == &self.start && end == &self.end => PerfectFit,
                (start, _) if start == &self.start => FitAtStart(
                    DateTimeRange::new(range.start.clone(), range.end.clone()),
                    DateTimeRange::new(range.end.clone(), self.end.clone()),
                ),
                (_, end) if end == &self.end => FitAtEnd(
                    DateTimeRange::new(self.start.clone(), range.start.clone()),
                    DateTimeRange::new(range.start.clone(), range.end.clone()),
                ),
                _ => FitInBetween(
                    DateTimeRange::new(self.start.clone(), range.start.clone()),
                    DateTimeRange::new(range.start.clone(), range.end.clone()),
                    DateTimeRange::new(range.end.clone(), self.end.clone()),
                ),
            }
        } else {
            NoFit
        }
    }
}
impl PartialEq<Self> for DateTimeRange {
    fn eq(&self, other: &Self) -> bool {
        self.start().eq(&other.start()) && self.end().eq(&other.end())
    }
}
impl PartialOrd<Self> for DateTimeRange {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.start().cmp(other.start()))
    }
}
impl Eq for DateTimeRange {}
impl Ord for DateTimeRange {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start().cmp(&other.start())
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

    #[test]
    fn test_date_time_range() {
        let (range, date_start, date_end) = create_range("2022-01-01T00:00:00", 3);
        assert_eq!(range.start(), &date_start);
        assert_eq!(range.end(), &date_end);

        let range = range.shift(2);
        assert_eq!(range.start(), &date_start.inc_by(2));
        assert_eq!(range.end(), &date_end.inc_by(2));

        let date_between = DateTime::from_str("2022-01-01T04:00:00").unwrap();
        assert!(range.contains_date_time(&date_between));
        let date_between = DateTime::from_str("2022-01-01T03:00:00").unwrap();
        assert!(range.contains_date_time(&date_between));
        let date_between = DateTime::from_str("2022-01-01T01:00:00").unwrap();
        assert!(!range.contains_date_time(&date_between));
        let date_between = DateTime::from_str("2022-01-01T05:00:00").unwrap();
        assert!(!range.contains_date_time(&date_between));
    }

    #[test]
    fn test_date_time_range_contains() {
        let date_start = DateTime::from_str("2022-01-01T03:00:00").unwrap();
        let range = date_start.make_range(3);

        let range1 = DateTime::from_str("2022-01-01T04:00:00")
            .unwrap()
            .make_range(1);
        assert!(range.contains(&range1));

        let range1 = DateTime::from_str("2022-01-01T02:00:00")
            .unwrap()
            .make_range(6);
        assert!(!range.contains(&range1));

        let range1 = DateTime::from_str("2022-01-01T02:00:00")
            .unwrap()
            .make_range(4);
        assert!(!range.contains(&range1));

        let range1 = DateTime::from_str("2022-01-01T03:00:00")
            .unwrap()
            .make_range(4);
        assert!(!range.contains(&range1));
    }

    #[test]
    fn test_is_fitting() {
        let (ref_range, _, _) = create_range("2022-01-01T00:00:00", 3);

        let (range, _, _) = create_range("2022-01-01T00:00:00", 3);
        assert_eq!(ref_range.is_fitting(&range), PerfectFit);

        let (range, _, _) = create_range("2022-01-01T00:00:00", 2);
        let (start, _, _) = create_range("2022-01-01T00:00:00", 2);
        let (end, _, _) = create_range("2022-01-01T02:00:00", 1);
        assert_eq!(ref_range.is_fitting(&range), FitAtStart(start, end));

        let (range, _, _) = create_range("2022-01-01T01:00:00", 2);
        let (start, _, _) = create_range("2022-01-01T00:00:00", 1);
        let (end, _, _) = create_range("2022-01-01T01:00:00", 2);
        assert_eq!(ref_range.is_fitting(&range), FitAtEnd(start, end));

        let (range, _, _) = create_range("2022-01-01T01:00:00", 1);
        let (start, _, _) = create_range("2022-01-01T00:00:00", 1);
        let (mid, _, _) = create_range("2022-01-01T01:00:00", 1);
        let (end, _, _) = create_range("2022-01-01T02:00:00", 1);
        assert_eq!(ref_range.is_fitting(&range), FitInBetween(start, mid, end));

        let (range, _, _) = create_range("2022-01-01T00:00:00", 5);
        assert_eq!(ref_range.is_fitting(&range), NoFit);
    }

    fn create_range(date: &str, span: usize) -> (DateTimeRange, DateTime, DateTime) {
        let date_start = DateTime::from_str(date).unwrap();
        let date_end = date_start.inc_by(span);
        assert!(date_start < date_end);
        assert!(!(date_start >= date_end));
        (date_start.make_range(span), date_start, date_end)
    }
}
