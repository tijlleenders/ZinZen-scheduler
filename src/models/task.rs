use crate::errors::Error;
use crate::models::goal::Tag;
use crate::models::slot::Slot;
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
    pub start: Option<NaiveDateTime>,
    pub deadline: Option<NaiveDateTime>,
    pub slots: Vec<Slot>,
    #[serde(default)]
    pub tags: Vec<Tag>,
    #[serde(default)]
    pub after_goals: Option<Vec<String>>,
    pub calender_start: NaiveDateTime,
    pub calender_end: NaiveDateTime,
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
        if (self.status == TaskStatus::ReadyToSchedule)
            && !(other.status == TaskStatus::ReadyToSchedule)
        {
            Ordering::Less
        } else if (other.status == TaskStatus::ReadyToSchedule)
            && !(self.status == TaskStatus::ReadyToSchedule)
        {
            Ordering::Greater
        } else if !self.tags.contains(&Tag::Optional) && other.tags.contains(&Tag::Optional) {
            Ordering::Less
        } else if self.tags.contains(&Tag::Optional) && !other.tags.contains(&Tag::Optional) {
            Ordering::Greater
        } else if !self.tags.contains(&Tag::Filler) && other.tags.contains(&Tag::Filler) {
            Ordering::Less
        } else if self.tags.contains(&Tag::Filler) && !other.tags.contains(&Tag::Filler) {
            Ordering::Greater
        } else if self.flexibility == other.flexibility {
            Ordering::Equal
        } else if self.flexibility == 1 {
            Ordering::Less
        } else if other.flexibility == 1 {
            Ordering::Greater
        } else if self.flexibility > other.flexibility {
            Ordering::Less
        } else if other.flexibility > self.flexibility {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl Task {
    pub fn calculate_flexibility(&mut self) {
        if self.status == TaskStatus::Scheduled {
            return;
        }
        let mut flexibility = 0;
        for slot in &self.slots {
            if slot.num_hours() < self.duration {
                flexibility += slot.num_hours();
                continue;
            }
            //todo check correctness
            flexibility += slot.num_hours() - self.duration + 1;
        }
        self.flexibility = flexibility;
        if self.flexibility == 0 {
            self.status = TaskStatus::Impossible
        }
    }

    pub fn flexibility(&mut self) -> usize {
        self.flexibility
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
                slots: self.get_slots(),
                tags: self.tags.clone(),
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

    pub fn remove_slot(&mut self, s: Slot) {
        //Todo: duplicate of remove_taken_slots?
        if self.status == TaskStatus::Scheduled {
            return;
        }

        let mut new_slots = Vec::new();
        for slot in &mut self.slots {
            new_slots.extend(*slot - s);
        }
        self.slots = new_slots;
        if self.status == TaskStatus::Blocked {
            Self::remove_taken_slots(self, s);
        }

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

    pub fn remove_from_blocked_by(&mut self, _id_string: String) {
        // Todo!
        // if self.after_goals.is_none() {
        //     return;
        // }
        // let mut ids = self.after_goals.clone().unwrap();
        // let index = ids.clone().iter().position(|x| x.eq(&id_string));
        // if index.is_some() {
        //     ids.remove(index.unwrap());
        //     if ids.is_empty() {
        //         self.after_goals = None;
        //         self.status = TaskStatus::ReadyToSchedule;
        //     } else {
        //         self.after_goals = Some(ids);
        //     }
        // }
    }
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum TaskStatus {
    Scheduled,
    Impossible,
    Uninitialized,
    Blocked,
    ReadyToSchedule,
    BudgetMinWaitingForAdjustment,
}
