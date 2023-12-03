use crate::models::{
    slot::{Slot, SlotConflict},
    step::{Step, StepStatus},
};
use chrono::Duration;

/// Find best slots for steps by splitting, finding conflicts and return list of slots which can be scheduled
pub(crate) fn find_best_slots_for_first_step(steps: &[Step]) -> Option<Vec<Slot>> {
    if steps.is_empty() {
        return None;
    }

    let step = &steps[0];
    let slot_conflicts = step.get_conflicts_in_steps(steps);

    // return the full array of potential slots in order of most ideal to least
    // get_conflicts_in_steps returns the reverse order, so iterate in reverse
    Some(slot_conflicts.iter().rev().map(|sc| sc.slot).collect())
}

impl Slot {
    /// Get conflicts of a slot in list of slots
    fn get_conflicts_in_slots(&self, slots_list: &[Slot]) -> SlotConflict {
        let mut count: usize = 0;

        slots_list.iter().for_each(|slot| {
            count += self.intersection(slot);
        });

        SlotConflict {
            slot: *self,
            num_conflicts: count,
        }
    }

    /// Get conflicts of a slot in list of Steps
    /// - NOTE: function check conflicts for only steps with status StepStatus::ReadyToSchedule
    fn get_conflicts_in_steps(&self, slots_list: &[Step]) -> SlotConflict {
        let mut count: usize = 0;
        slots_list
            .iter()
            .filter(|step| step.status == StepStatus::ReadyToSchedule)
            .for_each(|step| {
                let slot_conflict = self.get_conflicts_in_slots(step.slots.as_slice());
                count += slot_conflict.num_conflicts;
            });

        SlotConflict {
            slot: *self,
            num_conflicts: count,
        }
    }

    /// Generate list of schedulable slots which can be scheduled in a given
    /// slot based on given slot's duration and given slot
    /// - Example:
    ///     ```markdown
    ///     slot: 22-08 (10 hours)
    ///     duration: 8 (hours)
    ///
    ///     So will return 3 slots:
    ///         Slot: 22-06
    ///         Slot: 23-07
    ///         Slot: 22-08
    ///
    ///
    ///     slot: 07-11 (4 hours)
    ///     duration: 1 hour
    ///     return: slots:
    ///         - Slot: 07-08
    ///         - Slot: 08-09
    ///         - Slot: 09-10
    ///         - Slot: 10-11
    ///     ```
    fn generate_schedulable_slots(&self, duration: usize) -> Vec<Slot> {
        // ===
        /*
        Algorithm:
        - if slot's duration less than duration, return empty list
        - if slot's duration equal to duration, return same slot
        - if slot's duration greater than duration:
            - calculate flexibility by slot's duration - duration + 1
            - loop from 0 to flexibility
                - Initialize a slot with slot's start time + duration
                - add it to schedulable_slots

            return schedulable_slots
        */
        // ===
        let slot_duration = self.span();
        let mut schedulable_slots = vec![];

        match slot_duration.cmp(&duration) {
            std::cmp::Ordering::Less => {
                return schedulable_slots;
            }
            std::cmp::Ordering::Equal => {
                return vec![*self];
            }
            std::cmp::Ordering::Greater => {
                let flexibility = slot_duration - duration + 1;

                let mut start_time = self.start;
                let mut end_time = start_time + Duration::hours(duration as i64);

                for _ in 0..flexibility {
                    if end_time <= self.end {
                        let new_slot = Slot {
                            start: start_time,
                            end: end_time,
                        };

                        schedulable_slots.push(new_slot);

                        start_time += Duration::hours(1);
                        end_time = start_time + Duration::hours(duration as i64);
                    }
                }
            }
        }

        schedulable_slots
    }
}

