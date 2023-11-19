use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use crate::new_models::calendar::{Flex, Span};
use crate::new_models::date::{DateTime, DateTimeRange};

#[derive(Debug, PartialEq)]
enum Seat {
    Occupied,
    Free,
}

#[derive(Debug, PartialEq)]
pub struct Day {
    range: DateTimeRange,
    seats: Vec<RefCell<Seat>>,
}
impl Day {
    pub fn new(date: DateTime) -> Self {
        let start = date.start_of_day();
        let end = date.end_of_day();
        let mut current = start.clone();

        let mut seats = vec![];
        while current.lt(&end) {
            seats.push(RefCell::new(Seat::Free));
            current = current.inc();
        }

        let range = DateTimeRange::new(start, end);
        Self { range, seats }
    }
    pub fn occupy(&self, range: &DateTimeRange) {
        if !self.range.contains(range) {
            return;
        }

        for idx in 0..self.seats.len() {
            let date = self.range.start().start_of_day().inc_by(idx);
            if range.contains_date_time(&date) {
                *self.seats[idx].borrow_mut() = Seat::Occupied;
            }
        }
    }
    pub fn occupy_inverse_range(&self, range: &DateTimeRange) {
        self.occupy(&DateTimeRange::new(self.range.start().start_of_day(), range.start().clone()));
        self.occupy(&DateTimeRange::new(range.end().clone(), self.range.end().start_of_day()));
    }
    pub fn flexibility(&self, span: Span) -> Flex {
        self.slots(span).len()
    }
    pub fn slots(&self, span: usize) -> Vec<DateTimeRange> {
        if self.seats.is_empty() {
            return vec![];
        }
        let mut out = vec![];
        let start_of_day = self.range.start();
        for idx in 0..self.seats.len() {
            if self.all_occupied(idx, span) {
                let date = start_of_day.inc_by(idx);
                out.push(DateTimeRange::new(date.clone(), date.inc_by(span).clone()));
            }
        }
        out
    }
    pub fn overlap(&self, range: &Vec<DateTimeRange>) -> Vec<(usize, DateTimeRange)> {
        let start_of_day = self.range.start();
        let mut out = vec![];
        for r in range {
            let mut count = 0;
            for (idx, seat) in self.seats.iter().enumerate() {
                if r.contains_date_time(&start_of_day.inc_by(idx)) && *seat.borrow() == Seat::Free {
                    count += 1;
                }
            }
            out.push(count);
        }
        out.iter().zip(range).map(|(idx, range)| (*idx, range.clone())).collect()
    }
    pub fn first_fit(&self, span: usize) -> DateTimeRange {
        self.slots(span)[0].clone()
    }
    fn all_occupied(&self, idx: usize, span: usize) -> bool {
        self.seats[idx..idx + span].iter()
            .all(|seat| match *seat.borrow() {
                Seat::Occupied => true,
                Seat::Free => false,
            })
    }
    pub fn differences(&self, other: &Day) -> usize {
        self.seats.iter().zip(&other.seats)
            .map(|(a, b)|
                if a == b { 0 } else { 1 }
            )
            .sum()
    }
}

impl Display for Day {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self.seats.iter()
                .map(|s| match *s.borrow() {
                    Seat::Free => ".",
                    Seat::Occupied => "#",
                })
                .collect::<Vec<_>>()
                .join("")
        )
    }
}
