use crate::{
    errors::Error,
    models::{
        goal::Goal,
        output::Task,
        slot::Slot,
        step::{NewStep, Step, StepStatus},
        timeline::{Timeline, TimelineSlotsType},
    },
};
use chrono::{Days, Duration, Timelike};
use std::{collections::BTreeSet, ops::Add};

impl Timeline {
    /// Get splitted timeline into slots with 1 day interval
    pub fn split_into_days(&self) -> Timeline {
        // TODO 2023-04-25 | test scenario:
        //  - when slots in timeline are not full days!!!! Is the split
        // will return full day or will respect the tha slot not full day!!
        let mut new_slots: TimelineSlotsType = BTreeSet::new();
        for slot in self.slots.iter() {
            new_slots.extend(slot.split_into_days());
        }
        Timeline { slots: new_slots }
    }
}

impl Step {
    /// When a step duration exceeded threshold, it will be splitted
    /// into 1 hour steps
    pub fn apply_duration_threshold(&self, counter: &mut usize) -> Vec<Step> {
        let threshold: usize = 8;
        let mut new_step = self.clone();

        if new_step.duration > 0 && new_step.duration <= threshold {
            vec![new_step]
        } else {
            let mut steps: Vec<Step> = Vec::new();
            steps.extend(new_step.split(counter).unwrap());
            steps
        }
    }

    /// Split a Step into list of Steps based on given Step duration.
    /// - Note: This function will change below in the resulted steps:
    ///     - Step.status = StepStatus::ReadyToSchedule
    ///     - Step.tags = inherited from parent
    pub fn split(&mut self, counter: &mut usize) -> Result<Vec<Step>, Error> {
        // TODO 2023-06-22: Debug notes: This function not clone step.start and step.deadline
        if self.duration == 1 {
            return Err(Error::CannotSplit);
        }
        let mut steps = Vec::new();
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
        let new_step = NewStep {
            step_id: *counter,
            title: self.title.clone(),
            duration: 1,
            goal,
            timeline,
            status: StepStatus::Uninitialized,
            timeframe: None,
        };

        for _ in 0..self.duration {
            let mut step = Step::new(new_step.clone());
            step.id = *counter;
            step.status = StepStatus::ReadyToSchedule;
            step.tags = self.tags.clone();
            *counter += 1;
            steps.push(step);
        }
        Ok(steps)
    }

    /// Split a Step slots into list of slots based on given threshold.
    pub fn split_into_custom_hours(&self, threshold: usize) -> Vec<Slot> {
        let given_step = self.clone();
        let mut result = vec![];

        given_step.slots.iter().for_each(|slot| {
            result.extend(slot.split_into_custom_hours(threshold));
        });

        result
    }
}

impl Slot {
    /// Divide a Slot into list of slots with 1 hour interval
    /// If you pass a Slot for 5 hours, then it will splitted
    ///  into 5 slots with 1 hour interval:
    /// ```markdown
    /// Param:
    ///     Slot [ 2022-01-01 00:00:00 - 2022-01-01 05:00:00 ]
    ///     Duration: 5 hours
    ///
    /// Returns:
    ///     Slot [ 2022-01-01 00:00:00 - 2022-01-01 01:00:00 ]
    ///     Slot [ 2022-01-01 01:00:00 - 2022-01-01 02:00:00 ]
    ///     Slot [ 2022-01-01 02:00:00 - 2022-01-01 03:00:00 ]
    ///     Slot [ 2022-01-01 03:00:00 - 2022-01-01 04:00:00 ]
    ///     Slot [ 2022-01-01 04:00:00 - 2022-01-01 05:00:00 ]
    ///
    /// ```
    pub fn split_into_1h_slots(&self) -> Vec<Slot> {
        let mut result = vec![];
        let duration = self.span();

        for hour in 0..duration {
            result.push(Slot {
                start: self.start.add(Duration::hours(hour as i64)),
                end: self.start.add(Duration::hours((hour + 1) as i64)),
            });
        }
        result
    }

