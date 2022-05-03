use crate::{goal::Goal, task::Task};

pub struct Schedule<'a> {
	goals: Vec<Goal>,
	tasks: Vec<Task<'a>>,
}
