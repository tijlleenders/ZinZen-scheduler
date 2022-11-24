use crate::errors::Error;
use crate::goal::Goal;
use crate::slot::Slot;
use chrono::Duration;
use chrono::{NaiveDate, NaiveDateTime, Timelike};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

/// One or many created from a Goal.
/// To be scheduled in order by the scheduler.
#[derive(Deserialize, Debug, Eq, Clone)]
pub struct Task {
    pub id: usize,
    pub goal_id: String,
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
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.flexibility == other.flexibility
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Task {
    //Custom ordering for collections of Tasks:
    //All tasks with flex 1 should be first, followed by task with highest flex to
    //task with lowest flex.
    fn cmp(&self, other: &Self) -> Ordering {
        if self.flexibility == other.flexibility {
            Ordering::Equal
        } else if self.flexibility == 1 {
            Ordering::Less
        } else if other.flexibility == 1 {
            Ordering::Greater
        } else {
            other.flexibility.cmp(&self.flexibility)
        }
    }
}

//An iterator for start/deadline combinations for the task.
//e.g. if a task has duration of 2, and one slot with start 10 and end 14,
//then the following start/deadline combinations are possible:
//10-12, 11-13, and 12-14.
//It also needs to handle scenarios where the task has multiple slots.
pub struct StartDeadlineIterator {
    slots: Vec<Slot>,
    duration: usize,
    marker: NaiveDateTime,
    slot_index: usize,
}

impl StartDeadlineIterator {
    fn new(slots: Vec<Slot>, duration: usize) -> StartDeadlineIterator {
        let marker = slots[0].start;
        let slot_index = 0_usize;
        StartDeadlineIterator {
            slots,
            duration,
            marker,
            slot_index,
        }
    }
}

impl Iterator for StartDeadlineIterator {
    type Item = Slot;
    fn next(&mut self) -> Option<Self::Item> {
        while self.slot_index < self.slots.len() {
            if self.marker > self.slots[self.slot_index].end - Duration::hours(self.duration as i64)
            {
                if self.slot_index >= self.slots.len() - 1 {
                    return None;
                }
                self.slot_index += 1;
                self.marker = self.slots[self.slot_index].start;
                continue;
            }
            let start = self.marker;
            let end = self.marker + Duration::hours(self.duration as i64);
            self.marker += Duration::hours(1);
            return Some(Slot { start, end });
        }
        None
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
        after_time: usize,
        before_time: usize,
    ) -> Self {
        Self {
            id,
            goal_id: goal.id.clone(),
            title: goal.title.clone(),
            duration: goal.duration,
            status: TaskStatus::UNSCHEDULED,
            flexibility,
            start,
            deadline,
            after_time,
            before_time,
            slots,
            confirmed_start: None,
            confirmed_deadline: None,
        }
    }

    fn calculate_flexibility(&mut self) {
        let mut flexibility = 0;
        for slot in &self.slots {
            flexibility += slot.num_hours() - self.duration + 1;
        }
        self.flexibility = flexibility;
    }

    pub fn flexibility(&mut self) -> usize {
        self.flexibility
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
        for _ in 0..self.duration {
            let task = Task {
                id: *counter,
                goal_id: self.goal_id.clone(),
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
            };
            *counter += 1;
            tasks.push(task);
        }
        Ok(tasks)
    }

    pub fn start_deadline_iterator(&mut self) -> StartDeadlineIterator {
        StartDeadlineIterator::new(self.get_slots(), self.duration)
    }

    pub fn schedule(&mut self, slot: Slot) {
        self.set_confirmed_start(slot.start);
        self.set_confirmed_deadline(slot.end);
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
        if (self.duration == 1 && other_task.duration == 1)
            && self.slots.len() == other_task.slots.len()
            && self.slots == other_task.slots
        {
            return true;
        }
        false
    }

    fn remove_invalid_slots(&mut self) {
        self.slots
            .retain(|slot| (slot.end - slot.start).num_hours() >= self.duration as i64);
    }

    pub fn remove_slot(&mut self, s: Slot) {
        let mut new_slots = Vec::new();
        for slot in &mut self.slots {
            new_slots.extend(*slot - s);
        }
        self.slots = new_slots;
        self.remove_invalid_slots();
        self.calculate_flexibility();
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum TaskStatus {
    UNSCHEDULED,
    SCHEDULED,
    IMPOSSIBLE,
    WAITING,
}
