use crate::models::day::Day;
use crate::models::goal::Goal;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Flexibility {
    pub goal: Rc<Goal>,
    pub day: Rc<Day>,
}
impl PartialEq for Flexibility {
    fn eq(&self, other: &Self) -> bool {
        self.goal.eq(&other.goal) && self.day.eq(&other.day)
    }
}
