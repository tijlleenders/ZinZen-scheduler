use crate::slot_generator::slot_generator;
use crate::task::Task;
use crate::time_slot_iterator::TimeSlotIterator;
use crate::{repetition::Repetition, task::TaskStatus};
use chrono::{Duration, NaiveDateTime, Timelike};
use serde::Deserialize;
use std::option::Option;

#[derive(Deserialize, Debug, Default)]
pub struct Goal {
    pub id: String,
    pub title: String,
    /// How much total time should a user put into their goal, eg "I want to learn how to code, and I want to code 6 hours per day"
    pub duration: usize,
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
        self.duration = duration;
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
         **If the repetition is WEEKLY, a different task will be generated for each mon-sun
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
            Some(Repetition::WEEKLY(x)) => x,
            Some(Repetition::DAILY(x)) => x,
            _ => 1,
        };
        for time_period in time_periods {
            for _ in 0..tasks_per_period {
                let task_id = *counter;
                *counter += 1;
                let t = Task::new(task_id, time_period.start, time_period.end, &self);
                //assign slots that are within the specified after_time and before_time
                let mut t = slot_generator(t, &time_period);
                //calculate flexibility
                let mut flexibility = 0;
                for slot in &t.slots {
                    flexibility += slot.num_hours() - self.duration + 1;
                }
                t.flexibility = flexibility;
                t.status = TaskStatus::UNSCHEDULED;
                tasks.push(t);
            }
        }
        tasks
    }
}
