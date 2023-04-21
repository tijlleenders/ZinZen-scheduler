use std::cmp::Ordering;

use crate::{models::{goal::Tag, slot::Slot}, errors::Error};

use super::{Task, TaskStatus};

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
            if slot.calc_duration_in_hours() < self.duration {
                flexibility += slot.calc_duration_in_hours();
                continue;
            }
            //todo check correctness
            flexibility += slot.calc_duration_in_hours() - self.duration + 1;
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
                calendar_start: self.calendar_start,
                calendar_end: self.calendar_end,
            };
            task.calculate_flexibility();
            task.status = TaskStatus::ReadyToSchedule;
            *counter += 1;
            tasks.push(task);
        }
        Ok(tasks)
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
