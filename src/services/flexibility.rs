use crate::models::{
    input::StepsToPlace,
    step::{Step, StepStatus},
};

impl StepsToPlace {
    /// Calculate flexibility for each task in tasks then sort them
    pub fn sort_on_flexibility(&mut self) {
        self.calculate_flexibilities();
        self.steps.sort();
    }

    fn calculate_flexibilities(&mut self) {
        for task in self.steps.iter_mut() {
            task.calculate_flexibility();
        }
    }
}

impl Step {
    /// Calculate flexibility of a task slots
    pub fn calculate_flexibility(&mut self) {
        if self.status == StepStatus::Scheduled || self.status == StepStatus::Impossible {
            let message = format!(
                "TaskStatus must be ReadyToSchedule, but it is now TaskStatus::{:?}",
                self.status.clone()
            );
            dbg!(message);
            return;
        }

        let task_duration = self.duration;
        let flexibility = self.slots.iter().fold(0, |acc, slot| {
            let slot_duration = slot.duration_as_hours();

            if slot_duration >= task_duration {
                acc + slot_duration - task_duration + 1
            } else {
                /*
                TODO 2023-06-15: below fixed flexibility calculation for goal
                "sleep" for test bug_215, but will affect other tests and not
                accurate for other cases like budgeting tasks
                */

                acc
            }
        });

        if flexibility == 0 {
            self.status = StepStatus::Impossible;
        }

        self.flexibility = flexibility;
    }
}

#[cfg(test)]
mod tests {

    mod status {
        use chrono::Duration;

        use crate::models::{
            slot::Slot,
            step::{Step, StepStatus},
        };

        /// Test when TaskStatus::Blocked
        /// - Expected Should panic when TaskStatus is not ReadyToSchedule
        #[test]
        #[should_panic]
        #[ignore]
        fn test_blocked() {
            let mut task = Step::mock(
                "test",
                1,
                168,
                StepStatus::Blocked,
                vec![Slot::mock(Duration::days(6), 2023, 05, 01, 0, 0)],
                None,
            );
            task.calculate_flexibility();
        }

        /// Test when TaskStatus::BudgetMinWaitingForAdjustment
        /// - Expected Should panic when TaskStatus is not ReadyToSchedule
        #[test]
        #[should_panic]
        #[ignore]
        fn test_budget_min_for_adjstmnt() {
            let mut task = Step::mock(
                "test",
                1,
                168,
                StepStatus::BudgetMinWaitingForAdjustment,
                vec![Slot::mock(Duration::days(6), 2023, 05, 01, 0, 0)],
                None,
            );
            task.calculate_flexibility();
        }

        /// Test when TaskStatus::Impossible
        /// - Expected Should panic when TaskStatus is not ReadyToSchedule
        #[test]
        #[should_panic]
        #[ignore]
        fn test_impossible() {
            let mut task = Step::mock(
                "test",
                1,
                168,
                StepStatus::Impossible,
                vec![Slot::mock(Duration::days(6), 2023, 05, 01, 0, 0)],
                None,
            );
            task.calculate_flexibility();
        }

        /// Test when TaskStatus::Scheduled
        /// - Expected Should panic when TaskStatus is not ReadyToSchedule
        #[test]
        #[should_panic]
        #[ignore]
        fn test_scheduled() {
            let mut task = Step::mock(
                "test",
                1,
                168,
                StepStatus::Scheduled,
                vec![Slot::mock(Duration::days(6), 2023, 05, 01, 0, 0)],
                None,
            );
            task.calculate_flexibility();
        }

        /// Test when TaskStatus::Uninitialized
        /// - Expected Should panic when TaskStatus is not ReadyToSchedule
        #[test]
        #[should_panic]
        #[ignore]
        fn test_uninitialized() {
            let mut task = Step::mock(
                "test",
                1,
                168,
                StepStatus::Uninitialized,
                vec![Slot::mock(Duration::days(6), 2023, 05, 01, 0, 0)],
                None,
            );
            task.calculate_flexibility();
        }
    }

    mod single_tasks {
        use chrono::Duration;

        use crate::models::{
            slot::Slot,
            step::{Step, StepStatus},
        };

