use crate::models::goal::Goal;
use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub enum Slot<'a> {
    Empty(NaiveDateTime),
    Goal(NaiveDateTime, &'a Goal),
}

impl<'a> Slot<'a> {
    pub fn date(&self) -> &NaiveDateTime {
        match self {
            Slot::Empty(date) => date,
            Slot::Goal(date, _) => date,
        }
    }
    pub fn goal(&self) -> Option<&Goal> {
        if let Slot::Goal(_, goal) = self {
            Some(goal)
        } else {
            None
        }
    }
}
