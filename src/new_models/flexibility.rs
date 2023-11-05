use crate::new_models::slot::Slot;

#[derive(Debug)]
pub struct Flexibility<'a> {
    pub flexibility: usize,
    pub slots: Vec<Slot<'a>>,
}
