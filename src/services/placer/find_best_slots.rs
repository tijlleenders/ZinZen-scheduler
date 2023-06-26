use chrono::Duration;

use crate::models::{
    slot::{Slot, SlotConflict},
    step::{Step, StepStatus},
};

/// Find best slots for steps by splitting, finding conflicts and return list of slots which can be scheduled
pub(crate) fn find_best_slots(steps: &Vec<Step>) -> Option<Vec<Slot>> {
    // TODO 2023-05-25  \ Avoid spliting slots which causing wrong scheduling
    // Issued while debugging test case bug_215

    if steps.is_empty() {
        return None;
    }

    /*
    TODO 2023-06-04  \
    - Isolate checking conflicts in a seperate function
    - Check step duration and split based on that list of
    potential slots as below example:
        - Consider below step:
            ```
            Step{
                title: sleep
                duration: 8
                slots: 22-08 (10 hours)
            }

            So 3 slots will be generated:
                Slot: 22-06
                Slot: 23-07
                Slot: 22-08

            ```
    */

    let step = &steps[0];
    let mut slot_conflicts = step.get_conflicts_in_steps(steps);

    let mut result = vec![];
    for _dur in 0..step.duration {
        match slot_conflicts.pop() {
            Some(s) => result.push(s.slot),
            None => break,
        }
    }

    Some(result)
}