impl Step {
    /// Get conflicts of a step slots in list of Steps
    /// - Notes about function:
    ///     - It check conflicts for only steps with status StepStatus::ReadyToSchedule
    ///     - It returns sorted list of SlotConflict based on slot.start then num_conflicts
    ///     - This sorting puts the optimal slots (and the first occuring slots) last. This makes sure that when we pop an element from the list we get the optimal element.
    ///     - Splitting slots into schedulable slots based on slot's timing and step's duration
    fn get_conflicts_in_steps(&self, steps_list: &[Step]) -> Vec<SlotConflict> {
        // Remove given_step from steps_list to avoid wrong conflicts calculation

        let steps_list: Vec<Step> = steps_list
            .iter()
            .filter(|step| step.id != self.id)
            .cloned()
            .collect();

        let mut conflicts_list: Vec<SlotConflict> = vec![];

        if self.status != StepStatus::ReadyToSchedule {
            return conflicts_list;
        }

        self.slots.iter().for_each(|slot| {
            let schedulable_slots = slot.generate_schedulable_slots(self.duration);

            schedulable_slots.iter().for_each(|schedulable_slot| {
                let slot_conflict = schedulable_slot.get_conflicts_in_steps(steps_list.as_slice());

                conflicts_list.push(slot_conflict);
            });
        });

        conflicts_list.sort_by(|a, b| b.slot.start.partial_cmp(&a.slot.start).unwrap());
        conflicts_list.sort_by(|a, b| b.num_conflicts.partial_cmp(&a.num_conflicts).unwrap());

        conflicts_list
    }
}

#[cfg(test)]
mod tests {
    use chrono::Duration;

    use super::*;

    #[test]
    fn test_empty_steps() {
        let steps_to_place = Vec::new();
        assert_eq!(find_best_slots_for_first_step(&steps_to_place), None);
    }

    /// Test single step
    /// Expected:
    /// ```markdown
    /// Some([Slot {
    ///     start: 2023-05-01 00
    ///     end: 2023-05-01 01 }])
    /// ```
    #[test]
    fn test_single_step() {
        let step = Step::mock(
            "test",
            1,
            168,
            StepStatus::ReadyToSchedule,
            vec![Slot::mock(Duration::days(6), 2023, 5, 1, 0, 0)],
            None,
        );

        let expected = Some(Slot::mock(Duration::hours(1), 2023, 5, 1, 0, 0));

        let result = find_best_slots_for_first_step(&vec![step]).and_then(|v| v.first().cloned());

        assert_eq!(result, expected);
    }

    /// Test multiple steps with conflicts
    #[test]
    fn test_find_best_slot_amongst_multiple_steps_with_conflicts() {
        let first_step = Step::mock(
            "test",
            1,
            5,
            StepStatus::ReadyToSchedule,
            vec![Slot::mock(Duration::hours(5), 2023, 5, 1, 12, 0)],
            None,
        );

        let conflicting_step_1 = Step::mock(
            "test-conflict",
            1,
            2,
            StepStatus::ReadyToSchedule,
            vec![Slot::mock(Duration::hours(2), 2023, 5, 1, 12, 0)],
            None,
        );

        let conflicting_step_2 = Step::mock(
            "test-conflict-2",
            1,
            2,
            StepStatus::ReadyToSchedule,
            vec![Slot::mock(Duration::hours(2), 2023, 5, 1, 15, 0)],
            None,
        );

        let expected = Some(Slot::mock(Duration::hours(1), 2023, 5, 1, 14, 0));

        let result = find_best_slots_for_first_step(&vec![
            first_step,
            conflicting_step_1,
            conflicting_step_2,
        ])
        .and_then(|v| v.first().cloned());

        assert_eq!(result, expected);
    }