    /// Split a Slot into list of slots based on given threshold.
    pub fn split_into_custom_hours(&self, threshold: usize) -> Vec<Slot> {
        let given_slot = *self;
        let duration = given_slot.span();
        if threshold == 0 || threshold > duration {
            return vec![given_slot];
        }

        let mut result = vec![];

        for hour in (0..duration).step_by(threshold) {
            if hour == duration - 1 {
                result.push(Slot {
                    start: self.start.add(Duration::hours(hour as i64)),
                    end: self.end,
                });
                break;
            }
            result.push(Slot {
                start: self.start.add(Duration::hours(hour as i64)),
                end: self.start.add(Duration::hours((hour + threshold) as i64)),
            });
        }

        result
    }

    /// Divide a Slot into list of slots with 1 day interval
    /// If you pass a Slot for a week, then it will splitted
    ///  into 7 slots for each day of the week:
    /// ```markdown
    /// Param:
    ///     Slot [ 2022-01-01 00:00:00 - 2022-01-08 00:00:00 ]
    ///
    /// Returns:
    ///     Slot [ 2022-01-01 00:00:00 - 2022-01-02 00:00:00 ]
    ///     Slot [ 2022-01-02 00:00:00 - 2022-01-03 00:00:00 ]
    ///     Slot [ 2022-01-03 00:00:00 - 2022-01-04 00:00:00 ]
    ///     Slot [ 2022-01-04 00:00:00 - 2022-01-05 00:00:00 ]
    ///     Slot [ 2022-01-05 00:00:00 - 2022-01-06 00:00:00 ]
    ///     Slot [ 2022-01-06 00:00:00 - 2022-01-07 00:00:00 ]
    ///     Slot [ 2022-01-07 00:00:00 - 2022-01-08 00:00:00 ]
    /// ```
    pub fn split_into_days(&self) -> Vec<Slot> {
        let mut result = vec![];
        let mut start_slider = self.start;

        while start_slider.lt(&self.end) {
            if start_slider.date().eq(&self.end.date()) {
                result.push(Slot {
                    start: start_slider,
                    end: self.end,
                });

                start_slider = start_slider
                    .with_hour(0)
                    .unwrap()
                    .checked_add_days(Days::new(1))
                    .unwrap();
                continue;
            } else {
                result.push(Slot {
                    start: start_slider,
                    end: start_slider
                        .with_hour(0)
                        .unwrap()
                        .checked_add_days(Days::new(1))
                        .unwrap(),
                });

                start_slider = start_slider
                    .with_hour(0)
                    .unwrap()
                    .checked_add_days(Days::new(1))
                    .unwrap();
            }
        }
        result
    }
}

/// Split list of slots into a list of slots with 1 hour interval
pub fn split_slots_into_1h_slots(slots: Vec<Slot>) -> Vec<Slot> {
    let mut all_slots: Vec<Slot> = vec![];
    for slot in slots.iter() {
        let mut slots_1h = slot.split_into_1h_slots();
        all_slots.append(slots_1h.as_mut());
    }

    all_slots
}

/// Splitting list of crossed tasks.
/// - If a Task starts in one day and ends in the next day, it should be splitted into two Tasks.
/// - Example
/// ```markdown
///     Input:
///         Task `Sleep` from 22 - 06 (next day)
///
///     Output:
///         Task `Sleep` 22 - 00 (current day)
///         Task `Sleep` 00 - 06 (next day)
/// ```
pub fn split_crossed_tasks(tasks: &mut Vec<Task>) {
    /*
    TODO 2023-06-04  | Debug note | case bug_215
    - For param "tasks", it contains wrong duration for steps "hurdle" and "sleep".
    - Attention to function "is_cross_day" which comparison need to be enhanced. Check task.title:"hurdle"
    - Attention to code line "step2.duration -= step.duration;" which seems is not accurate and also affected by function "is_cross_day"
    */

    let mut new_tasks = vec![];
    for task in tasks.iter_mut() {
        if is_cross_day(task) {
            let mut task2 = task.clone();
            task.deadline = task.deadline.with_hour(0).unwrap();
            task2.start = task.deadline.with_hour(0).unwrap();
            task.duration = Slot {
                start: task.start,
                end: task.deadline,
            }
            .span();
            task2.duration -= task.duration;

            new_tasks.push(task.clone());
            if task2.duration > 0 {
                new_tasks.push(task2);
            }
        } else {
            new_tasks.push(task.clone());
        }
    }

    tasks.clear();
    tasks.extend(new_tasks);
}