impl Slot {
    /// Get conflicts of a slot in list of slots
    fn get_conflicts_in_slots(&self, slots_list: &[Slot]) -> SlotConflict {
        let mut count: usize = 0;

        slots_list.iter().for_each(|slot| {
            if self.is_intersect_with_slot(slot) {
                count += 1;
            }
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
    ///     ```
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
        dbg!(&self);
        let slot_duration = self.duration_as_hours();
        let mut schedulable_slots = vec![];

        if slot_duration < duration {
            return schedulable_slots;
        } else if slot_duration == duration {
            return vec![*self];
        } else {
            let flexibility = slot_duration - duration + 1;

            let mut start_time = self.start;
            // used to make sure that endtime for each new slot not exceed self.end
            let mut end_time = start_time + Duration::hours(duration as i64);

            for _ in 0..flexibility {
                if end_time <= self.end {
                    let new_slot = Slot {
                        start: start_time,
                        end: end_time,
                    };

                    schedulable_slots.push(new_slot);

                    start_time = start_time + Duration::hours(1);
                    end_time = start_time + Duration::hours(duration as i64);
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
    ///     - Splitting slots into schedulable slots based on slot's timing and step's duration
    fn get_conflicts_in_steps(&self, slots_list: &[Step]) -> Vec<SlotConflict> {
        dbg!(&self, &slots_list);
        let mut conflicts_list: Vec<SlotConflict> = vec![];

        if self.status != StepStatus::ReadyToSchedule {
            return conflicts_list;
        }

        self.slots.iter().for_each(|slot| {
            let schedulable_slots = slot.generate_schedulable_slots(self.duration);
            dbg!(&schedulable_slots);
            schedulable_slots.iter().for_each(|hour_slot| {
                let slot_conflict = hour_slot.get_conflicts_in_steps(slots_list);
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
        assert_eq!(find_best_slots(&steps_to_place), None);
    }

    /// Test single step
    /// Expected:
    /// ```
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
            vec![Slot::mock(Duration::days(6), 2023, 05, 01, 0, 0)],
            None,
        );

        let expected = Some(vec![Slot::mock(Duration::hours(1), 2023, 05, 01, 0, 0)]);

        let result = find_best_slots(&vec![step]);

        assert_eq!(result, expected);
    }

    #[test]
    #[ignore]
    fn test_sleep() {
        let step = Step::mock(
            "test",
            8,
            19,
            StepStatus::ReadyToSchedule,
            vec![Slot::mock(Duration::days(6), 2023, 05, 01, 0, 0)],
            None,
        );

        let expected = Some(vec![Slot::mock(Duration::hours(8), 2023, 05, 01, 0, 0)]);
        dbg!(&step, &expected);

        let result = find_best_slots(&vec![step]);
        dbg!(&result);

        assert_eq!(result, expected);
    }

    #[test]
    #[ignore]
    fn test_multiple_steps() {
        todo!("not implemented");
    }

    /// - Example:
    ///     ```
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
        use chrono::Duration;

        use crate::models::{
            slot::Slot,
            step::{Step, StepStatus},
        };

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
            /// ```
            /// slot to search:
            ///     2023-06-01 01 to 03
            ///
            /// list of slots to search in:
            ///     2023-05-30 to 2023-06-04
            ///     2023-06-01 00 to 05
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
                assert_eq!(conflicts.num_conflicts, 2);
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
            /// ```
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
                assert_eq!(conflicts.num_conflicts, 4);
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

            /// Testing many conflicts for a step slots within list of
            /// steps but, one of them with status Scheduled
            /// ```
            /// step to search:
            ///         ReadyToSchedule
            ///         2023-06-01 01 to 03
            ///         2023-06-02 08 to 09
            ///
            /// Taks:
            ///     Step 1 ReadyToSchedule
            ///         2023-05-30 to 2023-06-04 (conflicts with both slots in step1)
            ///         2023-06-01 00 to 05 (conflicts with step1.slot1)
            ///         2023-06-02 00 to 08
            ///         2023-06-03 00 to 08
            ///     Step 2 ReadyToSchedule
            ///         2023-05-30 to 2023-06-04 (conflicts with both slots in step1)
            ///         2023-06-01 00 to 05 (conflicts with step1.slot1)
            ///         2023-06-02 00 to 08
            ///         2023-06-03 00 to 08
            ///     Step 3 Scheduled
            ///         2023-05-30 to 2023-06-04 (will not conflicts)
            ///         2023-06-01 00 to 05 (will not conflicts)
            ///         2023-06-02 00 to 08
            ///         2023-06-03 00 to 08
            ///     
            ///
            /// Expected 4 slots with conflicts:
            ///     SlotConflict:
            ///         - Slot: 2023-06-01 02 to 03 (step1.slot1)
            ///         - Conflicts: 4
            ///     SlotConflict:
            ///         - Slot: 2023-06-01 01 to 02 (step1.slot1)
            ///         - Conflicts: 4
            ///     SlotConflict:
            ///         - Slot: 2023-06-02 09 to 10 (step1.slot1)
            ///         - Conflicts: 2
            ///     SlotConflict:
            ///         - Slot: 2023-06-02 08 to 09 (step1.slot2)
            ///         - Conflicts: 2
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

                let step_to_search = Step::mock(
                    "test step 1",
                    1,
                    0,
                    StepStatus::ReadyToSchedule,
                    vec![
                        Slot::mock(Duration::hours(2), 2023, 6, 1, 1, 0),
                        Slot::mock(Duration::hours(2), 2023, 6, 2, 8, 0),
                    ],
                    None,
                );

                let conflicts = step_to_search.get_conflicts_in_steps(&steps_list);

                // Will result into 4 slots with 1 hour block
                let expected: Vec<SlotConflict> = vec![
                    SlotConflict {
                        slot: Slot::mock(Duration::hours(1), 2023, 6, 1, 2, 0),
                        num_conflicts: 4,
                    },
                    SlotConflict {
                        slot: Slot::mock(Duration::hours(1), 2023, 6, 1, 1, 0),
                        num_conflicts: 4,
                    },
                    SlotConflict {
                        slot: Slot::mock(Duration::hours(1), 2023, 6, 2, 9, 0),
                        num_conflicts: 2,
                    },
                    SlotConflict {
                        slot: Slot::mock(Duration::hours(1), 2023, 6, 2, 8, 0),
                        num_conflicts: 2,
                    },
                ];

                assert_eq!(conflicts, expected);
            }

            /// Testing conflicts for a step with Scheduled status
            /// within list of steps
            /// ```
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
        }
    }
}