    /// Test multiple steps with conflicts when already splitted
    #[test]
    fn test_find_best_slot_amongst_multiple_steps_with_conflicts_already_splitted() {
        let first_step = Step::mock(
            "test",
            1,
            5,
            StepStatus::ReadyToSchedule,
            vec![
                Slot::mock(Duration::hours(1), 2023, 5, 1, 12, 0),
                Slot::mock(Duration::hours(1), 2023, 5, 1, 13, 0),
                Slot::mock(Duration::hours(1), 2023, 5, 1, 14, 0),
                Slot::mock(Duration::hours(1), 2023, 5, 1, 15, 0),
                Slot::mock(Duration::hours(1), 2023, 5, 1, 16, 0),
            ],
            None,
        );

        let conflicting_step_1 = Step::mock(
            "test-conflict",
            1,
            2,
            StepStatus::ReadyToSchedule,
            vec![
                Slot::mock(Duration::hours(1), 2023, 5, 1, 12, 0),
                Slot::mock(Duration::hours(1), 2023, 5, 1, 13, 0),
            ],
            None,
        );

        let conflicting_step_2 = Step::mock(
            "test-conflict-2",
            1,
            2,
            StepStatus::ReadyToSchedule,
            vec![
                Slot::mock(Duration::hours(1), 2023, 5, 1, 15, 0),
                Slot::mock(Duration::hours(1), 2023, 5, 1, 16, 0),
            ],
            None,
        );

        let expected = Some(Slot::mock(Duration::hours(1), 2023, 5, 1, 14, 0));

        let result = find_best_slots_for_first_step(&vec![
            first_step,
            conflicting_step_1,
            conflicting_step_2,
        ])
        .and_then(|v| v.first().cloned());

        assert_eq!(result, expected);
    }

    /// - Example:
    ///     ```markdown
    ///     slot: 22-08 (10 hours)
    ///     duration: 8 (hours)
    ///
    ///     So will return 3 slots:
    ///         Slot: 22-06
    ///         Slot: 23-07
    ///         Slot: 22-08
    ///     ```
    #[test]
    fn test_generate_schedulable_slots() {
        let slot = Slot::mock(Duration::hours(10), 2023, 6, 1, 22, 0);
        let duration: usize = 8;

        let result = slot.generate_schedulable_slots(duration);
        let expected = vec![
            Slot::mock(Duration::hours(8), 2023, 6, 1, 22, 0),
            Slot::mock(Duration::hours(8), 2023, 6, 1, 23, 0),
            Slot::mock(Duration::hours(8), 2023, 6, 2, 0, 0),
        ];

        assert_eq!(result, expected);
    }

    mod conflicts {
        use crate::models::{
            slot::Slot,
            step::{Step, StepStatus},
        };
        use chrono::Duration;

        mod slot {
            use super::*;

            #[test]
            fn test_no_conflicts_in_slots() {
                let slots_list = vec![
                    Slot::mock(Duration::hours(5), 2023, 6, 1, 0, 0),
                    Slot::mock(Duration::hours(10), 2023, 6, 1, 9, 0),
                ];

                let slot_to_search = Slot::mock(Duration::hours(2), 2023, 6, 1, 6, 0);
                let conflicts = slot_to_search.get_conflicts_in_slots(&slots_list);

                assert_eq!(conflicts.slot, slot_to_search);
                assert_eq!(conflicts.num_conflicts, 0);
            }

            /// Testing many conflicts for a slot within list of slots
            /// ```markdown
            /// slot to search:
            ///     2023-06-01 01 to 03
            ///
            /// list of slots to search in:
            ///     2023-05-30 to 2023-06-04 (conflicts )
            ///     2023-06-01 00 to 05 (conflicts)
            ///     2023-06-02 00 to 08
            ///     2023-06-03 00 to 08
            ///
            /// Expected:
            ///     SlotConflict:
            ///         - Slot: 2023-06-01 01 to 03
            ///         - Conflicts: 2
            /// ```
            #[test]
            fn test_conflicts_in_slots() {
                let slots_list = vec![
                    Slot::mock(Duration::days(5), 2023, 5, 30, 0, 0),
                    Slot::mock(Duration::hours(5), 2023, 6, 1, 0, 0),
                    Slot::mock(Duration::hours(8), 2023, 6, 2, 0, 0),
                    Slot::mock(Duration::hours(8), 2023, 6, 3, 0, 0),
                ];

                let slot_to_search = Slot::mock(Duration::hours(2), 2023, 6, 1, 1, 0);

                let conflicts = slot_to_search.get_conflicts_in_slots(&slots_list);

                assert_eq!(conflicts.slot, slot_to_search);
                assert_eq!(conflicts.num_conflicts, 4);
            }

