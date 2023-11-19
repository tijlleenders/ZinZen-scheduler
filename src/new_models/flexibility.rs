use std::rc::Rc;
use crate::new_models::goal::Goal;
use crate::new_models::day::Day;

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
