use crate::models::{
    repetition::Repetition, slot_iterator::TimeSlotsIterator, task::Task, task::TaskStatus,
};
use chrono::NaiveDateTime;
use log::info;
use serde::{Deserialize, Serialize};
use std::option::Option;

use super::slot::Slot;

/// An aim or desired result someone wants to reach.  
#[derive(Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Goal {
    /// The id passed by the frontend, usually a uuid.
    pub id: String,
    /// The title given to the Goal, ie "Run", "Read a book" or "Become a nuclear scientist".
    pub title: String,
    /// The minimum duration per increment towards the Goal.
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
    /// Filters that reduce where on the Timeline Increments for this Goal can be scheduled.
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
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum Day {
    Fri,
    Sat,
    Sun,
    Mon,
    Tue,
    Wed,
    Thu,
}

impl From<String> for Day {
    fn from(day: String) -> Self {
        info!("From<String> day-string: {:?}", day);

        match day.to_lowercase().as_str() {
            "fri" => Day::Fri,
            "sat" => Day::Sat,
            "sun" => Day::Sun,
            "mon" => Day::Mon,
            "tue" => Day::Tue,
            "wed" => Day::Wed,
            "thu" => Day::Thu,
            _ => panic!("Invalid day selection"),
        }
    }
}

impl From<Day> for String {
    fn from(day: Day) -> Self {
        info!("From<Days> day: {:?}", day);
        match day {
            Day::Fri => "Fri".into(),
            Day::Sat => "Sat".into(),
            Day::Sun => "Sun".into(),
            Day::Mon => "Mon".into(),
            Day::Tue => "Tue".into(),
            Day::Wed => "Wed".into(),
            Day::Thu => "Thu".into(),
        }
    }
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

// imple Disply for TimeFilter
impl std::fmt::Display for TimeFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TimeFilter [ after_time: {:?}, before_time: {:?}, on_days: {:?}, not_on: {:?} ]",
            self.after_time, self.before_time, self.on_days, self.not_on
        )
    }
}

/// Keeps track of the min and max time allowed and scheduled per time period for a collection of Increments/Tasks.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Budget {
    pub budget_type: BudgetType,
    pub min: Option<usize>,
    pub max: Option<usize>,
}

/// weekly or daily
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum BudgetType {
    Weekly,
    Daily,
}

//#[cfg(test)]
impl Goal {
    pub fn new(id: usize) -> Self {
        Self {
            id: id.to_string(),
            title: String::from("Test"),
            ..Default::default()
        }
    }

    // Todo: Check all these setters - Why are they needed? Why public?

    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn duration(mut self, min_duration: usize) -> Self {
        self.min_duration = Some(min_duration);
        self
    }

    pub fn repeat(mut self, repetition: Repetition) -> Self {
        self.repeat = Some(repetition);
        self
    }

    pub fn start(mut self, start: NaiveDateTime) -> Self {
        self.start = Some(start);
        self
    }

    pub fn deadline(mut self, deadline: NaiveDateTime) -> Self {
        self.deadline = Some(deadline);
        self
    }

    pub fn generate_tasks(
        self,
        calendar_start: NaiveDateTime,
        calendar_end: NaiveDateTime,
        counter: &mut usize,
    ) -> Vec<Task> {
        /*There are four type of Tasks:
        **1. Regular tasks: If the repetition is NONE, only one task will be generated for the period between
        ** the start and deadline. These are scheduled first by making them first in the sort order of Task::Ord.

        **2. Habits are Tasks made from a goal that has a repeat (hourly/(flex)daily,(flex)weekly, every mon/.../week/weekend/mon-...).
        ** If the repetition of the goal is DAILY, a different task will be generated for each day between
        ** the start and deadline.
        **If the repetition is MONDAYS, a different task will be generated for each monday
        **between the start and deadline.
        **If the repetition is Weekly, a different task will be generated for each mon-sun
        **period between the start and deadline. etc...(to see all handled scenarios see time_slot_iterator.rs.)

        ** 3. Budget tasks will get a task per time period - per day or per week - with the minimum duration
        ** The minimum duration of budget tasks will be adjusted by the TaskBudgets object after Regular and Filler goals are scheduled.

        ** 4. Optional tasks are tasks that don't HAVE to be scheduled - but nice to do so.
        ** They can come from two places : flex Habits and flex Budgets
        ** Optional tasks represent the duration between the minimum and maximum for a flex Habit or a flex Budget

        ** Before placing a task, the task_placer has to check with the TaskBudgets object to see:
        ** - if they are allowed to be scheduled (no max budgets exceeded)
        ** - and so that any budgets are adjusted

        */

        let mut tasks: Vec<Task> = Vec::new();
        if self.tags.contains(&Tag::IgnoreForTaskGeneration) {
            return tasks;
        }

        if self.tags.contains(&Tag::Budget) {
            return tasks;
        }
        let start = self.start.unwrap_or(calendar_start);
        let deadline = self.deadline.unwrap_or(calendar_end);

        let time_slots_iterator = TimeSlotsIterator::new(
            start,
            deadline,
            self.repeat,
            self.filters,
            // Todo! add self.before_time filter
        );

        for time_slots in time_slots_iterator {
            let task_id = *counter;
            *counter += 1;
            if !time_slots.is_empty() && self.min_duration.is_some() {
                let task = Task {
                    id: task_id,
                    goal_id: self.id.clone(),
                    title: self.title.clone(),
                    duration: self.min_duration.unwrap(),
                    start: None,
                    deadline: None,
                    calender_start: calendar_start,
                    calender_end: calendar_end,
                    slots: time_slots,
                    status: TaskStatus::ReadyToSchedule,
                    tags: self.tags.clone(),
                    after_goals: self.after_goals.clone(),
                    flexibility: 0,
                };
                tasks.push(task);
            }
        }
        tasks
    }
}

/// Helper tags for the algorithm - should not be public
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
