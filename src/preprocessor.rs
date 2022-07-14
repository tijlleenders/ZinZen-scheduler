use serde::Deserialize;
use time::macros::datetime;
use time::PrimitiveDateTime;
use time::{Duration, OffsetDateTime};

use crate::goal::Goal;
use crate::task::{Slot, Task};

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

#[cfg(test)]
mod tests {
	use crate::Input;

	use super::*;

	#[test]
	fn t() {
		let plan: Input = serde_json::from_str(
			r#"
{
    "startDate": "2022-01-01T00:00:00Z",
    "endDate": "2022-01-02T00:00:00Z",
    "goals": [
        {
          "id": 1,
          "title" : "shopping",
          "duration": 1,
          "start": "2022-01-01T10:00:00Z",
          "deadline": "2022-01-01T13:00:00Z"
        },
        {
          "id": 2,
          "title": "dentist",
          "duration": 1,
          "start": "2022-01-01T10:00:00Z",
          "deadline": "2022-01-01T11:00:00Z"
        },
        {
          "id": 3,
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
		dbg!(plan);
	}
}
