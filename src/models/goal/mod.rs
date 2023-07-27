pub mod impls;

use crate::models::repetition::Repetition;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    option::Option,
    sync::{Once, RwLock},
};

use super::{budget::Budget, slot::Slot};

// TODO 2023-07-14: Apply new concept to all the system becuase still parts
//of the system using GoalMap.0 as goal_id
/// Goal's Map of id to Goal. That Id is not goal_id.
pub type GoalsMap = BTreeMap<String, Goal>;

/// Static flag to ensure goals initialization happens only once to the static variable GOALS
static GOALS_INITIALIZED: Once = Once::new();

/// An aim or desired result someone wants to reach.  
#[derive(Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Goal {
    /// The id passed by the frontend, usually a uuid.
    pub id: String,
    /// The title given to the Goal, ie "Run", "Read a book" or "Become a nuclear scientist".
    pub title: String,
    /// The minimum duration per Step towards the Goal.
    #[serde(default)]
    pub min_duration: Option<usize>,
    /// The maximum duration, if the other Goals allow for it.
    #[serde(default)]
    pub max_duration: Option<usize>,
    /// Budgets that apply to this Goal, and all of its subGoals - if any.
    #[serde(default)]
    pub budgets: Option<Vec<Budget>>,
    /// Repetition like 'daily' or 'weekly'.
    pub repeat: Option<Repetition>,
    /// Schedule on calender after this datetime only.
    #[serde(default)]
    pub start: Option<NaiveDateTime>,
    /// Goal has to be achieved by this datetime.
    #[serde(default)]
    pub deadline: Option<NaiveDateTime>,
    /// Internal - should be private
    #[serde(default)]
    pub tags: Vec<Tag>,
    /// Filters that reduce the potential Timeline of the Steps for this Goal.
    /// Examples: After 8, Weekends, not this afternoon
    #[serde(default)]
    pub filters: Option<TimeFilter>,
    /// Ids of the subGoals this Goal has - if any.
    /// Example: Goal 'Work' has subGoal 'ProjectA', which has subGoals 'Prepare for meeting', 'Meeting', etc...
    #[serde(default)]
    pub children: Option<Vec<String>>,
    /// If there is a specific order, this Goal can only be scheduled after certain other Goals complete.
    #[serde(default)]
    pub after_goals: Option<Vec<String>>,
}

/// Mon Tue Wed Thu Fri Sat Sun
#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub enum Day {
    Fri,
    Sat,
    Sun,
    Mon,
    Tue,
    Wed,
    Thu,
}

/// Filters used to reduce the Timeline on which a Goal can be scheduled.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct TimeFilter {
    /// Whatever day this Goal gets scheduled on - only schedule it after this time.
    pub after_time: Option<usize>,
    /// Whatever day this Goal gets scheduled on - only schedule it before this time.
    pub before_time: Option<usize>,
    /// Only schedule this Goal on these days of the week.
    pub on_days: Option<Vec<Day>>,
    /// For whatever reason - don't schedule the Goal during these time slots.
    pub not_on: Option<Vec<Slot>>,
}

// TODO 2023-05-05 | Struct Tag should not be public and think hard about if we can remove them as it complicates the logic
// As agreed in a meeting to refactor this and seperate Tag for Goals and Taasks
// ===
/// Helper tags for the algorithm
#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Eq, Clone, Copy)]
pub enum Tag {
    Donotsplit,
    Weekly,
    Optional,
    FlexDur,
    Remove,
    IgnoreStepGeneration,
    Filler,
    Budget,
}


