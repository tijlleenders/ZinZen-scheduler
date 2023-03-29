use crate::task::Task;
use crate::time_slot_iterator::TimeSlotsIterator;
use crate::{repetition::Repetition, task::TaskStatus};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::option::Option;

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

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct TimeFilter {
    pub after_time: Option<usize>,
    pub before_time: Option<usize>,
    pub on_days: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Budget {
    pub budget_type: BudgetType,
    pub min: Option<usize>,
    pub max: Option<usize>,
}

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
        println!(" = Goal.generate_tasks.time_slots_iterator.new(): {:#?}", time_slots_iterator);
        for time_slots in time_slots_iterator {
            let task_id = *counter;
            *counter += 1;
            println!(" = Goal.generate_tasks.task_id: {:#?}", task_id);
            println!("Goal.generate_tasks.time_slots.len(): {:?}", time_slots.len());
            if time_slots.len() > 0 && self.min_duration.is_some() {
                println!("Goal.generate_tasks.time_slots: {:?}", time_slots);
                let t = Task::new(
                    task_id,
                    self.id.clone(),
                    self.title.clone(),
                    self.min_duration.unwrap(),
                    None,
                    None,
                    calendar_start,
                    calendar_end,
                    time_slots,
                    TaskStatus::ReadyToSchedule,
                    self.tags.clone(),
                    self.after_goals.clone(),
                );
                tasks.push(t);
            } 
            // else {
            //     // TODO FIX It seems panic is not accurate, it shouldn't panic but end iteration
            //     panic!("time_slots expected")
            // }
        }
        print!("Goal.generate_tasks.tasks before end fn generate_tasks: {:?}", tasks);
        tasks

    }

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
