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

    /// Remove conflicted task slots with a given slot [slot_to_remove]
    /// - Function will do nothing with Scheduled tasks
    pub fn remove_conflicted_slots(&mut self, slot_to_remove: Slot) {
        /*
        TODO 2023-06-10: Add test case to guerntee not adding extra hours for the Task.slot
        Todo: duplicate of remove_taken_slots? (NOTE: This todo need to be reviewed)
        */
        if self.status == TaskStatus::Scheduled {
            return;
        }

        dbg!(&self.slots, &slot_to_remove);

        let mut slots_after_filter = Vec::new();
        for slot in &mut self.slots {
            let task_slot = *slot;
            dbg!(&slot_to_remove, &task_slot);

            let subtracted_slot = task_slot - slot_to_remove;
            dbg!(&subtracted_slot);

            slots_after_filter.extend(subtracted_slot);
            dbg!(&slots_after_filter);
        }
        dbg!(&self.slots);
        self.slots = slots_after_filter;
        dbg!(&self.slots);
        // =====

        /*
        Todo 2023-06-08:
        - create a test case and avoid using remove_taken_slots or btter approach.
        Todo 2023-06-09:
        - removed calling Task::remove_taken_slots in case TaskStatus is Blocked
        becasue it is not functional properly and need to be fixed.
        */

        self.calculate_flexibility();
    }

    pub fn remove_taken_slots(&mut self, slot_to_remove: Slot) {
        // TODO 2023-06-09  | This function is not accurate which need to be fixed and create test cases.
        let mut slots_after_filter = Vec::new();
        for task_slot in &mut self.slots {
            dbg!(&task_slot, &slot_to_remove);
            if task_slot.start >= slot_to_remove.end {
                slots_after_filter.push(*task_slot);
            }
            if task_slot.end > slot_to_remove.end && task_slot.start < slot_to_remove.start {
                task_slot.start = slot_to_remove.start;
                slots_after_filter.push(*task_slot);
            }
            if task_slot.end > slot_to_remove.end && task_slot.start >= slot_to_remove.start {
                slots_after_filter.push(*task_slot);
            }
            if !(task_slot.end <= slot_to_remove.end && task_slot.start <= slot_to_remove.start) {
                slots_after_filter.push(*task_slot);
            }

            dbg!(&slots_after_filter);
        }
        dbg!(&slots_after_filter);
        self.slots = slots_after_filter;
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

#[cfg(test)]
mod tests {

    mod remove_conflicted_slots {
        use crate::models::{
            slot::Slot,
            task::{Task, TaskStatus},
        };
        use chrono::Duration;

        /// Testing edge case in bug_215 which slot_to_remove
        /// is bigger than task_slot and task_slot is contained in slot_to_remove
        ///
        /// ```
        /// # "chosen_slot" to be removed from all tasks:
        /// slot_to_remove: 2023-01-03 00 to 08 (8 hours)
        /// # "task_slot" which has less duration than chosen_slot but not removed
        /// task_slot: 2023-01-03 01 to 03 (2 hour)
        /// # Also slot_to_remove contains task_slot
        ///
        /// # Output should be:
        /// task.slots.len(): 0
        /// ```
        #[test]
        fn test_intersected_bigger_slot() {
            let slot_to_remove = Slot::mock(Duration::hours(8), 2023, 01, 03, 0, 0);
            let mut task = Task::mock(
                "test",
                1,
                12,
                TaskStatus::ReadyToSchedule,
                vec![Slot::mock(Duration::hours(2), 2023, 01, 03, 1, 0)],
            );

            task.remove_conflicted_slots(slot_to_remove);

            assert_eq!(task.slots.len(), 0);
        }

        #[test]
        fn test_task_is_scheduled() {
            let slot_to_remove = Slot::mock(Duration::hours(8), 2023, 01, 03, 0, 0);
            let mut task = Task::mock_scheduled(
                1,
                "1",
                "test",
                1,
                12,
                Slot::mock(Duration::hours(2), 2023, 01, 03, 1, 0),
            );

            task.remove_conflicted_slots(slot_to_remove);
            let task_after_remove = task.clone();

            assert_eq!(task_after_remove, task);
        }

        /// Test non intersected task's slots with a given slot (slot_to_remove)
        /// which returns the same task_slot
        #[test]
        fn test_nonintersected_slot() {
            let slot_to_remove = Slot::mock(Duration::hours(8), 2023, 01, 03, 0, 0);
            let task_slot = Slot::mock(Duration::hours(10), 2023, 01, 04, 1, 0);
            let mut task = Task::mock(
                "test",
                1,
                12,
                TaskStatus::ReadyToSchedule,
                vec![task_slot.clone()],
            );

            task.remove_conflicted_slots(slot_to_remove);

            assert_eq!(task.slots[0], task_slot);
        }

        /// Testing normal case which removing conflicted task's slots with
        /// slot_to_remove
        /// ```
        /// slot_to_remove: 2023-01-03 00 to 03 (3 hours)
        ///
        /// task_slot: 2023-01-03 01 to 11 (10 hour)
        ///
        /// Expected:
        /// task_slot: 2023-01-03 03 to 11 (8 hour)
        /// ```
        #[test]
        fn test_normal() {
            let slot_to_remove = Slot::mock(Duration::hours(3), 2023, 01, 03, 0, 0);
            let task_slot = Slot::mock(Duration::hours(10), 2023, 01, 03, 1, 0);
            let mut task = Task::mock(
                "test",
                1,
                12,
                TaskStatus::ReadyToSchedule,
                vec![task_slot.clone()],
            );

            task.remove_conflicted_slots(slot_to_remove);

            let expected_task_slot = Slot::mock(Duration::hours(8), 2023, 01, 03, 3, 0);

            assert_eq!(task.slots[0], expected_task_slot);
        }
    }
}
