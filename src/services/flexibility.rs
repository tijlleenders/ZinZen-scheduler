use crate::models::{
    input::StepsToPlace,
    step::{Step, StepStatus},
};

impl StepsToPlace {
    /// Calculate flexibility for each step in steps then sort them
    pub fn sort_on_flexibility(&mut self) {
        self.calculate_flexibilities();
        self.steps.sort();
    }

    fn calculate_flexibilities(&mut self) {
        for step in self.steps.iter_mut() {
            step.calculate_flexibility();
        }
    }
}

impl Step {
    /// Calculate flexibility of a step slots
    pub fn calculate_flexibility(&mut self) {
        if self.status == StepStatus::Scheduled || self.status == StepStatus::Impossible {
            let message = format!(
                "StepStatus must be ReadyToSchedule, but it is now StepStatus::{:?}",
                self.status.clone()
            );
            log::debug!("{:#?}", message);
            return;
        }

        let step_duration = self.duration;
        let flexibility = self.slots.iter().fold(0, |acc, slot| {
            let slot_duration = slot.duration_as_hours();

            if slot_duration >= step_duration {
                acc + slot_duration - step_duration + 1
            } else {
                /*
                TODO 2023-06-15: below fixed flexibility calculation for goal
                "sleep" for test bug_215, but will affect other tests and not
                accurate for other cases like budgeting steps
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

        /// Test when StepStatus::Blocked
        /// - Expected Should panic when StepStatus is not ReadyToSchedule
        #[test]
        #[should_panic]
        #[ignore]
        fn test_blocked() {
            let mut step = Step::mock(
                "test",
                1,
                168,
                StepStatus::Blocked,
                vec![Slot::mock(Duration::days(6), 2023, 5, 1, 0, 0)],
                None,
            );
            step.calculate_flexibility();
        }

        /// Test when StepStatus::BudgetMinWaitingForAdjustment
        /// - Expected Should panic when StepStatus is not ReadyToSchedule
        #[test]
        #[should_panic]
        #[ignore]
        fn test_budget_min_for_adjstmnt() {
            let mut step = Step::mock(
                "test",
                1,
                168,
                StepStatus::BudgetMinWaitingForAdjustment,
                vec![Slot::mock(Duration::days(6), 2023, 5, 1, 0, 0)],
                None,
            );
            step.calculate_flexibility();
        }

        /// Test when StepStatus::Impossible
        /// - Expected Should panic when StepStatus is not ReadyToSchedule
        #[test]
        #[should_panic]
        #[ignore]
        fn test_impossible() {
            let mut step = Step::mock(
                "test",
                1,
                168,
                StepStatus::Impossible,
                vec![Slot::mock(Duration::days(6), 2023, 5, 1, 0, 0)],
                None,
            );
            step.calculate_flexibility();
        }

        /// Test when StepStatus::Scheduled
        /// - Expected Should panic when StepStatus is not ReadyToSchedule
        #[test]
        #[should_panic]
        #[ignore]
        fn test_scheduled() {
            let mut step = Step::mock(
                "test",
                1,
                168,
                StepStatus::Scheduled,
                vec![Slot::mock(Duration::days(6), 2023, 5, 1, 0, 0)],
                None,
            );
            step.calculate_flexibility();
        }

        /// Test when StepStatus::Uninitialized
        /// - Expected Should panic when StepStatus is not ReadyToSchedule
        #[test]
        #[should_panic]
        #[ignore]
        fn test_uninitialized() {
            let mut step = Step::mock(
                "test",
                1,
                168,
                StepStatus::Uninitialized,
                vec![Slot::mock(Duration::days(6), 2023, 5, 1, 0, 0)],
                None,
            );
            step.calculate_flexibility();
        }
    }

    mod single_steps {
        use chrono::Duration;

        use crate::models::{
            slot::Slot,
            step::{Step, StepStatus},
        };

        /// Simulate one Step in test case bug_215 which is Sleep
        /// ```markdown
        ///     Step: Sleep,
        ///     Duration: 8 hours
        ///     Timing: 22-08
        /// ```
        #[test]
        fn test_sleep() {
            let mut step = Step::mock(
                "test",
                8,
                0,
                StepStatus::ReadyToSchedule,
                vec![
                    Slot::mock(Duration::hours(8), 2023, 1, 3, 0, 0),
                    Slot::mock(Duration::hours(10), 2023, 1, 3, 22, 0),
                    Slot::mock(Duration::hours(10), 2023, 1, 4, 22, 0),
                    Slot::mock(Duration::hours(10), 2023, 1, 5, 22, 0),
                    Slot::mock(Duration::hours(10), 2023, 1, 6, 22, 0),
                    Slot::mock(Duration::hours(10), 2023, 1, 7, 22, 0),
                    Slot::mock(Duration::hours(10), 2023, 1, 8, 22, 0),
                    // Slot::mock(Duration::hours(2), 2023, 01, 09, 22, 0),
                    Slot::mock(Duration::hours(10), 2023, 1, 9, 22, 0),
                ],
                None,
            );

            step.calculate_flexibility();

            assert_eq!(22, step.flexibility);
        }

        /// Simulate a Step in test case bug_215 which is
        /// have highest flexibility because it can be assigned anytime
        /// ```markdown
        ///     Step: Refreshing,
        ///     Duration: 1 hour
        ///     Timing: anytime
        /// ```
        #[test]
        fn test_anytime_1hr() {
            let mut step = Step::mock(
                "test",
                1,
                0,
                StepStatus::ReadyToSchedule,
                vec![Slot::mock(Duration::days(7), 2023, 1, 3, 0, 0)],
                None,
            );

            step.calculate_flexibility();

            assert_eq!(168, step.flexibility);
        }

        /// Simulate a Step in test case bug_215 which is
        /// for taking dineer daily basis
        /// ```markdown
        ///     Step: Dinner,
        ///     Duration: 1 hour
        ///     Timing: daily between (6pm - 9pm)
        /// ```
        #[test]
        fn test_dinner_time() {
            let mut step = Step::mock(
                "test",
                1,
                0,
                StepStatus::ReadyToSchedule,
                vec![
                    Slot::mock(Duration::hours(3), 2023, 1, 3, 18, 0),
                    Slot::mock(Duration::hours(3), 2023, 1, 4, 18, 0),
                    Slot::mock(Duration::hours(3), 2023, 1, 5, 18, 0),
                    Slot::mock(Duration::hours(3), 2023, 1, 6, 18, 0),
                    Slot::mock(Duration::hours(3), 2023, 1, 7, 18, 0),
                    Slot::mock(Duration::hours(3), 2023, 1, 8, 18, 0),
                    Slot::mock(Duration::hours(3), 2023, 1, 9, 18, 0),
                ],
                None,
            );

            step.calculate_flexibility();

            assert_eq!(21, step.flexibility);
        }
    }

    mod multiple_steps {
        use chrono::Duration;

        use crate::models::{
            slot::Slot,
            step::{Step, StepStatus},
        };

        /// An edge case test which simulating 2 steps and avail slots
        ///  in the first day is less than min_duration for first step,
        ///  so other step will be assigned in first instead.
        /// ```markdown
        ///     Step 1: Sleep,
        ///     Duration: 8 hours
        ///     Timing: 22-08
        ///
        ///     Step 2: Thinking,
        ///     Duration: 1 hour
        ///     Timing: anytime
        /// ```
        #[test]
        fn test_avail_slots_less_than_dur() {
            // todo!("test_avail_slots_less_than_dur");

            let sleep_step = Step::mock(
                "test",
                8,
                0,
                StepStatus::ReadyToSchedule,
                vec![
                    Slot::mock(Duration::hours(8), 2023, 1, 3, 0, 0),
                    Slot::mock(Duration::hours(10), 2023, 1, 3, 22, 0),
                    Slot::mock(Duration::hours(10), 2023, 1, 4, 22, 0),
                    Slot::mock(Duration::hours(10), 2023, 1, 5, 22, 0),
                    Slot::mock(Duration::hours(10), 2023, 1, 6, 22, 0),
                    Slot::mock(Duration::hours(10), 2023, 1, 7, 22, 0),
                    Slot::mock(Duration::hours(10), 2023, 1, 8, 22, 0),
                    // Slot::mock(Duration::hours(2), 2023, 01, 09, 22, 0),
                    Slot::mock(Duration::hours(10), 2023, 1, 9, 22, 0),
                ],
                None,
            );

            let thinking_step = Step::mock(
                "thinking",
                1,
                0,
                StepStatus::ReadyToSchedule,
                vec![Slot::mock(Duration::days(7), 2023, 1, 3, 0, 0)],
                None,
            );

            let steps = vec![sleep_step, thinking_step];

            for mut step in steps {
                step.calculate_flexibility();

                if step.duration == 8 {
                    assert_eq!(22, step.flexibility);
                } else if step.duration == 1 {
                    assert_eq!(168, step.flexibility);
                } else {
                    panic!();
                }
            }
        }
    }

    ///
    #[test]
    #[ignore]
    fn test_overlapped_steps() {
        todo!("test_overlapped_steps");
    }
}
