use crate::errors::Error;
use crate::goal::{Goal, Tag};
use crate::slot::Slot;
use chrono::Duration;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

/// Tasks are generated from Goals, and represent a concrete activity of a
/// specified duration, that is to be carried out at a specific time.
/// A task can be in a number of different Task Statuses.  
/// A Goal can generate one or more Tasks.
/// # Examples (greatly simplified to highlight the concept)
/// | User Enters | Goal | Task(s) |
/// | ----------- | ---- | ------- |
/// | 'Dentist 1hr after 10 before 11' | Goal {..., after_time: 10, before_time: 11} | Task {..., id: 1, confirmed_start: 2023-01-23T10:00:00 , confirmed_deadline: 2023-01-23T11:00:00} |
/// | 'Read 1hr daily' | Goal {..., repeat: daily} | Task {..., id: 1, confirmed_start: 2023-01-23T17:00:00, confirmed_deadline: 2023-01-23T18:00:00 }, Task {..., id: 2, confirmed_start: 2023-01-24T17:00:00, confirmed_deadline: 2024-01-24T18:00:00 }  
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
    pub conflicts: Vec<(Slot, String)>,
    #[serde(default)]
    pub tags: Vec<Tag>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<ScheduleOption>>,
    #[serde(default)]
    pub after_goals: Option<Vec<String>>,
    pub calender_start: NaiveDateTime,
    pub calender_end: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct ScheduleOption {
    pub start: NaiveDateTime,
    pub deadline: NaiveDateTime,
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
        if other.tags.contains(&Tag::Optional) && !self.tags.contains(&Tag::Optional) {
            Ordering::Less
        } else if !other.tags.contains(&Tag::Optional) && self.tags.contains(&Tag::Optional) {
            Ordering::Greater
        } else if self.flexibility == other.flexibility {
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
    fn new(slots: Vec<Slot>, duration: usize) -> Option<StartDeadlineIterator> {
        if slots.is_empty() {
            return None;
        }
        let marker = slots[0].start;
        let slot_index = 0_usize;
        Some(StartDeadlineIterator {
            slots,
            duration,
            marker,
            slot_index,
        })
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
        goal: &Goal,
        calender_start: NaiveDateTime,
        calender_end: NaiveDateTime,
    ) -> Self {
        Self {
            id,
            goal_id: goal.id.clone(),
            title: goal.title.clone(),
            duration: goal.duration.0,
            status: TaskStatus::Uninitialized,
            flexibility: 0,
            start,
            deadline,
            after_time: goal.after_time.unwrap_or(0),
            before_time: goal.before_time.unwrap_or(24),
            slots: Vec::new(),
            confirmed_start: None,
            confirmed_deadline: None,
            conflicts: Vec::new(),
            tags: goal.tags.clone(),
            options: None,
            after_goals: goal.after_goals.clone(),
            calender_start,
            calender_end,
        }
    }

    fn calculate_flexibility(&mut self) {
        let mut flexibility = 0;
        for slot in &self.slots {
            if slot.num_hours() < self.duration {
                flexibility += slot.num_hours();
                continue;
            }
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
            // && !self.tags.contains(&Tag::DoNotSort) {
            return Err(Error::CannotSplit);
        }
        let mut tasks = Vec::new();

        for _ in 0..self.duration {
            let mut task = Task {
                id: *counter,
                goal_id: self.goal_id.clone(),
                title: self.title.clone(),
                duration: 1,
                status: TaskStatus::Uninitialized,
                flexibility: 0,
                start: self.start,
                deadline: self.deadline,
                after_time: self.after_time,
                before_time: self.before_time,
                slots: self.get_slots(),
                confirmed_start: None,
                confirmed_deadline: None,
                conflicts: Vec::new(),
                tags: self.tags.clone(),
                options: None,
                after_goals: self.after_goals.clone(),
                calender_start: self.calender_start,
                calender_end: self.calender_end,
            };
            task.calculate_flexibility();
            task.status = TaskStatus::ReadyToSchedule;
            *counter += 1;
            tasks.push(task);
        }
        Ok(tasks)
    }

    pub fn start_deadline_iterator(&mut self) -> Option<StartDeadlineIterator> {
        StartDeadlineIterator::new(self.get_slots(), self.duration)
    }

    pub fn schedule(&mut self, slot: Slot) {
        self.set_confirmed_start(slot.start);
        self.set_confirmed_deadline(slot.end);
        self.status = TaskStatus::Scheduled;
    }

    pub fn size_of_slots_to_be_assigned(&self) -> usize {
        if self.before_time > self.after_time {
            self.before_time - self.after_time
        } else {
            self.before_time + (24 - self.after_time)
        }
    }

    //Tasks of duration 1 with equal slots and flex > 1 should be allowed to eat into each other's
    //slots. This happens for example after splitting tasks to 1hr tasks.
    //Without this condition, these tasks would never get scheduled.
    //e.g. walk 1hr with slot (10-12) and dentist 1hr with slot (10-12).
    //The scheduler should allow walk to be scheduled at 10-11 and dentist at 11-12.
    pub fn can_coexist_with(&self, other_task: &Task) -> bool {
        (self.duration == 1 && other_task.duration == 1)
            && self.flexibility > 1
            && other_task.flexibility > 1
            && self.slots == other_task.slots
    }

    pub fn remove_slot(&mut self, s: Slot) {
        let mut new_slots = Vec::new();
        for slot in &mut self.slots {
            new_slots.extend(*slot - s);
        }
        self.slots = new_slots;
        if self.status == TaskStatus::Blocked {
            Self::remove_taken_slots(self, s);
        }
        //if no more slots left, this is an impossible task - mark it as such and return
        if self.slots.is_empty() && self.status != TaskStatus::Scheduled {
            self.status = TaskStatus::Impossible;
            return;
        }
        //the flexibility should be recalculated
        self.calculate_flexibility();
    }
    pub fn remove_taken_slots(&mut self, s: Slot) {
        let mut new_slots = Vec::new();
        for slot in &mut self.slots {
            if slot.start >= s.end {
                new_slots.push(*slot);
            }
            if slot.end > s.end && slot.start < s.start {
                slot.start = s.start;
                new_slots.push(*slot);
            }
            if slot.end > s.end && slot.start >= s.start {
                new_slots.push(*slot);
            }
            if !(slot.end <= s.end && slot.start <= s.start) {
                new_slots.push(*slot);
            }
        }
        self.slots = new_slots;
    }

    pub fn bounds_contain(&self, hour: usize) -> bool {
        //checks if the provided hour is within the time bounds of the task.
        if self.before_time < self.after_time {
            hour < self.before_time || hour >= self.after_time
        } else {
            hour >= self.after_time && hour < self.before_time
        }
    }
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum TaskStatus {
    Scheduled,
    Impossible,
    Uninitialized,
    Blocked,
    ReadyToSchedule,
}