/// Is a task crossing a day so day of start date is different from day of end date
fn is_cross_day(task: &Task) -> bool {
    let start = task.start.date();
    let end = task.deadline.date();

    // start < end
    start < end || (start == end && task.start.time() > task.deadline.time())
}

#[cfg(test)]
mod tests {
    mod slot {
        use crate::models::slot::Slot;
        use chrono::Duration;

        #[test]
        fn test_split_into_custom_hours() {
            let slot = Slot::mock(Duration::hours(5), 2023, 6, 1, 0, 0);

            // Test when threshold > slot.duration which return same slot
            let result = slot.split_into_custom_hours(10);
            let expected = slot;
            assert_eq!(result.len(), 1);
            assert_eq!(result[0], expected);

            // Test when threshold=0 which return same slot
            let result = slot.split_into_custom_hours(0);
            assert_eq!(result.len(), 1);
            assert_eq!(result[0], expected);

            // Test when threshold < slot.duration whcih return splitted slots based on threshold
            let result = slot.split_into_custom_hours(2);
            let expected = vec![
                Slot::mock(Duration::hours(2), 2023, 6, 1, 0, 0),
                Slot::mock(Duration::hours(2), 2023, 6, 1, 2, 0),
                Slot::mock(Duration::hours(1), 2023, 6, 1, 4, 0),
            ];
            assert_eq!(result.len(), 3);
            assert_eq!(result, expected);
        }
    }
    mod step {
        use chrono::Duration;

        use crate::models::{
            slot::Slot,
            step::{Step, StepStatus},
        };

        mod apply_duration_threshold {
            use chrono::Duration;

            use crate::models::{
                goal::Goal,
                slot::Slot,
                step::{NewStep, Step, StepStatus},
                timeline::Timeline,
            };

            #[test]
            fn test_duration_less_8_hrs() {
                let duration: usize = 7;
                let timeframe = Slot::mock(Duration::days(5), 2023, 6, 1, 0, 0);

                let new_step = NewStep {
                    step_id: 1,
                    title: "test".to_string(),
                    duration,
                    goal: Goal::mock("1", "test", timeframe),
                    timeline: Timeline::new(),
                    status: StepStatus::ReadyToSchedule,
                    timeframe: Some(timeframe),
                };
                let new_step = Step::new(new_step);

                let generated_steps = new_step.apply_duration_threshold(&mut 1);

                let mut expected_step = Step::mock(
                    "test",
                    7,
                    0,
                    StepStatus::ReadyToSchedule,
                    vec![],
                    Option::from(timeframe),
                );
                expected_step.id = 1;

                assert_eq!(generated_steps, vec![expected_step.clone()]);
            }

