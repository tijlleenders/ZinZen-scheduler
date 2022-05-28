use crate::{
	error::{Explode, SchedulerError, SchedulerResult},
	task::Task,
};
use linked_list::LinkedList;

/// A schedule is just a list of tasks which satisfy the user's time and location constraints
#[derive(Debug)]
pub struct Schedule {
	pub(crate) slots: LinkedList<Task>,
	pub(crate) timeline: (usize, usize),
}

impl Schedule {
	/// Generate a vector of slots from a list of slots
	pub(crate) fn slots_vector(&self) -> Vec<Task> {
		self.slots.iter().cloned().collect::<Vec<_>>()
	}
}