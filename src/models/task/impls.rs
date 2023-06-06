use super::{NewTask, Task, TaskStatus};
use crate::{
    errors::Error,
    models::{
        goal::{Goal, Tag},
        slot::Slot,
        timeline::Timeline,
    },
};
use std::cmp::Ordering;

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
        // TODO 2023-06-01  | Refactor for readability
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
    /// Create new task
    pub fn new(new_task: NewTask) -> Task {
        let start = new_task.timeframe.map(|time| time.start);
        let deadline = new_task.timeframe.map(|time| time.end);

        Task {
            id: new_task.task_id,
            goal_id: new_task.goal.id,
            title: new_task.title,
            duration: new_task.duration,
            status: new_task.status,
            flexibility: 0,
            start,
            deadline,
            slots: new_task.timeline.slots.into_iter().collect(),
            tags: new_task.goal.tags,
            after_goals: new_task.goal.after_goals,
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
        let timeline = Timeline {
            slots: self.get_slots().into_iter().collect(),
        };
        let goal = Goal {
            id: self.goal_id.clone(),
            title: self.title.clone(),
            tags: self.tags.clone(),
            after_goals: self.after_goals.clone(),
            ..Default::default()
        };
        let new_task = NewTask {
            task_id: *counter,
            title: self.title.clone(),
            duration: 1,
            goal,
            timeline,
            status: TaskStatus::Uninitialized,
            timeframe: None,
        };

        for _ in 0..self.duration {
            let mut task = Task::new(new_task.clone());

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
        // TODO | 2023-06-06 | Seeking more info about this function
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
