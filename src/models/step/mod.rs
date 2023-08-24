use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use super::{
    goal::{Goal, Tag},
    slot::Slot,
    timeline::Timeline,
};

pub mod impls;

/// Steps are generated to achieve a Goal in one or more Steps.
/// A leaf Goal can generate one or more Steps.
#[derive(Deserialize, Debug, Eq, Clone)]
pub struct Step {
    /// Only used by the scheduler.
    /// Unstable between scheduler runs if input changes.
    pub id: usize,
    /// Reference to the Goal a Step was generated from.
    pub goal_id: String,
    /// Title of the Goal the Step was generated from.
    /// Duplicated for ease of debugging and simplicity of code.
    pub title: String,
    /// Duration the Step wants to claim on the Calendar.
    /// This duration is equal or part of the Goal duration.
    pub duration: usize,
    /// Used for finding next Step to be scheduled in combination with Step flexibility and Tags.
    pub status: StepStatus,
    /// Used for finding next Step to be scheduled in combination with Step Status and Tags.
    pub flexibility: usize,
    /// Final start time for Step on Calendar - should be removed in favor of Timeline + SlotStatus combination.
    pub start: Option<NaiveDateTime>,
    /// Final end time for Step on Calendar - should be removed in favor of Timeline + SlotStatus combination.
    pub deadline: Option<NaiveDateTime>,
    /// The places on Calendar that could potentially be used given the Goal constraints - and what other scheduled Steps already have consumed.
    pub slots: Vec<Slot>,
    /// Used for finding next Step to be scheduled in combination with Step flexibility and Status.
    #[serde(default)]
    pub tags: Vec<Tag>,
    /// Used for adding Blocked Step Tag, used in finding next Step to be scheduled.
    #[serde(default)]
    pub after_goals: Option<Vec<String>>,
}

/// Used to decide in which order to schedule steps, together with their flexibility
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum StepStatus {
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
}

#[derive(Debug, Clone)]
pub struct NewStep {
    pub step_id: usize,
    pub title: String,
    pub duration: usize,
    pub goal: Goal,
    pub timeline: Timeline,
    pub status: StepStatus,
    pub timeframe: Option<Slot>,
}
