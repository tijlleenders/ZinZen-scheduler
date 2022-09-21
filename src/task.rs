use crate::errors::Error;
use crate::goal::Goal;
use chrono::{NaiveDateTime, Timelike};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

/// One or many created from a Goal.
/// To be scheduled in order by the scheduler.
#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Task {
    pub id: usize,
    pub goal_id: usize,
    pub title: String,
    pub duration: usize,
    pub status: TaskStatus,
    pub flexibility: usize,
    pub start: NaiveDateTime,
    pub deadline: NaiveDateTime,
    pub after_time: usize,
    pub before_time: usize,
    pub slots: Vec<(NaiveDateTime, NaiveDateTime)>,
    pub confirmed_start: Option<NaiveDateTime>,
    pub confirmed_deadline: Option<NaiveDateTime>,
    pub internal_index: usize,
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        self.flexibility.cmp(&other.flexibility)
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Task {
    pub fn new(id: usize, start: NaiveDateTime, deadline: NaiveDateTime, goal: &Goal) -> Self {
        Self {
            id,
            goal_id: goal.id,
            title: goal.title.clone(),
            duration: goal.duration,
            status: TaskStatus::UNSCHEDULED,
            flexibility: 0,
            start,
            deadline,
            after_time: goal.after_time.unwrap_or(0),
            before_time: goal.before_time.unwrap_or(24),
            slots: Vec::new(),
            confirmed_start: None,
            confirmed_deadline: None,
            internal_index: 0,
        }
    }

    //TODO: The current way this is done may not be entirely accurate for tasks that can be done on
    //multiple days within certain time bounds.
    pub fn calculate_flexibility(&mut self) {
        let hours_available = self.slots.len();
        self.flexibility = hours_available - self.duration + 1;
    }

    pub fn set_confirmed_start(&mut self, start: NaiveDateTime) {
        self.confirmed_start = Some(start);
    }

    pub fn set_confirmed_deadline(&mut self, deadline: NaiveDateTime) {
        self.confirmed_deadline = Some(deadline);
    }

    pub fn get_slots(&self) -> Vec<(NaiveDateTime, NaiveDateTime)> {
        self.slots.clone()
    }

    pub fn remove_slot(&mut self, slot: &(NaiveDateTime, NaiveDateTime)) {
        let mut index = 0;
        for i in 0..self.slots.len() {
            if &self.slots[i] == slot {
                index = i;
            }
        }
        self.slots.remove(index);
    }

    pub fn split(&mut self, counter: &mut usize) -> Result<Vec<Task>, Error> {
        if self.duration == 1 {
            return Err(Error::CannotSplit);
        }
        let mut tasks = Vec::new();
        for _ in 0..self.duration {
            let task = Task {
                id: *counter,
                goal_id: self.goal_id,
                title: self.title.clone(),
                duration: 1,
                status: TaskStatus::UNSCHEDULED,
                flexibility: self.slots.len(),
                start: self.start,
                deadline: self.deadline,
                after_time: self.after_time,
                before_time: self.before_time,
                slots: self.get_slots(),
                confirmed_start: None,
                confirmed_deadline: None,
                internal_index: 0,
            };
            *counter += 1;
            tasks.push(task);
        }
        Ok(tasks)
    }

    pub fn next_start_deadline_combination(&mut self) -> Option<(NaiveDateTime, NaiveDateTime)> {
        if self.internal_index + self.duration - 1 >= self.slots.len() {
            return None;
        }
        let index = self.internal_index;
        self.internal_index += 1;
        return Some((self.slots[index].0, self.slots[index + self.duration - 1].1));
    }

    pub fn schedule(&mut self, start: NaiveDateTime, deadline: NaiveDateTime) {
        self.set_confirmed_start(start);
        self.set_confirmed_deadline(deadline);
        self.status = TaskStatus::SCHEDULED;
    }

    pub fn num_slots_to_be_assigned(&self) -> usize {
        if self.before_time > self.after_time {
            self.before_time - self.after_time
        } else {
            self.before_time + (24 - self.after_time)
        }
    }

    pub fn can_coexist_with(&self, other_task: &Task) -> bool {
        (self.duration == 1 && other_task.duration == 1)
            && (self.after_time == other_task.after_time)
            && (self.before_time == other_task.before_time)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum TaskStatus {
    UNSCHEDULED,
    SCHEDULED,
    IMPOSSIBLE,
    WAITING,
}
