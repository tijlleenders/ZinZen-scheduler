use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use super::{
    goal::{Goal, Tag},
    slot::Slot,
    timeline::Timeline,
};

pub mod impls;

/// Tasks/Increments are generated to achieve a Goal in one or more Increments.
/// A Goal can generate one or more Tasks.
#[derive(Deserialize, Debug, Eq, Clone)]
pub struct Task {
    /// Only used by the scheduler.
    /// Unstable between scheduler runs if input changes.
    pub id: usize,
    /// Reference to the Goal a Taks/Increment was generated from.
    pub goal_id: String,
    /// Title of the Goal the Task/Increment was generated from.
    /// Duplicated for ease of debugging and simplicity of code.
    pub title: String,
    /// Duration the Task/Increment wants to claim on the Calendar.
    /// This duration is equal or part of the Goal duration.
    pub duration: usize,
    /// Used for finding next Task/Increment to be scheduled in combination with Task/Increment flexibility and Tags.
    pub status: TaskStatus,
    /// Used for finding next Task/Increment to be scheduled in combination with Task/Increment Status and Tags.
    pub flexibility: usize,
    /// Final start time for Task/Increment on Calendar - should be removed in favor of Timeline + SlotStatus combination.
    pub start: Option<NaiveDateTime>,
    /// Final end time for Task/Increment on Calendar - should be removed in favor of Timeline + SlotStatus combination.
    pub deadline: Option<NaiveDateTime>,
    /// The places on Calendar that could potentially be used given the Goal constraints - and what other scheduled Tasks/Increments already have consumed.
    pub slots: Vec<Slot>,
    /// Used for finding next Task/Increment to be scheduled in combination with Task/Increment flexibility and Status.
    #[serde(default)]
    pub tags: Vec<Tag>,
    /// Used for adding Blocked Task/Increment Tag, used in finding next Task/Increment to be scheduled.
    #[serde(default)]
    pub after_goals: Option<Vec<String>>,
}

/// Used to decide in which order to schedule tasks, together with their flexibility
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum TaskStatus {
    /// Task is scheduled and can't be modified any more.
    Scheduled,
    /// Task is impossible - its MaybeSlots Timeline is removed.
    Impossible,
    /// Task is waiting for something to be properly initialized.
    Uninitialized,
    /// Task is waiting for another Goal to be scheduled first.
    Blocked,
    /// Task is available for scheduling, but its relative flexibility and Tags will determine if it gets picked first
    ReadyToSchedule,
    /// Special Task that will try to fill in any missing hours to reach the minimum budget for a time period.
    BudgetMinWaitingForAdjustment,
}

#[derive(Debug, Clone)]
pub struct NewTask {
    pub task_id: usize,
    pub title: String,
    pub duration: usize,
    pub goal: Goal,
    pub timeline: Timeline,
    pub status: TaskStatus,
    pub timeframe: Option<Slot>,
}
