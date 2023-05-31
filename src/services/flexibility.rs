use crate::models::task::{Task, TaskStatus};

impl Task {
    /// Calculate flexibility of a task slots
    pub fn calculate_flexibility(&mut self) {
        if self.status == TaskStatus::Scheduled || self.status == TaskStatus::Impossible {
            return;
        }

        let task_duration = self.duration;
        let flexibility = self.slots.iter().fold(0, |acc, slot| {
            let slot_duration = slot.duration_as_hours();
            // TODO 2023-05-30  |   remove below condition "if slot_duration < task_duration"
            // and just ask "if slot_duration >= task_duration"
            // We can add partial condition when allow splitting min_duration tasks
            if slot_duration >= task_duration {
                acc + slot_duration - task_duration + 1
            } else {
                acc
            }
        });

        if flexibility == 0 {
            self.status = TaskStatus::Impossible;
        }

        self.flexibility = flexibility;
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
    #[ignore]
    fn test_status_uninitialized() {
        // TODO 2023-05-29  |  add rules for TaskStatus Uninitialized
        todo!("test_status_Uninitialized");
    }

    /// Test when Task Status is Blocked
    #[test]
    #[ignore]
    fn test_status_blocked() {
        // TODO 2023-05-29  |  add rules for TaskStatus Blocked
        todo!("test_status_blocked");
    }

    /// Test when Task Status is BudgetMinWaitingForAdjustment
    #[test]
    #[ignore]
    fn test_status_budget_min_wait_adj() {
        // TODO 2023-05-29  |  add rules for TaskStatus BudgetMinWaitingForAdjustment
        todo!("test_status_budget_min_wait_adj");
    }

    mod single_tasks {
        use chrono::Duration;

        use crate::models::{
            slot::Slot,
            task::{Task, TaskStatus},
        };

        /// Simulate one Task in test case bug_215 which is Sleep
        /// ```
        ///     Task: Sleep,
        ///     Duration: 8 hours
        ///     Timing: 22-08
        /// ```
        #[test]
        fn test_sleep() {
            let mut task = Task::mock(
                8,
                0,
                TaskStatus::ReadyToSchedule,
                vec![
                    Slot::mock(Duration::hours(8), 2023, 01, 03, 0, 0),
                    Slot::mock(Duration::hours(10), 2023, 01, 03, 22, 0),
                    Slot::mock(Duration::hours(10), 2023, 01, 04, 22, 0),
                    Slot::mock(Duration::hours(10), 2023, 01, 05, 22, 0),
                    Slot::mock(Duration::hours(10), 2023, 01, 06, 22, 0),
                    Slot::mock(Duration::hours(10), 2023, 01, 07, 22, 0),
                    Slot::mock(Duration::hours(10), 2023, 01, 08, 22, 0),
                    Slot::mock(Duration::hours(2), 2023, 01, 09, 22, 0),
                    // Slot::mock(Duration::hours(10), 2023, 01, 09, 22, 0),
                ],
            );
            dbg!(&task);

            task.calculate_flexibility();
            dbg!(&task);

            assert_eq!(19, task.flexibility);
        }

        /// Simulate a Task in test case bug_215 which is
        /// have highest flexibility because it can be assigned anytime
        /// ```
        ///     Task: Refreshing,
        ///     Duration: 1 hour
        ///     Timing: anytime
        /// ```
        #[test]
        fn test_anytime_1hr() {
            let mut task = Task::mock(
                1,
                0,
                TaskStatus::ReadyToSchedule,
                vec![Slot::mock(Duration::days(7), 2023, 01, 03, 0, 0)],
            );
            dbg!(&task);

            task.calculate_flexibility();
            dbg!(&task);

            assert_eq!(168, task.flexibility);
        }

        /// Simulate a Task in test case bug_215 which is
        /// for taking dineer daily basis
        /// ```
        ///     Task: Dinner,
        ///     Duration: 1 hour
        ///     Timing: daily between (6pm - 9pm)
        /// ```
        #[test]
        fn test_dinner_time() {
            let mut task = Task::mock(
                1,
                0,
                TaskStatus::ReadyToSchedule,
                vec![
                    Slot::mock(Duration::hours(3), 2023, 01, 03, 18, 0),
                    Slot::mock(Duration::hours(3), 2023, 01, 04, 18, 0),
                    Slot::mock(Duration::hours(3), 2023, 01, 05, 18, 0),
                    Slot::mock(Duration::hours(3), 2023, 01, 06, 18, 0),
                    Slot::mock(Duration::hours(3), 2023, 01, 07, 18, 0),
                    Slot::mock(Duration::hours(3), 2023, 01, 08, 18, 0),
                    Slot::mock(Duration::hours(3), 2023, 01, 09, 18, 0),
                ],
            );
            dbg!(&task);

            task.calculate_flexibility();
            dbg!(&task);

            assert_eq!(21, task.flexibility);
        }
    }

    mod multiple_tasks {
        use chrono::Duration;

        use crate::models::{
            slot::Slot,
            task::{Task, TaskStatus},
        };

        /// An edge case test which simulating 2 tasks and avail slots
        ///  in the first day is less than min_duration for first task,
        ///  so other task will be assigned in first instead.
        /// ```
        ///     Task 1: Sleep,
        ///     Duration: 8 hours
        ///     Timing: 22-08
        ///
        ///     Task 2: Thinking,
        ///     Duration: 1 hour
        ///     Timing: anytime
        /// ```
        #[test]
        fn test_avail_slots_less_than_dur() {
            let mut task = Task::mock(
                1,
                0,
                TaskStatus::ReadyToSchedule,
                vec![
                    Slot::mock(Duration::hours(3), 2023, 01, 03, 18, 0),
                    Slot::mock(Duration::hours(3), 2023, 01, 04, 18, 0),
                    Slot::mock(Duration::hours(3), 2023, 01, 05, 18, 0),
                    Slot::mock(Duration::hours(3), 2023, 01, 06, 18, 0),
                    Slot::mock(Duration::hours(3), 2023, 01, 07, 18, 0),
                    Slot::mock(Duration::hours(3), 2023, 01, 08, 18, 0),
                    Slot::mock(Duration::hours(3), 2023, 01, 09, 18, 0),
                ],
            );
            dbg!(&task);

            task.calculate_flexibility();
            dbg!(&task);

            assert_eq!(21, task.flexibility);
        }
    }

    ///
    #[test]
    #[ignore]
    fn test_overlapped_tasks() {
        todo!("test_overlapped_tasks");
    }
}
