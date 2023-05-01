use crate::errors::Error;
use crate::models::goal::Tag;
use crate::models::slot::Slot;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

/// Tasks/Increments are generated to achieve a Goal in one or more Increments.
/// A Goal can generate one or more Tasks.
#[derive(Deserialize, Debug, Eq, Clone)]
pub struct Task {
    /// Only used by the scheduler.
    /// Unstable between scheduler runs if input changes.
    pub id: usize,
    /// Reference to the Goal a Taks/Increment was generated from.
    pub goal_id: String,
    /// Title of the Goal the Task/Increment was generated from.
    /// Duplicated for ease of debugging and simplicity of code.
    pub title: String,
    /// Duration the Task/Increment wants to claim on the Calendar.
    /// This duration is equal or part of the Goal duration.
    pub duration: usize,
    /// Used for finding next Task/Increment to be scheduled in combination with Task/Increment flexibility, after_goals and Tags.
    pub status: TaskStatus,
    /// Used for finding next Task/Increment to be scheduled in combination with Task/Increment Status, after_goals and Tags.
    pub flexibility: usize,
    /// Final start time for Task/Increment on Calendar - should be removed in favor of Timeline + SlotStatus combination.
    pub start: Option<NaiveDateTime>,
    /// Final end time for Task/Increment on Calendar - should be removed in favor of Timeline + SlotStatus combination.
    pub deadline: Option<NaiveDateTime>,
    /// The places on Calendar that could potentially be used given the Goal constraints - and what other scheduled Tasks/Increments already have consumed.
    pub slots: Vec<Slot>,
    /// Used for finding next Task/Increment to be scheduled in combination with Task/Increment flexibility, after_goals, and Status.
    #[serde(default)]
    pub tags: Vec<Tag>,
    /// Used for finding next Task/Increment to be scheduled in combination with Task/Increment tags, flexibility and Status.
    #[serde(default)]
    pub after_goals: Option<Vec<String>>,
    /// Duplicated info from Input - can be removed as Goal has already been adjusted to Calendar bounds?
    pub calender_start: NaiveDateTime,
    /// Duplicated info from Input - can be removed as Goal has already been adjusted to Calendar bounds?
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
    /// ### Custom ordering for collections of Tasks:
    ///
    /// TODO!: Rething Tags/Statusses to simplify and make this easier to understand
    ///
    /// **Careful!:** Recalculate flexibilities and re-sort after every Task/Increment placement
    /// This is required because finalizing the place(s) on the Calendar of Task/Increment makes
    /// those Slots unavailable for other Task/Increments, thus changing their flexibility. Also,
    /// some Tasks are waiting for others to be placed, and at some point they are ready to go too.
    ///
    /// 0. Exclude the following Tasks/Increments from being picked:
    /// - Scheduled
    /// - Impossible
    /// - Uninitialized (should not be there - panic if you find it!)
    /// - Blocked
    /// - BudgetMinWaitingForAdjustment
    /// - ReadyToSchedule with Remove Tag
    ///
    /// 1. Sort on Task/Increment Status first using following order:
    /// - ReadyToSchedule without Optional Tag,  without Filler Tag
    /// - ReadyToSchedule without Optional Tag, with Filler Tag
    /// - BudgetMinWaitingForAdjustment - should always be without Optional Tag
    /// - ReadyToSchedule with Optional Tag - with or without FlexDur/FlexNumber Tag
    /// - BudgetMaxWaitingForAdjustment
    ///
    ///
    /// 2. Then apply custom sort on flexibility within the Tasks/Increments with highest Status:
    /// - If there is a Tasks/Increments with flexibility 1, pick that one
    /// - If there are no more Tasks/Increments with flexibility 1 - pick the Task/Increment with **highest** flexibility
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

/// Used to decide in which order to schedule tasks, together with their flexibility
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum TaskStatus {
    /// Task is scheduled and can't be modified any more.
    Scheduled,
    /// Task is impossible - its MaybeSlots Timeline is removed.
    Impossible,
    /// Task is waiting for something to be properly initialized.
    Uninitialized,
    /// Task is waiting for another Goal to be scheduled first.
    Blocked,
    /// Task is available for scheduling, but its relative flexibility and Tags will determine if it gets picked first
    ReadyToSchedule,
    /// Special Task that will try to fill in any missing hours to reach the minimum budget for a time period.
    BudgetMinWaitingForAdjustment,
}
