use time::macros::datetime;
use time::Duration;
use time::PrimitiveDateTime;

use crate::goal::Goal;
use crate::task::{Slot, Task};

/// The [PreProcessor] takes in a user's goals, given a duration
pub struct PreProcessor;

impl PreProcessor {
	pub fn preprocess(goals: &[Goal], timeline: (PrimitiveDateTime, PrimitiveDateTime)) -> (Vec<Task>) {
		let tasks = goals
			.iter()
			.enumerate()
			.map(|(id, goal)| Task::new(id, goal.id, goal.duration));

		tasks
			.map(|mut task| {
				task.slots.push(Slot {
					begin: datetime!(1970-01-01 0:00),
					end: datetime!(1970-01-01 0:00),
				});
				task
			})
			.collect::<Vec<_>>()
	}
}

#[cfg(test)]
mod tests {
	use crate::Plan;

	use super::*;

	#[test]
	fn t() {
		let plan: Plan = serde_json::from_str(
			r#"
{
    "startDate": "2022-01-01T00:00:00Z",
    "endDate": "2022-01-02T00:00:00Z",
    "goals": [
        {
          "goalId": "1",
          "title" : "shopping",
          "duration": 1,
          "start": "2022-01-01T10:00:00Z",
          "deadline": "2022-01-01T13:00:00Z"
        },
        {
          "goalId": "2",
          "title": "dentist",
          "duration": 1,
          "start": "2022-01-01T10:00:00Z",
          "deadline": "2022-01-01T11:00:00Z"
        },
        {
          "goalId": "3",
          "title" : "exercise",
          "duration": 1,
          "start": "2022-01-01T10:00:00Z",
          "deadline": "2022-01-01T18:00:00Z"
        }
    ]
}
        "#,
		)
		.unwrap();
	}
}