            /// Test Step::apply_duration_threshold when goal.min_duration > 8 hours
            /// ```markdown
            /// =========================
            /// Input:
            /// Goal {
            ///    id: "1",
            ///    title: "test",
            ///    min_duration: Some(
            ///        10,
            ///    ),
            ///    max_duration: None,
            ///    budgets: None,
            ///    repeat: None,
            ///    start: Some(
            ///        2023-06-01T00:00:00,
            ///    ),
            ///    deadline: Some(
            ///        2023-06-06T00:00:00,
            ///    ),
            ///    tags: [],
            ///    filters: None,
            ///    children: None,
            ///    after_goals: None,
            ///}
            ///
            /// ===========================
            /// Output:
            /// expected_step = [
            ///    Step {
            ///     id  |   goal_id |   duration    |       slots
            ///     1   |   "1"     |       1       |   2023-06-01 00-06
            ///     2   |   "1"     |       1       |   2023-06-01 00-06
            ///     3   |   "1"     |       1       |   2023-06-01 00-06
            ///     4   |   "1"     |       1       |   2023-06-01 00-06
            ///     5   |   "1"     |       1       |   2023-06-01 00-06
            ///     6   |   "1"     |       1       |   2023-06-01 00-06
            ///     7   |   "1"     |       1       |   2023-06-01 00-06
            ///     8   |   "1"     |       1       |   2023-06-01 00-06
            ///     9   |   "1"     |       1       |   2023-06-01 00-06
            ///     10  |   "1"     |       1       |   2023-06-01 00-06
            /// ]
            ///```
            #[test]
            fn test_duration_more_8_hrs() {
                let duration: usize = 10;
                let timeframe = Slot::mock(Duration::days(5), 2023, 6, 1, 0, 0);
                let timeline = Timeline {
                    slots: vec![timeframe].into_iter().collect(),
                };

                let new_step = NewStep {
                    step_id: 1,
                    title: "test".to_string(),
                    duration,
                    goal: Goal::mock("1", "test", timeframe),
                    timeline,
                    status: StepStatus::ReadyToSchedule,
                    timeframe: None,
                };
                let new_step = Step::new(new_step);
                let generated_steps = new_step.apply_duration_threshold(&mut 1);
                let mut expected_steps = vec![
                    Step::mock(
                        "test",
                        1,
                        0,
                        StepStatus::ReadyToSchedule,
                        vec![timeframe],
                        None,
                    ),
                    Step::mock(
                        "test",
                        1,
                        0,
                        StepStatus::ReadyToSchedule,
                        vec![timeframe],
                        None,
                    ),
                    Step::mock(
                        "test",
                        1,
                        0,
                        StepStatus::ReadyToSchedule,
                        vec![timeframe],
                        None,
                    ),
                    Step::mock(
                        "test",
                        1,
                        0,
                        StepStatus::ReadyToSchedule,
                        vec![timeframe],
                        None,
                    ),
                    Step::mock(
                        "test",
                        1,
                        0,
                        StepStatus::ReadyToSchedule,
                        vec![timeframe],
                        None,
                    ),
                    Step::mock(
                        "test",
                        1,
                        0,
                        StepStatus::ReadyToSchedule,
                        vec![timeframe],
                        None,
                    ),
                    Step::mock(
                        "test",
                        1,
                        0,
                        StepStatus::ReadyToSchedule,
                        vec![timeframe],
                        None,
                    ),
                    Step::mock(
                        "test",
                        1,
                        0,
                        StepStatus::ReadyToSchedule,
                        vec![timeframe],
                        None,
                    ),
                    Step::mock(
                        "test",
                        1,
                        0,
                        StepStatus::ReadyToSchedule,
                        vec![timeframe],
                        None,
                    ),
                    Step::mock(
                        "test",
                        1,
                        0,
                        StepStatus::ReadyToSchedule,
                        vec![timeframe],
                        None,
                    ),
                ];
                expected_steps[0].id = 1;
                expected_steps[1].id = 2;
                expected_steps[2].id = 3;
                expected_steps[3].id = 4;
                expected_steps[4].id = 5;
                expected_steps[5].id = 6;
                expected_steps[6].id = 7;
                expected_steps[7].id = 8;
                expected_steps[8].id = 9;
                expected_steps[9].id = 10;

                assert_eq!(generated_steps, expected_steps);
                assert_eq!(generated_steps.len(), 10);

                assert_eq!(generated_steps[0].id, expected_steps[0].id);
                assert_eq!(generated_steps[1].id, expected_steps[1].id);
                assert_eq!(generated_steps[2].id, expected_steps[2].id);
                assert_eq!(generated_steps[3].id, expected_steps[3].id);
                assert_eq!(generated_steps[4].id, expected_steps[4].id);
                assert_eq!(generated_steps[5].id, expected_steps[5].id);
                assert_eq!(generated_steps[6].id, expected_steps[6].id);
                assert_eq!(generated_steps[7].id, expected_steps[7].id);
                assert_eq!(generated_steps[8].id, expected_steps[8].id);
                assert_eq!(generated_steps[9].id, expected_steps[9].id);

                assert_eq!(generated_steps[0].duration, expected_steps[0].duration);
                assert_eq!(generated_steps[1].duration, expected_steps[1].duration);
                assert_eq!(generated_steps[2].duration, expected_steps[2].duration);
                assert_eq!(generated_steps[3].duration, expected_steps[3].duration);
                assert_eq!(generated_steps[4].duration, expected_steps[4].duration);
                assert_eq!(generated_steps[5].duration, expected_steps[5].duration);
                assert_eq!(generated_steps[6].duration, expected_steps[6].duration);
                assert_eq!(generated_steps[7].duration, expected_steps[7].duration);
                assert_eq!(generated_steps[8].duration, expected_steps[8].duration);
                assert_eq!(generated_steps[9].duration, expected_steps[9].duration);

                assert_eq!(generated_steps[0].status, expected_steps[0].status);
                assert_eq!(generated_steps[1].status, expected_steps[1].status);
                assert_eq!(generated_steps[2].status, expected_steps[2].status);
                assert_eq!(generated_steps[3].status, expected_steps[3].status);
                assert_eq!(generated_steps[4].status, expected_steps[4].status);
                assert_eq!(generated_steps[5].status, expected_steps[5].status);
                assert_eq!(generated_steps[6].status, expected_steps[6].status);
                assert_eq!(generated_steps[7].status, expected_steps[7].status);
                assert_eq!(generated_steps[8].status, expected_steps[8].status);
                assert_eq!(generated_steps[9].status, expected_steps[9].status);
            }
        }

