use crate::models::{
    slot::{Slot, SlotConflict},
    task::{Task, TaskStatus},
};

/// Find best slots for tasks by splitting, finding conflicts and return list of slots which can be scheduled
pub(crate) fn find_best_slots(tasks: &Vec<Task>) -> Option<Vec<Slot>> {
    // TODO 2023-05-25  \ Avoid spliting slots which causing wrong scheduling
    // Issued while debugging test case bug_215

    if tasks.is_empty() {
        return None;
    }

    let mut slot_conflicts: Vec<SlotConflict> = vec![];
    let task = &tasks[0];
    // TODO 2023-06-04   | Coninue Dev use function Task::get_conflicts_in_tasks(...)

    /*
    TODO 2023-06-04  \
    - Isolate checking conflicts in a seperate function
    - Check task duration and split based on that list of
    potential slots as below example:
        - Consider below task:
            ```
            Task{
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

    for slot in task.slots.iter() {
        for hour_slot in slot.divide_into_1h_slots() {
            let conflicts = hour_slot.get_conflicts_in_tasks(tasks);

            slot_conflicts.push(SlotConflict {
                slot: hour_slot,
                num_conflicts: conflicts.num_conflicts,
            });
        }
    }
    slot_conflicts.sort_by(|a, b| b.slot.start.partial_cmp(&a.slot.start).unwrap());

    slot_conflicts.sort_by(|a, b| b.num_conflicts.partial_cmp(&a.num_conflicts).unwrap());

    let mut result = vec![];
    for _dur in 0..task.duration {
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

    /// Get conflicts of a slot in list of Tasks
    /// - NOTE: function check conflicts for only tasks with status TaskStatus::ReadyToSchedule
    fn get_conflicts_in_tasks(&self, slots_list: &[Task]) -> SlotConflict {
        let mut count: usize = 0;

        slots_list
            .iter()
            .filter(|task| task.status == TaskStatus::ReadyToSchedule)
            .for_each(|task| {
                let slot_conflict = self.get_conflicts_in_slots(task.slots.as_slice());
                count += slot_conflict.num_conflicts;
            });

        SlotConflict {
            slot: *self,
            num_conflicts: count,
        }
    }
}

impl Task {
    /// Get conflicts of a task slots in list of Tasks
    /// - NOTE: function check conflicts for only tasks with status TaskStatus::ReadyToSchedule
    fn get_conflicts_in_tasks(&self, slots_list: &[Task]) -> Vec<SlotConflict> {
        let mut conflicts_list: Vec<SlotConflict> = vec![];

        if self.status != TaskStatus::ReadyToSchedule {
            return conflicts_list;
        }

        self.slots.iter().for_each(|slot| {
            let hour_slot = slot.divide_into_1h_slots();
            hour_slot.iter().for_each(|hour_slot| {
                let slot_conflict = hour_slot.get_conflicts_in_tasks(slots_list);
                conflicts_list.push(slot_conflict);
            });
        });

        conflicts_list
    }
}

#[cfg(test)]
mod tests {
    use chrono::Duration;

    use super::*;

    #[test]
    fn test_empty_tasks() {
        let tasks_to_place = Vec::new();
        assert_eq!(find_best_slots(&tasks_to_place), None);
    }

    /// Test single task
    /// Expected:
    /// ```
    /// Some([Slot {
    ///     start: 2023-05-01 00
    ///     end: 2023-05-01 01 }])
    /// ```
    #[test]
    fn test_single_task() {
        let task = Task::mock(
            "test",
            1,
            168,
            TaskStatus::ReadyToSchedule,
            vec![Slot::mock(Duration::days(6), 2023, 05, 01, 0, 0)],
        );

        let expected = Some(vec![Slot::mock(Duration::hours(1), 2023, 05, 01, 0, 0)]);
        dbg!(&expected);

        let result = find_best_slots(&vec![task]);
        dbg!(&result);

        assert_eq!(result, expected);
    }

    #[test]
    #[ignore]
    fn test_sleep() {
        let task = Task::mock(
            "test",
            8,
            19,
            TaskStatus::ReadyToSchedule,
            vec![Slot::mock(Duration::days(6), 2023, 05, 01, 0, 0)],
        );

        let expected = Some(vec![Slot::mock(Duration::hours(8), 2023, 05, 01, 0, 0)]);
        dbg!(&task, &expected);

        let result = find_best_slots(&vec![task]);
        dbg!(&result);

        assert_eq!(result, expected);
    }

    #[test]
    #[ignore]
    fn test_multiple_tasks() {
        // todo!("not implemented");

        // let tasks_to_place = vec![
        //     Task {
        //         id: 1,
        //         status: TaskStatus::ReadyToSchedule,
        //         duration: 2,
        //         slots: vec![Slot {
        //             start: NaiveDate::from_ymd(2023, 5, 25).and_hms(10, 0, 0),
        //             end: NaiveDate::from_ymd(2023, 5, 25).and_hms(12, 0, 0),
        //         }],
        //     },
        //     Task {
        //         id: 2,
        //         status: TaskStatus::ReadyToSchedule,
        //         duration: 1,
        //         slots: vec![Slot {
        //             start: NaiveDate::from_ymd(2023, 5, 25).and_hms(11, 0, 0),
        //             end: NaiveDate::from_ymd(2023, 5, 25).and_hms(12, 0, 0),
        //         }],
        //     },
        //     Task {
        //         id: 3,
        //         status: TaskStatus::ReadyToSchedule,
        //         duration: 1,
        //         slots: vec![
        //             Slot {
        //                 start: NaiveDate::from_ymd(2023, 5, 25).and_hms(10, 0, 0),
        //                 end: NaiveDate::from_ymd(2023, 5, 25).and_hms(11, 0, 0),
        //             },
        //             Slot {
        //                 start: NaiveDate::from_ymd(2023, 5, 25).and_hms(11, 0, 0),
        //                 end: NaiveDate::from_ymd(2023, 5, 25).and_hms(12, 0, 0),
        //             },
        //         ],
        //     },
        // ];
        // let expected = Some(vec![
        //     Slot {
        //         start: NaiveDate::from_ymd(2023, 5, 25).and_hms(11, 0, 0),
        //         end: NaiveDate::from_ymd(2023, 5, 25).and_hms(12, 0, 0),
        //     },
        //     Slot {
        //         start: NaiveDate::from_ymd(2023, 5, 25).and_hms(10, 0, 0),
        //         end: NaiveDate::from_ymd(2023, 5, 25).and_hms(11, 0, 0),
        //     },
        // ]);
        // assert_eq!(find_best_slots(&tasks_to_place), expected);
    }

    mod conflicts {
        use chrono::Duration;

        use crate::models::{
            slot::Slot,
            task::{Task, TaskStatus},
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
            fn test_no_conflicts_in_tasks() {
                let tasks_list = vec![
                    Task::mock(
                        "task 1",
                        10,
                        0,
                        TaskStatus::ReadyToSchedule,
                        vec![
                            Slot::mock(Duration::hours(5), 2023, 6, 1, 0, 0),
                            Slot::mock(Duration::hours(10), 2023, 6, 1, 9, 0),
                        ],
                    ),
                    Task::mock(
                        "task 2",
                        2,
                        0,
                        TaskStatus::ReadyToSchedule,
                        vec![
                            Slot::mock(Duration::hours(5), 2023, 6, 5, 0, 0),
                            Slot::mock(Duration::hours(10), 2023, 6, 6, 9, 0),
                        ],
                    ),
                ];

                let slot_to_search = Slot::mock(Duration::hours(2), 2023, 5, 1, 6, 0);

                let conflicts = slot_to_search.get_conflicts_in_tasks(&tasks_list);

                assert_eq!(conflicts.slot, slot_to_search);
                assert_eq!(conflicts.num_conflicts, 0);
            }

            /// Testing many conflicts for a slot within list of
            /// tasks but, one of them with status Scheduled
            /// ```
            /// slot to search:
            ///     2023-06-01 01 to 03
            ///
            /// Taks:
            ///     Task 1 ReadyToSchedule
            ///         2023-05-30 to 2023-06-04 (conflicts)
            ///         2023-06-01 00 to 05 (conflicts)
            ///         2023-06-02 00 to 08
            ///         2023-06-03 00 to 08
            ///     Task 2 ReadyToSchedule
            ///         2023-05-30 to 2023-06-04 (conflicts)
            ///         2023-06-01 00 to 05 (conflicts)
            ///         2023-06-02 00 to 08
            ///         2023-06-03 00 to 08
            ///     Task 3 Scheduled
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
            fn test_conflicts_in_tasks() {
                let slots_list = vec![
                    Slot::mock(Duration::days(5), 2023, 5, 30, 0, 0),
                    Slot::mock(Duration::hours(5), 2023, 6, 1, 0, 0),
                    Slot::mock(Duration::hours(8), 2023, 6, 2, 0, 0),
                    Slot::mock(Duration::hours(8), 2023, 6, 3, 0, 0),
                ];

                let tasks_list = vec![
                    Task::mock(
                        "task 1",
                        10,
                        0,
                        TaskStatus::ReadyToSchedule,
                        slots_list.clone(),
                    ),
                    Task::mock(
                        "task 2",
                        2,
                        0,
                        TaskStatus::ReadyToSchedule,
                        slots_list.clone(),
                    ),
                    Task::mock("task 2", 2, 0, TaskStatus::Scheduled, slots_list),
                ];

                let slot_to_search = Slot::mock(Duration::hours(2), 2023, 6, 1, 1, 0);

                let conflicts = slot_to_search.get_conflicts_in_tasks(&tasks_list);

                assert_eq!(conflicts.slot, slot_to_search);
                assert_eq!(conflicts.num_conflicts, 4);
            }
        }

        mod task {
            use super::*;

            #[test]
            fn test_no_conflicts_in_tasks() {
                let tasks_list = vec![
                    Task::mock(
                        "task 1",
                        10,
                        0,
                        TaskStatus::ReadyToSchedule,
                        vec![
                            Slot::mock(Duration::hours(5), 2023, 6, 1, 0, 0),
                            Slot::mock(Duration::hours(10), 2023, 6, 1, 9, 0),
                        ],
                    ),
                    Task::mock(
                        "task 2",
                        2,
                        0,
                        TaskStatus::ReadyToSchedule,
                        vec![
                            Slot::mock(Duration::hours(5), 2023, 6, 5, 0, 0),
                            Slot::mock(Duration::hours(10), 2023, 6, 6, 9, 0),
                        ],
                    ),
                ];

                let task_to_search = Task::mock(
                    "test task",
                    2,
                    0,
                    TaskStatus::ReadyToSchedule,
                    vec![
                        Slot::mock(Duration::hours(2), 2023, 5, 1, 6, 0),
                        Slot::mock(Duration::hours(2), 2023, 6, 6, 0, 0),
                    ],
                );

                let conflicts = task_to_search.get_conflicts_in_tasks(&tasks_list);

                // Will result into 4 slots with 1 hour block

                // Slot 1 ( 2023-05-01 06 to 07 ) in task_to_search
                assert_eq!(
                    conflicts[0].slot,
                    Slot::mock(Duration::hours(1), 2023, 5, 1, 6, 0)
                );
                assert_eq!(conflicts[0].num_conflicts, 0);

                // Slot 2 ( 2023-05-01 07 to 08 ) in task_to_search
                assert_eq!(
                    conflicts[1].slot,
                    Slot::mock(Duration::hours(1), 2023, 5, 1, 7, 0)
                );
                assert_eq!(conflicts[1].num_conflicts, 0);

                // Slot 3 ( 2023-06-06 00 to 01 ) in task_to_search
                assert_eq!(
                    conflicts[2].slot,
                    Slot::mock(Duration::hours(1), 2023, 6, 6, 0, 0)
                );
                assert_eq!(conflicts[2].num_conflicts, 0);

                // Slot 4 ( 2023-06-06 01 to 02 ) in task_to_search
                assert_eq!(
                    conflicts[3].slot,
                    Slot::mock(Duration::hours(1), 2023, 6, 6, 1, 0)
                );
                assert_eq!(conflicts[3].num_conflicts, 0);
            }

            /// Testing many conflicts for a task slots within list of
            /// tasks but, one of them with status Scheduled
            /// ```
            /// task to search:
            ///         ReadyToSchedule
            ///         2023-06-01 01 to 03
            ///         2023-06-02 08 to 09
            ///
            /// Taks:
            ///     Task 1 ReadyToSchedule
            ///         2023-05-30 to 2023-06-04 (conflicts with both slots in task1)
            ///         2023-06-01 00 to 05 (conflicts with task1.slot1)
            ///         2023-06-02 00 to 08
            ///         2023-06-03 00 to 08
            ///     Task 2 ReadyToSchedule
            ///         2023-05-30 to 2023-06-04 (conflicts with both slots in task1)
            ///         2023-06-01 00 to 05 (conflicts with task1.slot1)
            ///         2023-06-02 00 to 08
            ///         2023-06-03 00 to 08
            ///     Task 3 Scheduled
            ///         2023-05-30 to 2023-06-04 (will not conflicts)
            ///         2023-06-01 00 to 05 (will not conflicts)
            ///         2023-06-02 00 to 08
            ///         2023-06-03 00 to 08
            ///     
            ///
            /// Expected:
            ///     SlotConflict:
            ///         - Slot: 2023-06-01 01 to 02 (task1.slot1)
            ///         - Conflicts: 4
            ///     SlotConflict:
            ///         - Slot: 2023-06-01 02 to 03 (task1.slot1)
            ///         - Conflicts: 4
            ///     SlotConflict:
            ///         - Slot: 2023-06-02 08 to 09 (task1.slot2)
            ///         - Conflicts: 2
            /// ```
            #[test]
            fn test_conflicts_in_tasks() {
                let slots_list = vec![
                    Slot::mock(Duration::days(5), 2023, 5, 30, 0, 0),
                    Slot::mock(Duration::hours(5), 2023, 6, 1, 0, 0),
                    Slot::mock(Duration::hours(8), 2023, 6, 2, 0, 0),
                    Slot::mock(Duration::hours(8), 2023, 6, 3, 0, 0),
                ];

                let tasks_list = vec![
                    Task::mock(
                        "task 1",
                        10,
                        0,
                        TaskStatus::ReadyToSchedule,
                        slots_list.clone(),
                    ),
                    Task::mock(
                        "task 2",
                        2,
                        0,
                        TaskStatus::ReadyToSchedule,
                        slots_list.clone(),
                    ),
                    Task::mock("task 2", 2, 0, TaskStatus::Scheduled, slots_list),
                ];

                let task_to_search = Task::mock(
                    "test task 1",
                    2,
                    0,
                    TaskStatus::ReadyToSchedule,
                    vec![
                        Slot::mock(Duration::hours(2), 2023, 6, 1, 1, 0),
                        Slot::mock(Duration::hours(2), 2023, 6, 2, 8, 0),
                    ],
                );

                let conflicts = task_to_search.get_conflicts_in_tasks(&tasks_list);

                // Will result into 4 slots with 1 hour block

                // Slot 1 ( 2023-06-01 01 to 02 ) in task_to_search
                assert_eq!(
                    conflicts[0].slot,
                    Slot::mock(Duration::hours(1), 2023, 6, 1, 1, 0)
                );
                assert_eq!(conflicts[0].num_conflicts, 4);

                // Slot 2 ( 2023-06-01 02 to 03 ) in task_to_search
                assert_eq!(
                    conflicts[1].slot,
                    Slot::mock(Duration::hours(1), 2023, 6, 1, 2, 0)
                );
                assert_eq!(conflicts[1].num_conflicts, 4);

                // Slot 3 ( 2023-06-02 08 to 09 ) in task_to_search
                assert_eq!(
                    conflicts[2].slot,
                    Slot::mock(Duration::hours(1), 2023, 6, 2, 8, 0)
                );
                assert_eq!(conflicts[2].num_conflicts, 2);

                // Slot 4 ( 2023-06-02 08 to 09 ) in task_to_search
                assert_eq!(
                    conflicts[3].slot,
                    Slot::mock(Duration::hours(1), 2023, 6, 2, 9, 0)
                );
                assert_eq!(conflicts[3].num_conflicts, 2);
            }

            /// Testing conflicts for a task with Scheduled status
            /// within list of tasks
            /// ```
            /// task to search:
            ///         Scheduled
            ///         2023-06-01 01 to 03
            ///         2023-06-02 08 to 09
            ///
            /// Taks:
            ///     Task 1 ReadyToSchedule
            ///         2023-05-30 to 2023-06-04 (will not conflicts)
            ///         2023-06-01 00 to 05 (will not conflicts)
            ///         2023-06-02 00 to 08
            ///         2023-06-03 00 to 08
            ///     Task 2 ReadyToSchedule
            ///         2023-05-30 to 2023-06-04 (will not conflicts)
            ///         2023-06-01 00 to 05 (will not conflicts)
            ///         2023-06-02 00 to 08
            ///         2023-06-03 00 to 08
            ///     Task 3 Scheduled
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
            fn test_task_is_invalid() {
                let slots_list = vec![
                    Slot::mock(Duration::days(5), 2023, 5, 30, 0, 0),
                    Slot::mock(Duration::hours(5), 2023, 6, 1, 0, 0),
                    Slot::mock(Duration::hours(8), 2023, 6, 2, 0, 0),
                    Slot::mock(Duration::hours(8), 2023, 6, 3, 0, 0),
                ];

                let tasks_list = vec![
                    Task::mock(
                        "task 1",
                        10,
                        0,
                        TaskStatus::ReadyToSchedule,
                        slots_list.clone(),
                    ),
                    Task::mock(
                        "task 2",
                        2,
                        0,
                        TaskStatus::ReadyToSchedule,
                        slots_list.clone(),
                    ),
                    Task::mock("task 2", 2, 0, TaskStatus::Scheduled, slots_list),
                ];

                let task_to_search = Task::mock(
                    "test task 1",
                    2,
                    0,
                    TaskStatus::Scheduled,
                    vec![
                        Slot::mock(Duration::hours(2), 2023, 6, 1, 1, 0),
                        Slot::mock(Duration::hours(2), 2023, 6, 2, 8, 0),
                    ],
                );

                let conflicts = task_to_search.get_conflicts_in_tasks(&tasks_list);
                assert_eq!(conflicts.len(), 0);
            }
        }
    }
}
