use crate::slot_generator::slot_generator;
use crate::task::Task;
use crate::time_slot_iterator::TimeSlotIterator;
use crate::{repetition::Repetition, task::TaskStatus};
use chrono::NaiveDateTime;
use serde::de::{self, Visitor};
use serde::*;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::option::Option;

/// Represents a Goal passed in by the user from the front end.
/// Goals are converted into [Task](../task/index.html)s by the scheduler.
#[derive(Deserialize, Debug, Default, Clone)]
pub struct Goal {
    pub id: String,
    pub title: String,
    /// How much total time should a user put into their goal, eg "I want to learn how to code, and I want to code 6 hours per day"
    pub duration: GoalDuration,
    pub repeat: Option<Repetition>,
    /// start date bound for this Goal's Tasks
    #[serde(default)]
    pub start: Option<NaiveDateTime>,
    /// deadline date bound for this Goal's Tasks
    #[serde(default)]
    pub deadline: Option<NaiveDateTime>,
    /// start time bound after which activity should be done
    #[serde(default)]
    pub after_time: Option<usize>,
    /// deadline time bound before which activity should be done
    #[serde(default)]
    pub before_time: Option<usize>,
    #[serde(default)]
    pub tags: Vec<Tag>,
    #[serde(default)]
    pub children: Option<Vec<String>>,
    #[serde(default)]
    pub after_goals: Option<Vec<String>>,
}

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct GoalDuration(pub usize, pub Option<usize>); //in case of flex-duration, the second value represents the upper bound of the duration
struct GoalDurationVisitor;

impl<'de> Visitor<'de> for GoalDurationVisitor {
    type Value = GoalDuration;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "a string of either the duration or a flex duration."
        )
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if s.contains('-') && s.contains('h') {
            //e.g. '35-40h'
            let split = s.split('-').collect::<Vec<&str>>();
            let min = split[0];
            let max = &split[1][0..split[1].len() - 1];
            let min = min.parse::<usize>().expect("expected format to be x-yh");
            let max = max.parse::<usize>().expect("expected format to be x-yh");
            Ok(GoalDuration(min, Some(max)))
        } else {
            let duration = s.parse::<usize>().expect("expected format to be x-yh");
            Ok(GoalDuration(duration, None))
        }
    }
}

impl<'de> Deserialize<'de> for GoalDuration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(GoalDurationVisitor)
    }
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

    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn duration(mut self, duration: usize) -> Self {
        self.duration = GoalDuration(duration, None);
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

    pub fn after_time(mut self, after_time: usize) -> Self {
        self.after_time = Some(after_time);
        self
    }

    pub fn before_time(mut self, before_time: usize) -> Self {
        self.before_time = Some(before_time);
        self
    }

    pub fn generate_tasks(
        self,
        calendar_start: NaiveDateTime,
        calendar_end: NaiveDateTime,
        counter: &mut usize,
    ) -> Vec<Task> {
        let mut tasks = Vec::new();
        /*If the repetition of the goal is DAILY, a different task will be generated for each day between
         **the start and deadline.
         **If the repetition is MONDAYS, a different task will be generated for each monday
         **between the start and deadline.
         **If the repetition is Weekly, a different task will be generated for each mon-sun
         **period between the start and deadline. etc...(to see all handled scenarios see time_slot_iterator.rs.)
         **.
         **.
         **.
         **If the repetition is NONE, only one task will be generated for the period between
         **the start and deadline.*/
        let start = self.start.unwrap_or(calendar_start);
        let deadline = self.deadline.unwrap_or(calendar_end);

        let time_periods = TimeSlotIterator::new(
            start,
            deadline,
            self.repeat,
            self.after_time.unwrap_or(0),
            self.before_time.unwrap_or(24),
        );
        let tasks_per_period = match self.repeat {
            Some(Repetition::Weekly(x)) => x,
            Some(Repetition::DAILY(x)) => x,
            _ => 1,
        };

        for time_period in time_periods {
            for _ in 0..tasks_per_period {
                let task_id = *counter;
                *counter += 1;
                let t = Task::new(task_id, time_period.start, time_period.end, &self);
                //assign slots that are within the specified after_time and before_time
                let mut t = slot_generator(t, &time_period, self.deadline);
                //if only one slot was assigned and it is too short for the duration,
                //mark the task as impossible.
                if t.slots.len() == 1 && t.slots[0].num_hours() < t.duration {
                    t.status = TaskStatus::Impossible;
                    t.conflicts
                        .push((t.slots[0], "Passes Deadline".to_string()));
                } else {
                    //calculate flexibility and mark it as unscheduled.
                    let mut flexibility = 0;
                    for slot in &t.slots {
                        flexibility += slot.num_hours() - self.duration.0 + 1;
                    }
                    t.flexibility = flexibility;
                    t.status = TaskStatus::UNScheduled;
                }
                if let Some(Repetition::Weekly(_)) = self.repeat {
                    if !self.tags.contains(&Tag::FlexDur) {
                        t.tags.push(Tag::Weekly);
                    }
                }
                tasks.push(t);
            }
        }
        tasks
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub enum Tag {
    Donotsplit,
    Weekly,
    Optional,
    FlexDur,
}

pub fn handle_hierarchy(goals: Vec<Goal>) -> Vec<Goal> {
    let parent_goals = goals
        .iter()
        .filter(|goal| goal.children.is_some())
        .cloned()
        .collect::<Vec<Goal>>();
    let mut children_goals = goals
        .iter()
        .filter(|goal| goal.children.is_none())
        .cloned()
        .collect::<Vec<Goal>>();

    for p in parent_goals {
        let mut children_duration = 0;
        let mut child = p.clone();
        let child_ids = p.children.unwrap();

        for goal in children_goals.iter() {
            if child_ids.contains(&goal.id) {
                children_duration += goal.duration.0;
            }
        }

        child.title.push_str(" filler");
        child.duration.0 -= children_duration;
        children_goals.push(child);
    }
    children_goals
}

// pub fn handle_dependency(goals: Vec<Goal>) -> Vec<Goal> {
//     let dependent_goals = goals
//         .iter()
//         .filter(|goal| goal.after_goals.is_some())
//         .cloned()
//         .collect::<Vec<Goal>>();
//         let odrdered_goals= vec![];
//         dependent_goals.iter().filter(|s|s.after_goals)
//     let mut independent_goals = goals
//         .iter()
//         .filter(|goal| goal.children.is_none())
//         .cloned()
//         .collect::<Vec<Goal>>();
// }