        #[test]
        fn test_split() {
            let duration: usize = 3;
            let mut counter: usize = 1;

            let goal_timeframe = Slot::mock(Duration::days(5), 2023, 6, 1, 0, 0);
            let mut step = Step::mock(
                "test",
                duration,
                0,
                StepStatus::ReadyToSchedule,
                vec![goal_timeframe],
                None,
            );
            let steps = step.split(&mut counter).unwrap();

            let mut expected_steps = vec![
                Step::mock(
                    "test",
                    1,
                    0,
                    StepStatus::ReadyToSchedule,
                    vec![goal_timeframe],
                    None,
                ),
                Step::mock(
                    "test",
                    1,
                    0,
                    StepStatus::ReadyToSchedule,
                    vec![goal_timeframe],
                    None,
                ),
                Step::mock(
                    "test",
                    1,
                    0,
                    StepStatus::ReadyToSchedule,
                    vec![goal_timeframe],
                    None,
                ),
            ];
            expected_steps[0].id = 1;
            expected_steps[1].id = 2;
            expected_steps[2].id = 3;

            assert_eq!(steps, expected_steps);
            assert_eq!(counter, 4);

            assert_eq!(steps[0].id, expected_steps[0].id);
            assert_eq!(steps[1].id, expected_steps[1].id);
            assert_eq!(steps[2].id, expected_steps[2].id);

            assert_eq!(steps[0].duration, expected_steps[0].duration);
            assert_eq!(steps[1].duration, expected_steps[1].duration);
            assert_eq!(steps[2].duration, expected_steps[2].duration);

            assert_eq!(steps[0].status, expected_steps[0].status);
            assert_eq!(steps[1].status, expected_steps[1].status);
            assert_eq!(steps[2].status, expected_steps[2].status);
        }