            #[test]
            fn test_no_conflicts_in_steps() {
                let steps_list = vec![
                    Step::mock(
                        "step 1",
                        10,
                        0,
                        StepStatus::ReadyToSchedule,
                        vec![
                            Slot::mock(Duration::hours(5), 2023, 6, 1, 0, 0),
                            Slot::mock(Duration::hours(10), 2023, 6, 1, 9, 0),
                        ],
                        None,
                    ),
                    Step::mock(
                        "step 2",
                        2,
                        0,
                        StepStatus::ReadyToSchedule,
                        vec![
                            Slot::mock(Duration::hours(5), 2023, 6, 5, 0, 0),
                            Slot::mock(Duration::hours(10), 2023, 6, 6, 9, 0),
                        ],
                        None,
                    ),
                ];

                let slot_to_search = Slot::mock(Duration::hours(2), 2023, 5, 1, 6, 0);

                let conflicts = slot_to_search.get_conflicts_in_steps(&steps_list);

                assert_eq!(conflicts.slot, slot_to_search);
                assert_eq!(conflicts.num_conflicts, 0);
            }

            /// Testing many conflicts for a slot within list of
            /// steps but, one of them with status Scheduled
            /// ```markdown
            /// slot to search:
            ///     2023-06-01 01 to 03
            ///
            /// Taks:
            ///     Step 1 ReadyToSchedule
            ///         2023-05-30 to 2023-06-04 (conflicts)
            ///         2023-06-01 00 to 05 (conflicts)
            ///         2023-06-02 00 to 08
            ///         2023-06-03 00 to 08
            ///     Step 2 ReadyToSchedule
            ///         2023-05-30 to 2023-06-04 (conflicts)
            ///         2023-06-01 00 to 05 (conflicts)
            ///         2023-06-02 00 to 08
            ///         2023-06-03 00 to 08
            ///     Step 3 Scheduled
            ///         2023-05-30 to 2023-06-04 (will not conflicts)
            ///         2023-06-01 00 to 05 (will not conflicts)
            ///         2023-06-02 00 to 08
            ///         2023-06-03 00 to 08
            ///     
            ///
            /// Expected:
            ///     SlotConflict:
            ///         - Slot: 2023-06-01 01 to 03
            ///         - Conflicts: 4
            /// ```
            #[test]
            fn test_conflicts_in_steps() {
                let slots_list = vec![
                    Slot::mock(Duration::days(5), 2023, 5, 30, 0, 0),
                    Slot::mock(Duration::hours(5), 2023, 6, 1, 0, 0),
                    Slot::mock(Duration::hours(8), 2023, 6, 2, 0, 0),
                    Slot::mock(Duration::hours(8), 2023, 6, 3, 0, 0),
                ];

                let steps_list = vec![
                    Step::mock(
                        "step 1",
                        10,
                        0,
                        StepStatus::ReadyToSchedule,
                        slots_list.clone(),
                        None,
                    ),
                    Step::mock(
                        "step 2",
                        2,
                        0,
                        StepStatus::ReadyToSchedule,
                        slots_list.clone(),
                        None,
                    ),
                    Step::mock("step 2", 2, 0, StepStatus::Scheduled, slots_list, None),
                ];

                let slot_to_search = Slot::mock(Duration::hours(2), 2023, 6, 1, 1, 0);

                let conflicts = slot_to_search.get_conflicts_in_steps(&steps_list);

                assert_eq!(conflicts.slot, slot_to_search);
                assert_eq!(conflicts.num_conflicts, 8);
            }