        /// Simulate one Task in test case bug_215 which is Sleep
        /// ```
        ///     Task: Sleep,
        ///     Duration: 8 hours
        ///     Timing: 22-08
        /// ```
        #[test]
        fn test_sleep() {
            let mut task = Step::mock(
                "test",
                8,
                0,
                StepStatus::ReadyToSchedule,
                vec![
                    Slot::mock(Duration::hours(8), 2023, 01, 03, 0, 0),
                    Slot::mock(Duration::hours(10), 2023, 01, 03, 22, 0),
                    Slot::mock(Duration::hours(10), 2023, 01, 04, 22, 0),
                    Slot::mock(Duration::hours(10), 2023, 01, 05, 22, 0),
                    Slot::mock(Duration::hours(10), 2023, 01, 06, 22, 0),
                    Slot::mock(Duration::hours(10), 2023, 01, 07, 22, 0),
                    Slot::mock(Duration::hours(10), 2023, 01, 08, 22, 0),
                    // Slot::mock(Duration::hours(2), 2023, 01, 09, 22, 0),
                    Slot::mock(Duration::hours(10), 2023, 01, 09, 22, 0),
                ],
                None,
            );
            dbg!(&task);

            task.calculate_flexibility();
            dbg!(&task);

            assert_eq!(22, task.flexibility);
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
            let mut task = Step::mock(
                "test",
                1,
                0,
                StepStatus::ReadyToSchedule,
                vec![Slot::mock(Duration::days(7), 2023, 01, 03, 0, 0)],
                None,
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
            let mut task = Step::mock(
                "test",
                1,
                0,
                StepStatus::ReadyToSchedule,
                vec![
                    Slot::mock(Duration::hours(3), 2023, 01, 03, 18, 0),
                    Slot::mock(Duration::hours(3), 2023, 01, 04, 18, 0),
                    Slot::mock(Duration::hours(3), 2023, 01, 05, 18, 0),
                    Slot::mock(Duration::hours(3), 2023, 01, 06, 18, 0),
                    Slot::mock(Duration::hours(3), 2023, 01, 07, 18, 0),
                    Slot::mock(Duration::hours(3), 2023, 01, 08, 18, 0),
                    Slot::mock(Duration::hours(3), 2023, 01, 09, 18, 0),
                ],
                None,
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
            step::{Step, StepStatus},
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
            // todo!("test_avail_slots_less_than_dur");

            let sleep_task = Step::mock(
                "test",
                8,
                0,
                StepStatus::ReadyToSchedule,
                vec![
                    Slot::mock(Duration::hours(8), 2023, 01, 03, 0, 0),
                    Slot::mock(Duration::hours(10), 2023, 01, 03, 22, 0),
                    Slot::mock(Duration::hours(10), 2023, 01, 04, 22, 0),
                    Slot::mock(Duration::hours(10), 2023, 01, 05, 22, 0),
                    Slot::mock(Duration::hours(10), 2023, 01, 06, 22, 0),
                    Slot::mock(Duration::hours(10), 2023, 01, 07, 22, 0),
                    Slot::mock(Duration::hours(10), 2023, 01, 08, 22, 0),
                    // Slot::mock(Duration::hours(2), 2023, 01, 09, 22, 0),
                    Slot::mock(Duration::hours(10), 2023, 01, 09, 22, 0),
                ],
                None,
            );

            let thinking_task = Step::mock(
                "thinking",
                1,
                0,
                StepStatus::ReadyToSchedule,
                vec![Slot::mock(Duration::days(7), 2023, 01, 03, 0, 0)],
                None,
            );

            let tasks = vec![sleep_task, thinking_task];

            for mut task in tasks {
                task.calculate_flexibility();
                dbg!(&task);
                if task.duration == 8 {
                    assert_eq!(22, task.flexibility);
                } else if task.duration == 1 {
                    assert_eq!(168, task.flexibility);
                } else {
                    assert!(false);
                }
            }
        }
    }

    ///
    #[test]
    #[ignore]
    fn test_overlapped_tasks() {
        todo!("test_overlapped_tasks");
    }
}
