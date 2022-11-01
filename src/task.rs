use crate::errors::Error;
use crate::goal::Goal;
use crate::slot::Slot;
use chrono::Duration;
use chrono::{NaiveDate, NaiveDateTime, Timelike};
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
    pub slots: Vec<Slot>,
    pub confirmed_start: Option<NaiveDateTime>,
    pub confirmed_deadline: Option<NaiveDateTime>,
    pub internal_marker: NaiveDateTime,
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
    pub fn new(
        id: usize,
        start: NaiveDateTime,
        deadline: NaiveDateTime,
        slots: Vec<Slot>,
        flexibility: usize,
        goal: &Goal,
    ) -> Self {
        //set internal_marker to first possible hour for the task
        let mut internal_marker = start;
        internal_marker += Duration::hours(goal.after_time.unwrap_or(0) as i64);
        Self {
            id,
            goal_id: goal.id,
            title: goal.title.clone(),
            duration: goal.duration,
            status: TaskStatus::UNSCHEDULED,
            flexibility,
            start,
            deadline,
            after_time: goal.after_time.unwrap_or(0),
            before_time: goal.before_time.unwrap_or(24),
            slots,
            confirmed_start: None,
            confirmed_deadline: None,
            internal_marker,
        }
    }

    //TODO: The current way this is done may not be entirely accurate for tasks that can be done on
    //multiple days within certain time bounds.
    pub fn calculate_flexibility(&mut self) {
        let mut hours_available = 0;
        for slot in &self.slots {
            hours_available += slot.num_hours();
        }
        self.flexibility = hours_available - self.duration + 1;
    }

    pub fn set_confirmed_start(&mut self, start: NaiveDateTime) {
        self.confirmed_start = Some(start);
    }

    pub fn set_confirmed_deadline(&mut self, deadline: NaiveDateTime) {
        self.confirmed_deadline = Some(deadline);
    }

    pub fn get_slots(&self) -> Vec<Slot> {
        self.slots.clone()
    }

    pub fn split(&mut self, counter: &mut usize) -> Result<Vec<Task>, Error> {
        if self.duration == 1 {
            return Err(Error::CannotSplit);
        }
        let mut tasks = Vec::new();
        for i in 0..self.duration {
            //set internal_marker to first possible hour for the task
            let mut internal_marker = self.start;
            internal_marker += Duration::hours(self.after_time as i64);

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
                internal_marker,
            };
            *counter += 1;
            tasks.push(task);
        }
        Ok(tasks)
    }

    pub fn next_start_deadline_combination(&mut self) -> Option<(NaiveDateTime, NaiveDateTime)> {
        //uses the internal_marker belonging to this task to keep track of the last attempted start
        //time, and increments before the next call.
        //note that if the task has multiple slots (which will be separated by an hr or more),
        //the marker will be moved to the start of the next slot when it reaches the end of a
        //particular slot.
        for (index, slot) in self.slots.iter().enumerate() {
            if !(self.internal_marker >= slot.start && self.internal_marker < slot.end) {
                continue;
            }
            while (self.internal_marker + Duration::hours(self.duration as i64)) <= slot.end {
                let start = self.internal_marker;
                let end = self.internal_marker + Duration::hours(self.duration as i64);
                self.internal_marker += Duration::hours(1);
                if self.internal_marker == slot.end {
                    if index != self.slots.len() - 1 {
                        self.internal_marker = self.slots[index + 1].start;
                    }
                }
                return Some((start, end));
            }

            if index != self.slots.len() - 1 {
                self.internal_marker = self.slots[index + 1].start;
            }
        }
        return None;
    }

    pub fn schedule(&mut self, start: NaiveDateTime, deadline: NaiveDateTime) {
        self.set_confirmed_start(start);
        self.set_confirmed_deadline(deadline);
        self.status = TaskStatus::SCHEDULED;
    }

    pub fn size_of_slots_to_be_assigned(&self) -> usize {
        if self.before_time > self.after_time {
            self.before_time - self.after_time
        } else {
            self.before_time + (24 - self.after_time)
        }
    }

    //Tasks of duration 1 with equal slots should be allowed to eat into each other's
    //slots. This happens for example after splitting tasks to 1hr tasks.
    //Without this condition, these tasks would never get scheduled.
    pub fn can_coexist_with(&self, other_task: &Task) -> bool {
        if !(self.duration == 1 && other_task.duration == 1) {
            return false;
        }
        if self.slots.len() != other_task.slots.len() {
            return false;
        }
        for i in 0..self.slots.len() {
            if self.slots[i] != other_task.slots[i] {
                return false;
            }
        }
        true
    }

    pub fn remove_invalid_slots(&mut self) {
        self.slots = self
            .slots
            .iter()
            .filter(|slot| (slot.end - slot.start).num_hours() >= self.duration as i64)
            .copied()
            .collect();
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum TaskStatus {
    UNSCHEDULED,
    SCHEDULED,
    IMPOSSIBLE,
    WAITING,
}
