pub mod impls;

use crate::models::repetition::Repetition;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, option::Option};

use super::{budget::Budget, slot::Slot};

pub type GoalsMap = BTreeMap<String, Goal>;

/// Represents a Goal passed in by the user from the front end.
/// Goals are converted into [Task](../task/index.html)s by the scheduler.
#[derive(Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Goal {
    pub id: String,
    pub title: String,
    /// How much total time should a user put into their goal, eg "I want to learn how to code, and I want to code 6 hours per day"
    #[serde(default)]
    pub min_duration: Option<usize>,
    #[serde(default)]
    pub max_duration: Option<usize>,
    #[serde(default)]
    pub budgets: Option<Vec<Budget>>,
    pub repeat: Option<Repetition>,
    /// start date bound for this Goal's Tasks
    #[serde(default)]
    pub start: Option<NaiveDateTime>,
    /// deadline date bound for this Goal's Tasks
    #[serde(default)]
    pub deadline: Option<NaiveDateTime>,
    #[serde(default)]
    pub tags: Vec<Tag>,
    #[serde(default)]
    pub filters: Option<TimeFilter>,
    #[serde(default)]
    pub children: Option<Vec<String>>,
    #[serde(default)]
    pub after_goals: Option<Vec<String>>,
}

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

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct TimeFilter {
    pub after_time: Option<usize>,
    pub before_time: Option<usize>,
    pub on_days: Option<Vec<Day>>,
    /// Used for postpone slots
    pub not_on: Option<Vec<Slot>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub enum Tag {
    Donotsplit,
    Weekly,
    Optional,
    FlexDur,
    Remove,
    IgnoreForTaskGeneration,
    Filler,
    Budget,
}