        #[test]
        fn test_split_into_custom_hours() {
            let step = Step::mock(
                "work",
                1,
                0,
                StepStatus::ReadyToSchedule,
                vec![Slot::mock(Duration::hours(5), 2023, 6, 1, 0, 0)],
                None,
            );

            let expected = step.slots.clone();

            // Test when threshold > slot.duration which return same slot
            let result = step.split_into_custom_hours(6);
            assert_eq!(result.len(), 1);
            assert_eq!(result[0], expected[0]);

            // Test when threshold=0 which return same slot
            let result = step.split_into_custom_hours(0);
            assert_eq!(result.len(), 1);
            assert_eq!(result[0], expected[0]);

            // Test when threshold < slot.duration whcih return splitted slots based on threshold
            let expected = vec![
                Slot::mock(Duration::hours(2), 2023, 6, 1, 0, 0),
                Slot::mock(Duration::hours(2), 2023, 6, 1, 2, 0),
                Slot::mock(Duration::hours(1), 2023, 6, 1, 4, 0),
            ];
            let result = step.split_into_custom_hours(2);
            assert_eq!(result.len(), 3);
            assert_eq!(result, expected);
            assert_eq!(result[0], expected[0]);
            assert_eq!(result[1], expected[1]);
            assert_eq!(result[2], expected[2]);
        }
    }
    mod timeline {
        use crate::models::{slot::Slot, timeline::Timeline};
        use chrono::Duration;

        #[test]
        fn test_split_into_days() {
            let init_year = 2022;
            let init_month = 1;
            let init_day = 1;
            let hour: u32 = 0;
            let minute: u32 = 0;
            let days_count: i64 = 5;
            let duration = Duration::days(days_count);

            let timeline = Timeline::mock(duration, init_year, init_month, init_day);

            let expected_result = Timeline {
                slots: vec![
                    Slot::mock(Duration::days(1), init_year, init_month, 1, hour, minute),
                    Slot::mock(Duration::days(1), init_year, init_month, 2, hour, minute),
                    Slot::mock(Duration::days(1), init_year, init_month, 3, hour, minute),
                    Slot::mock(Duration::days(1), init_year, init_month, 4, hour, minute),
                    Slot::mock(Duration::days(1), init_year, init_month, 5, hour, minute),
                ]
                .into_iter()
                .collect(),
            };

            let splitted_timeline = timeline.split_into_days();

            assert_eq!(expected_result, splitted_timeline);
        }
    }

    mod is_cross_day {
        use chrono::Duration;

        use crate::{
            models::{output::Task, slot::Slot},
            services::splitters::is_cross_day,
        };

        #[test]
        fn test_first_day_of_month_with_hours_from_prev_day() {
            let task_time = Slot::mock(Duration::hours(8), 2021, 12, 31, 22, 0);

            let task = Task {
                taskid: 10,
                goalid: "1".to_string(),
                title: "sleep".to_string(),
                duration: 8,
                start: task_time.start,
                deadline: task_time.end,
                tags: vec![],
                impossible: false,
            };

            assert!(is_cross_day(&task));
        }

        #[test]
        fn test_cross_day() {
            let task_time = Slot::mock(Duration::hours(8), 2022, 1, 1, 22, 0);

            let task = Task {
                taskid: 10,
                goalid: "1".to_string(),
                title: "sleep".to_string(),
                duration: 8,
                start: task_time.start,
                deadline: task_time.end,
                tags: vec![],
                impossible: false,
            };

            assert!(is_cross_day(&task));
        }

        #[test]
        fn test_non_cross_day() {
            let task_time = Slot::mock(Duration::hours(8), 2022, 1, 1, 0, 0);

            let task = Task {
                taskid: 10,
                goalid: "1".to_string(),
                title: "sleep".to_string(),
                duration: 8,
                start: task_time.start,
                deadline: task_time.end,
                tags: vec![],
                impossible: false,
            };

            assert!(!is_cross_day(&task));
        }
    }
}
