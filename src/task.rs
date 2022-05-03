use crate::goal::Goal;

/// A [Task] is an item a user is expected to accomplish, it is defined in a user's schedule, which itself is just a contiguous list of tasks
pub struct Task<'a> {
	pub(crate) goal: &'a Goal,
	pub(crate) completed: bool,
}

impl<'a> Task<'a> {
	pub fn serialize_json(&self) -> String {
		format!("{{\"goal_id\": {}, \"completed\": {}}}", self.goal.id, self.completed)
	}
}

impl<'a> Task<'a> {
	pub fn new(goal: &Goal) -> Task {
		Task { goal, completed: false }
	}
}

/// A simple wrapper around an slice of tasks for easy serialization
pub(crate) struct Tasks<'a>(&'a [Task<'a>]);

impl<'a> Tasks<'a> {
	pub fn serialize_json(&self) -> String {
		let mut json = String::new();

		self.0.into_iter().for_each(|task| {
			if json.is_empty() {
				json.push_str("[");
			} else {
				json.push_str(",");
			}

			let serialized_task = task.serialize_json();
			json.push_str(&serialized_task);
		});

		if json.is_empty() {
			json.push_str(&"[]");
		} else {
			json.push(']');
		}

		json
	}
}

impl<'a> From<&'a [Task<'a>]> for Tasks<'a> {
	fn from(tasks: &'a [Task<'a>]) -> Self {
		Self(tasks)
	}
}
