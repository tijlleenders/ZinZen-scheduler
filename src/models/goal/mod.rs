pub mod impls;

use crate::models::repetition::Repetition;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, option::Option};

use super::{budget::Budget, slot::Slot};

pub type GoalsMap = BTreeMap<String, Goal>;

/// An aim or desired result someone wants to reach.  
#[derive(Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Goal {
    // mandatory fields
    /// The id passed by the frontend, usually a uuid.
    pub id: String,
    /// The title given to the Goal, ie "Run", "Read a book" or "Become a nuclear scientist".
    pub title: String,

    // calculation of the slots
    /// Schedule on calender after this datetime only.
    #[serde(default)]
    pub start: Option<NaiveDateTime>,
    /// Goal has to be achieved until this datetime.
    #[serde(default)]
    pub deadline: Option<NaiveDateTime>,
    /// The minimum duration per Step towards the Goal.
    #[serde(default)]
    pub min_duration: Option<usize>,
    /// The maximum duration, if the other Goals allow for it.
    #[serde(default)]
    pub max_duration: Option<usize>,

    // repeatability
    /// Repetition like 'daily' or 'weekly'.
    pub repeat: Option<Repetition>,

    // constraints
    /// Filters that reduce the potential Timeline of the Steps for this Goal.
    /// Examples: After 8, Weekends, not this afternoon
    #[serde(default)]
    pub filters: Option<TimeFilter>,
    /// Budgets that apply to this Goal, and all of its subGoals - if any.
    #[serde(deserialize_with = "Goal::deserialize_budget_vec", default)]
    pub budgets: Option<Vec<Budget>>,

    // additional stuff; yet not necessary for the algorithm
    /// Ids of the subGoals this Goal has - if any.
    /// Example: Goal 'Work' has subGoal 'ProjectA', which has subGoals 'Prepare for meeting', 'Meeting', etc...
    #[serde(default)]
    pub children: Option<Vec<String>>,
    /// If there is a specific order, this Goal can only be scheduled after certain other Goals complete.
    #[serde(default)]
    pub after_goals: Option<Vec<String>>,

    // ???
    /// Internal - should be private
    #[serde(default)]
    pub tags: Vec<Tag>,
}

/// Mon Tue Wed Thu Fri Sat Sun
#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
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
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub enum Tag {
    DoNotSplit,
    Weekly,
    Optional,
    FlexDur,
    Remove,
    IgnoreStepGeneration,
    Filler,
    Budget,
}
