use crate::models::task::{Task, TaskStatus};

impl Task {
    /// Calculate flexibility of a task slots
    pub fn calculate_flexibility(&mut self) {
        dbg!(&self);
        // Todo 2023-05-29  | add rules for other TaskStatus options
        if self.status == TaskStatus::Scheduled || self.status == TaskStatus::Impossible {
            return;
        }

        let mut flexibility = 0;
        let task_duration = self.duration;

        for slot in &self.slots {
            let slot_duration = slot.duration_as_hours();
            dbg!(&slot);
            if slot_duration < task_duration {
                flexibility += slot_duration;
                continue;
            }
            //todo check correctness
            flexibility += slot_duration - task_duration + 1;
        }
        self.flexibility = flexibility;

        if self.flexibility == 0 {
            self.status = TaskStatus::Impossible
        }
        dbg!(&self);
    }
}

#[cfg(test)]
mod tests {
    use chrono::Duration;

    use crate::models::slot::Slot;

    use super::*;

    /// Test when Task Status is Impossible
    #[test]
    fn test_status_impossible() {
        let mut task = Task::mock(
            1,
            168,
            TaskStatus::Impossible,
            vec![Slot::mock(Duration::days(6), 2023, 05, 01, 0, 0)],
        );
        let flex_before = task.flexibility;
        task.calculate_flexibility();
        assert_eq!(task.status, TaskStatus::Impossible);
        assert_eq!(task.flexibility, flex_before);
    }

    /// Test when Task Status is Scheduled
    #[test]
    fn test_status_scheduled() {
        let mut task = Task::mock(
            1,
            186,
            TaskStatus::Scheduled,
            vec![Slot::mock(Duration::days(6), 2023, 05, 01, 0, 0)],
        );
        let flex_before = task.flexibility;
        task.calculate_flexibility();
        assert_eq!(task.status, TaskStatus::Scheduled);
        assert_eq!(task.flexibility, flex_before);
    }

    /// Test when Task Status is Uninitialized
    #[test]
    fn test_status_uninitialized() {
        // TODO 2023-05-29  |  add rules for TaskStatus Uninitialized
        todo!("test_status_Uninitialized");
    }

    /// Test when Task Status is Blocked
    #[test]
    fn test_status_blocked() {
        // TODO 2023-05-29  |  add rules for TaskStatus Blocked
        todo!("test_status_blocked");
    }

    /// Test when Task Status is BudgetMinWaitingForAdjustment
    #[test]
    fn test_status_budget_min_wait_adj() {
        // TODO 2023-05-29  |  add rules for TaskStatus BudgetMinWaitingForAdjustment
        todo!("test_status_budget_min_wait_adj");
    }

    ///
    #[test]
    fn test_calculate_flexibility() {
        // todo!("test_calculate_flexibility");
        // // Create a task with two slots: one 2-hour slot and one 4-hour slot
        // let mut task = Task::mock(status, slots)
        // task.add_slot(Slot::new(
        //     LocalTime::from_hms(9, 0, 0),
        //     LocalTime::from_hms(11, 0, 0),
        // ));
        // task.add_slot(Slot::new(
        //     LocalTime::from_hms(13, 0, 0),
        //     LocalTime::from_hms(17, 0, 0),
        // ));

        // // Calculate flexibility
        // task.calculate_flexibility();

        // // Check that flexibility is correct
        // assert_eq!(task.flexibility, 3); // 2 hours from first slot + 1 hour from second slot

        // // Check that status is still New
        // assert_eq!(task.status, TaskStatus::New);

        // // Schedule the task by taking one of the slots
        // task.schedule(LocalTime::from_hms(9, 0, 0));

        // // Calculate flexibility again (should not do anything since task is scheduled)
        // task.calculate_flexibility();

        // // Check that flexibility is still zero
        // assert_eq!(task.flexibility, 0);

        // // Check that status is now Scheduled
        // assert_eq!(task.status, TaskStatus::Scheduled);
    }
}