            /// Testing finding conflicted slots on hour-based.
            /// This related to case `split-1`
            /// Issue: https://github.com/tijlleenders/ZinZen-scheduler/issues/343
            ///
            /// ```markdown
            /// slot to search:
            ///     name: work
            ///     timing: 2022-09-01 08 to 16
            ///     durtion: 8
            ///
            /// Steps list:
            ///     Title   | Duration  |   Slots    
            ///     work    |   8       | 2022-09-01 08 to 21
            ///     dentist |   1       | 2022-09-01 12 to 15
            ///
            /// Step to search in:
            ///     dentist |   1       | 2022-09-01 12 to 15
            ///
            /// * NOTE: function will split slots into min_duration based,
            /// then finding conflicts based on each
            ///
            /// ```
            #[test]
            fn test_no_conflicts_in_slots_for_work() {
                let steps_list = vec![Step::mock(
                    "dentist",
                    1,
                    2,
                    StepStatus::ReadyToSchedule,
                    vec![Slot::mock(Duration::hours(3), 2022, 9, 1, 12, 0)],
                    None,
                )];

                let slot_to_search = Slot::mock(Duration::hours(3), 2022, 9, 1, 12, 0);

                let conflicts = slot_to_search.get_conflicts_in_steps(&steps_list);

                assert_eq!(conflicts.slot, slot_to_search);
                assert_eq!(conflicts.num_conflicts, 3);
            }
        }

        mod step {
            use crate::models::slot::SlotConflict;

            use super::*;

            #[test]
            fn test_no_conflicts_in_steps() {
                let steps_list = vec![
                    Step::mock(
                        "step 1",
                        10,
                        0,
                        StepStatus::ReadyToSchedule,
                        vec![
                            Slot::mock(Duration::hours(5), 2023, 6, 1, 0, 0),
                            Slot::mock(Duration::hours(10), 2023, 6, 1, 9, 0),
                        ],
                        None,
                    ),
                    Step::mock(
                        "step 2",
                        2,
                        0,
                        StepStatus::ReadyToSchedule,
                        vec![
                            Slot::mock(Duration::hours(5), 2023, 6, 5, 0, 0),
                            Slot::mock(Duration::hours(10), 2023, 6, 6, 9, 0),
                        ],
                        None,
                    ),
                ];

                let step_to_search = Step::mock(
                    "test step",
                    1,
                    0,
                    StepStatus::ReadyToSchedule,
                    vec![
                        Slot::mock(Duration::hours(2), 2023, 5, 1, 6, 0),
                        Slot::mock(Duration::hours(2), 2023, 6, 6, 0, 0),
                    ],
                    None,
                );

                let conflicts = step_to_search.get_conflicts_in_steps(&steps_list);

                // Will result into 4 slots with 1 hour block

                let expected: Vec<SlotConflict> = vec![
                    SlotConflict {
                        slot: Slot::mock(Duration::hours(1), 2023, 6, 6, 1, 0),
                        num_conflicts: 0,
                    },
                    SlotConflict {
                        slot: Slot::mock(Duration::hours(1), 2023, 6, 6, 0, 0),
                        num_conflicts: 0,
                    },
                    SlotConflict {
                        slot: Slot::mock(Duration::hours(1), 2023, 5, 1, 7, 0),
                        num_conflicts: 0,
                    },
                    SlotConflict {
                        slot: Slot::mock(Duration::hours(1), 2023, 5, 1, 6, 0),
                        num_conflicts: 0,
                    },
                ];

                assert_eq!(conflicts, expected);
            }

