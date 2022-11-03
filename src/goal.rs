use crate::slot_generator::slot_generator;
use crate::task::Task;
use crate::time_slot_iterator::{Repetition, TimeSlotIterator};
use chrono::NaiveDateTime;
use serde::Deserialize;
use std::option::Option;

#[derive(Deserialize, Debug, Default)]
pub struct Goal {
    pub id: usize,
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
            id,
            title: String::from("Test"),
            ..Default::default()
        }
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
        let time_periods = TimeSlotIterator {
            start,
            end: deadline,
            repetition: self.repeat,
        };
        for time_period in time_periods {
            let task_id = *counter;
            *counter += 1;
            //assign slots that are within the specified after_time and before_time
            let slots = slot_generator(self.after_time, self.before_time, &time_period);
            //calculate flexibility
            let mut hours_available = 0;
            for slot in &slots {
                hours_available += slot.num_hours();
            }
            let flexibility = hours_available - self.duration + 1;
            let t = Task::new(task_id, start, deadline, slots, flexibility, &self);
            tasks.push(t);
        }
        tasks
    }
}
