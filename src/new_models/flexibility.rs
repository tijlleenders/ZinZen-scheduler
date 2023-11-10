use crate::models::goal::Goal;
use crate::new_models::slot::Slot;

#[derive(Debug)]
pub struct Flexibility<'a> {
    pub flexibility: usize,
    pub goal: &'a Goal,
    pub slots: Vec<Slot<'a>>,
}