            /// Testing conflicts for a step with Scheduled status
            /// within list of steps
            /// ```markdown
            /// step to search:
            ///         Scheduled
            ///         2023-06-01 01 to 03
            ///         2023-06-02 08 to 09
            ///
            /// Taks:
            ///     Step 1 ReadyToSchedule
            ///         2023-05-30 to 2023-06-04 (will not conflicts)
            ///         2023-06-01 00 to 05 (will not conflicts)
            ///         2023-06-02 00 to 08
            ///         2023-06-03 00 to 08
            ///     Step 2 ReadyToSchedule
            ///         2023-05-30 to 2023-06-04 (will not conflicts)
            ///         2023-06-01 00 to 05 (will not conflicts)
            ///         2023-06-02 00 to 08
            ///         2023-06-03 00 to 08
            ///     Step 3 Scheduled
            ///         2023-05-30 to 2023-06-04 (will not conflicts)
            ///         2023-06-01 00 to 05 (will not conflicts)
            ///         2023-06-02 00 to 08
            ///         2023-06-03 00 to 08
            ///     
            ///
            /// Expected:
            ///     empty list of SlotConflicts
            /// ```
            #[test]
            fn test_step_is_invalid() {
                let slots_list = vec![
                    Slot::mock(Duration::days(5), 2023, 5, 30, 0, 0),
                    Slot::mock(Duration::hours(5), 2023, 6, 1, 0, 0),
                    Slot::mock(Duration::hours(8), 2023, 6, 2, 0, 0),
                    Slot::mock(Duration::hours(8), 2023, 6, 3, 0, 0),
                ];

                let steps_list = vec![
                    Step::mock(
                        "step 1",
                        10,
                        0,
                        StepStatus::ReadyToSchedule,
                        slots_list.clone(),
                        None,
                    ),
                    Step::mock(
                        "step 2",
                        2,
                        0,
                        StepStatus::ReadyToSchedule,
                        slots_list.clone(),
                        None,
                    ),
                    Step::mock("step 2", 2, 0, StepStatus::Scheduled, slots_list, None),
                ];

                let step_to_search = Step::mock(
                    "test step 1",
                    2,
                    0,
                    StepStatus::Scheduled,
                    vec![
                        Slot::mock(Duration::hours(2), 2023, 6, 1, 1, 0),
                        Slot::mock(Duration::hours(2), 2023, 6, 2, 8, 0),
                    ],
                    None,
                );

                let conflicts = step_to_search.get_conflicts_in_steps(&steps_list);
                assert_eq!(conflicts.len(), 0);
            }

            #[test]
            fn test_conflicting_steps() {
                let steps_list = vec![
                    Step::mock(
                        "test-conflict",
                        1,
                        2,
                        StepStatus::ReadyToSchedule,
                        vec![
                            Slot::mock(Duration::hours(1), 2023, 5, 1, 12, 0),
                            Slot::mock(Duration::hours(1), 2023, 5, 1, 13, 0),
                        ],
                        None,
                    ),
                    Step::mock(
                        "test-conflict-2",
                        1,
                        2,
                        StepStatus::ReadyToSchedule,
                        vec![
                            Slot::mock(Duration::hours(1), 2023, 5, 1, 15, 0),
                            Slot::mock(Duration::hours(1), 2023, 5, 1, 16, 0),
                        ],
                        None,
                    ),
                ];

                let step_to_search = Step::mock(
                    "test",
                    1,
                    5,
                    StepStatus::ReadyToSchedule,
                    vec![
                        Slot::mock(Duration::hours(1), 2023, 5, 1, 12, 0),
                        Slot::mock(Duration::hours(1), 2023, 5, 1, 13, 0),
                        Slot::mock(Duration::hours(1), 2023, 5, 1, 14, 0),
                        Slot::mock(Duration::hours(1), 2023, 5, 1, 15, 0),
                        Slot::mock(Duration::hours(1), 2023, 5, 1, 16, 0),
                    ],
                    None,
                );

                let conflicts = step_to_search.get_conflicts_in_steps(&steps_list);

                // Will result into 5 slots with 1 hour block
                // only the 14h block will have no conflicts, both other slots should have exactly 1 conflict

                let expected: Vec<SlotConflict> = vec![
                    SlotConflict {
                        slot: Slot::mock(Duration::hours(1), 2023, 5, 1, 16, 0),
                        num_conflicts: 1,
                    },
                    SlotConflict {
                        slot: Slot::mock(Duration::hours(1), 2023, 5, 1, 15, 0),
                        num_conflicts: 1,
                    },
                    SlotConflict {
                        slot: Slot::mock(Duration::hours(1), 2023, 5, 1, 13, 0),
                        num_conflicts: 1,
                    },
                    SlotConflict {
                        slot: Slot::mock(Duration::hours(1), 2023, 5, 1, 12, 0),
                        num_conflicts: 1,
                    },
                    SlotConflict {
                        slot: Slot::mock(Duration::hours(1), 2023, 5, 1, 14, 0),
                        num_conflicts: 0,
                    },
                ];

                assert_eq!(conflicts, expected);
            }
        }
    }
}
