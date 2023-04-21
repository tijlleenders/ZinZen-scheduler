use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use super::{goal::Tag, slot::Slot};

pub mod impls;

/// Tasks are generated from Goals, and represent a concrete activity of a
/// specified duration, that is to be carried out at a specific time.
/// A task can be in a number of different Task Statuses.  
/// A Goal can generate one or more Tasks.
/// # Examples (greatly simplified to highlight the concept)
/// | User Enters | Goal | Task(s) |
/// | ----------- | ---- | ------- |
/// | 'Dentist 1hr after 10 before 11' | Goal {..., after_time: 10, before_time: 11} | Task {..., id: 1, confirmed_start: 2023-01-23T10:00:00 , confirmed_deadline: 2023-01-23T11:00:00} |
/// | 'Read 1hr daily' | Goal {..., repeat: daily} | Task {..., id: 1, confirmed_start: 2023-01-23T17:00:00, confirmed_deadline: 2023-01-23T18:00:00 }, Task {..., id: 2, confirmed_start: 2023-01-24T17:00:00, confirmed_deadline: 2024-01-24T18:00:00 }  
#[derive(Deserialize, Debug, Eq, Clone)]
pub struct Task {
    pub id: usize,
    pub goal_id: String,
    pub title: String,
    pub duration: usize,
    pub status: TaskStatus,
    pub flexibility: usize,
    pub start: Option<NaiveDateTime>,
    pub deadline: Option<NaiveDateTime>,
    pub slots: Vec<Slot>,
    #[serde(default)]
    pub tags: Vec<Tag>,
    #[serde(default)]
    pub after_goals: Option<Vec<String>>,
    pub calendar_start: NaiveDateTime,
    pub calendar_end: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum TaskStatus {
    Scheduled,
    Impossible,
    Uninitialized,
    Blocked,
    ReadyToSchedule,
    BudgetMinWaitingForAdjustment,
    BudgetMaxWaitingForAdjustment,
}
