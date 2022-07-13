//! The preprocessor formats the input so the core-processor can focus on scheduling.
//! It takes the goal-tree defined in the UI and turns it into a flat list of tasks
//! with 0-* corresponding time-constrained slots.
//!
//! https://github.com/tijlleenders/ZinZen-scheduler/wiki/preprocessor

use std::marker::PhantomData;
use std::time::Duration;

use chrono::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::{Calendar, Slot, Task, TaskStatus};

#[derive(Deserialize, Debug)]
struct Goal {
    #[serde(rename = "goalId")]
    id: String,
    title: String,
    // TODO this will need to be time units later. Right now hardcoded to be 1 hour
    duration: usize,
    // TODO: start and deadline may be optional
    start: DateTime<Utc>,
    // TODO: start and deadline may be optional
    deadline: DateTime<Utc>,
    preferred_time: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Debug)]
struct Input {
    #[serde(rename = "startDate")]
    start_date: DateTime<Utc>,
    #[serde(rename = "endDate")]
    end_date: DateTime<Utc>,
    // TODO timezone
    goals: Vec<Goal>,
    // TODO relationships
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct ProcessedInput {
    tasks: Vec<Task>,
    slots: Vec<Slot>,
}

fn preprocessor(input: Input) -> ProcessedInput {
    // TODO repetition
    // TODO filler

    // TODO should verify that goals are valid: time is not less than Input.start_date

    let tasks = input
        .goals
        .iter()
        .enumerate()
        .map(|(id, goal)| Task {
            task_id: id,
            goal_id: goal.id.clone(),
            duration_to_schedule: goal.duration,
            task_status: TaskStatus::UNSCHEDULED,
            ..Default::default()
        })
        .collect::<Vec<_>>();

    // TODO a goal may yield multiple slots
    let slots = input
        .goals
        .iter()
        .enumerate()
        .map(|(id, goal)| Slot {
            task_id: id,
            begin: (goal.start - input.start_date).num_hours() as usize,
            end: (goal.deadline - input.start_date).num_hours() as usize,
        })
        .collect::<Vec<_>>();

    ProcessedInput { tasks, slots }
}

#[cfg(test)]
mod test {
    use serde_json;

    use crate::TaskStatus::{SCHEDULED, UNSCHEDULED};

    use super::*;

    #[test]
    fn test() {
        let input: Input = serde_json::from_str(
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

        assert_eq!(
            preprocessor(input),
            ProcessedInput {
                tasks: vec![
                    Task {
                        task_id: 0,
                        duration_to_schedule: 1,
                        duration_scheduled: 0,
                        task_status: UNSCHEDULED,
                        goal_id: "1".into(),
                    },
                    Task {
                        task_id: 1,
                        duration_to_schedule: 1,
                        duration_scheduled: 0,
                        task_status: UNSCHEDULED,
                        goal_id: "2".into(),
                    },
                    Task {
                        task_id: 2,
                        duration_to_schedule: 1,
                        duration_scheduled: 0,
                        task_status: UNSCHEDULED,
                        goal_id: "3".into(),
                    },
                ],
                slots: vec![
                    Slot {
                        task_id: 0,
                        begin: 10,
                        end: 13,
                    },
                    Slot {
                        task_id: 1,
                        begin: 10,
                        end: 11,
                    },
                    Slot {
                        task_id: 2,
                        begin: 10,
                        end: 18,
                    },
                ],
            }
        );
    }
}
