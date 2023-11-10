use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime};

const SLOT_DURATION: Duration = Duration::hours(1);

#[derive(Clone)]
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
            current += SLOT_DURATION;
        }
        DateTimeRange::new(self.clone(), Self::from_naive_date_time(&current))
    }
    pub fn inc(&self) -> Self {
        self.inc_by(1)
    }
    pub fn inc_by(&self, span: usize) -> Self {
        let mut out = self.clone();
        for _ in 0..span {
            out.naive += SLOT_DURATION;
        }
        out
    }
    pub fn dec(&self) -> Self {
        self.dec_by(1)
    }
    pub fn dec_by(&self, span: usize) -> Self {
        let mut out = self.clone();
        for _ in 0..span {
            out.naive -= SLOT_DURATION;
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

    /// The span in slot durations in between the range
    pub fn span(&self) -> usize {
        let mut out = 0;

        let mut current = self.start.clone();
        while current <= self.end {
            out += 1;
            current += SLOT_DURATION;
        }

        out
    }

    pub fn contains_date_time(&self, date: &DateTime) -> bool {
        &date.naive.ge(&self.start) && &date.naive.lt(&self.end)
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
        out += SLOT_DURATION;
        if &out > date_time {
            out -= SLOT_DURATION;
            break;
        }
    }

    out
}
