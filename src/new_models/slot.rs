use crate::new_models::goal::Goal;
use crate::new_models::date::DateTime;

#[derive(Debug, Clone)]
pub enum Slot<'a> {
    Empty(DateTime),
    Goal(DateTime, &'a Goal),
}

impl<'a> Slot<'a> {
    pub fn date(&self) -> &DateTime {
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
