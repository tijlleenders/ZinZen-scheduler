use serde::Deserialize;
use time::macros::datetime;
use time::PrimitiveDateTime;
use time::{Duration, OffsetDateTime};

use crate::goal::Goal;
use crate::task::{Slot, Task};
use crate::Input;

pub fn preprocess(Input { start, end, goals }: Input) -> (Vec<Task>) {
	goals
		.iter()
		.enumerate()
		.map(|(id, goal)| {
			let mut task = Task::new(id, goal.id, goal.duration);

			// TODO: multiple slots
			task.slots.push(Slot {
				start: (goal.start - start).whole_hours() as usize,
				end: (goal.deadline - start).whole_hours() as usize,
			});

			task
		})
		.collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
	use crate::Input;

	use super::*;

	// XXX: this unit test is temporary. Transform it into an integration test
	// once we have the scheduler working, so we can test input/output directly
	// without having to test internals
	#[test]
	fn temporary_basic_unit_test_for_scheduler() {
		let input: Input = serde_json::from_str(
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

		let result = preprocess(input);

		let mut t1 = Task::new(0, 1, 1);
		t1.slots.push(Slot { start: 10, end: 13 });
		let mut t2 = Task::new(1, 2, 1);
		t2.slots.push(Slot { start: 10, end: 11 });
		let mut t3 = Task::new(2, 3, 1);
		t3.slots.push(Slot { start: 10, end: 18 });

		assert_eq!(result, vec![t1, t2, t3])
	}
}
