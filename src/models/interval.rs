use std::fmt;

#[derive(Clone, PartialEq)]
pub struct Interval {
    pub start: usize,
    pub end: usize,
}

impl fmt::Debug for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}-{} - {}:00-{}:00",
            self.start,
            self.end,
            self.start % 24,
            self.end % 24
        )
    }
}
