use crate::models::goal::{Goal, Tag};
use crate::models::slots_iterator::TimeSlotsIterator;
use crate::models::step::{NewStep, Step, StepStatus};
use chrono::NaiveDateTime;

impl Step {
    /// Create new task based on NewTask object
    pub fn new(new_task: NewStep) -> Step {
        let start = new_task.timeframe.map(|time| time.start);
        let deadline = new_task.timeframe.map(|time| time.end);

        Step {
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

    /// When a task duration exceeded threshold, it will be splitted
    /// into 1 hour tasks
    pub fn apply_duration_threshold(&self) -> Vec<Step> {
        let threshold: usize = 8;
        let mut new_task = self.clone();

        if new_task.duration > 0 && new_task.duration < threshold {
            dbg!(&new_task);
            vec![new_task]
        } else {
            // Make first Task with duration as threshold,
            // then split tasks into 1 hours tasks till finish
            // remaining_duration

            let mut tasks: Vec<Step> = Vec::new();
            let mut first_task = new_task.clone();
            first_task.duration = threshold;
            dbg!(&first_task);
            tasks.push(first_task);
            dbg!(&tasks);

            let mut remaining_duration = new_task.duration - threshold;
            new_task.duration = remaining_duration;
            let new_tasks_splitted = new_task.split(&mut remaining_duration).unwrap();
            dbg!(&new_tasks_splitted);
            tasks.extend(new_tasks_splitted);
            dbg!(&tasks);

            tasks
        }
    }
}

impl Goal {
    /// Generates a Task/Increment from a Processed Goal
    /// **Caution!:*** This can only be done after the Goals have been pre-processed!
    /// Creates and splits the Goal Timeline into one or more segments, making a Task/Increment for each.
    /// Depending on the Goal Tag, Task/Increments will also get Tags to help with scheduling order:
    /// - Optional Tag // Todo! add Regular Tag to simplify?
    /// - Filler Tag
    /// - FlexDur Tag
    /// - FlexNum Tag
    /// - Budget Tag
    pub fn generate_tasks(
        self,
        calendar_start: NaiveDateTime,
        calendar_end: NaiveDateTime,
        counter: &mut usize,
    ) -> Vec<Step> {
        let mut tasks: Vec<Step> = Vec::new();
        if self.tags.contains(&Tag::IgnoreStepGeneration) {
            return tasks;
        }

        if self.tags.contains(&Tag::Budget) {
            return tasks;
        }
        let start = self.start.unwrap_or(calendar_start);
        let deadline = self.deadline.unwrap_or(calendar_end);

        let time_slots_iterator = TimeSlotsIterator::new(
            start,
            deadline,
            self.repeat,
            self.filters.clone(),
            // Todo! add self.before_time filter
        );
        dbg!(&time_slots_iterator);

        for timeline in time_slots_iterator {
            dbg!(&timeline);
            let task_id = *counter;
            *counter += 1;

            if !timeline.slots.is_empty() && self.min_duration.is_some() {
                let title = self.title.clone();
                let duration = self.min_duration.unwrap();

                let new_task = NewStep {
                    task_id,
                    title,
                    duration,
                    goal: self.clone(),
                    timeline,
                    status: StepStatus::ReadyToSchedule,
                    timeframe: None,
                };

                let task = Step::new(new_task);
                dbg!(&task);
                // Apply split on threshold (8 hours) rule if goal is a leaf
                if self.children.is_none() {
                    let thresholded_tasks = task.apply_duration_threshold();
                    dbg!(&thresholded_tasks);

                    tasks.extend(thresholded_tasks);
                    dbg!(&tasks);
                } else {
                    tasks.push(task);
                    dbg!(&tasks);
                }
            }
        }
        dbg!(&tasks);
        tasks
    }
}

#[cfg(test)]
mod tests {

    mod goal {
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

                let new_task = NewStep {
                    task_id: 1,
                    title: "test".to_string(),
                    duration,
                    goal: Goal::mock("1", "test", timeframe.clone()),
                    timeline: Timeline::new(),
                    status: StepStatus::ReadyToSchedule,
                    timeframe: Some(timeframe),
                };
                let new_task = Step::new(new_task);
                dbg!(&new_task);

                let generated_tasks = new_task.apply_duration_threshold();
                dbg!(&generated_tasks);

                let expected_task = Step::mock(
                    "test",
                    7,
                    0,
                    StepStatus::ReadyToSchedule,
                    vec![timeframe],
                    None,
                );

                assert_eq!(generated_tasks, vec![expected_task.clone()]);
                assert_eq!(generated_tasks[0].id, expected_task.id);
                assert_eq!(generated_tasks[0].duration, expected_task.duration);
                assert_eq!(generated_tasks[0].status, expected_task.status);
            }

            /// Test Task::apply_duration_threshold when goal.min_duration > 8 hours
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
            /// expected_task = [
            ///    Task {
            ///        id: 1,
            ///        goal_id: "1",
            ///        title: "test",
            ///        duration: 8,
            ///        status: ReadyToSchedule,
            ///        flexibility: 0,
            ///        start: None,
            ///        deadline: None,
            ///        slots: [
            ///            Slot {
            ///                start:   2023-06-01 00,
            ///                 end:    2023-06-06 00,
            ///            },
            ///        ],
            ///        tags: [],
            ///        after_goals: None,
            ///    },
            ///    Task {
            ///        id: 2,
            ///        goal_id: "1",
            ///        title: "test",
            ///        duration: 1,
            ///        status: ReadyToSchedule,
            ///        flexibility: 0,
            ///        start: None,
            ///        deadline: None,
            ///        slots: [
            ///            Slot {
            ///                start:   2023-06-01 00,
            ///                 end:    2023-06-06 00,
            ///            },
            ///        ],
            ///        tags: [],
            ///        after_goals: None,
            ///    },
            ///    Task {
            ///        id: 3,
            ///        goal_id: "1",
            ///        title: "test",
            ///        duration: 1,
            ///        status: ReadyToSchedule,
            ///        flexibility: 0,
            ///        start: None,
            ///        deadline: None,
            ///        slots: [
            ///            Slot {
            ///                start:   2023-06-01 00,
            ///                 end:    2023-06-06 00,
            ///            },
            ///        ],
            ///        tags: [],
            ///        after_goals: None,
            ///    },
            ///
            ///]
            ///
            ///
            /// ```
            #[test]
            fn test_duration_more_8_hrs() {
                let duration: usize = 10;
                let timeframe = Slot::mock(Duration::days(5), 2023, 6, 1, 0, 0);
                let timeline = Timeline {
                    slots: vec![timeframe.clone()].into_iter().collect(),
                };

                let new_task = NewStep {
                    task_id: 1,
                    title: "test".to_string(),
                    duration,
                    goal: Goal::mock("1", "test", timeframe.clone()),
                    timeline,
                    status: StepStatus::ReadyToSchedule,
                    timeframe: None,
                };
                let new_task = Step::new(new_task);
                dbg!(&new_task);
                let generated_tasks = new_task.apply_duration_threshold();
                dbg!(&generated_tasks);

                let mut expected_task = vec![
                    Step::mock(
                        "test",
                        8,
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
                expected_task[1].id = 2;
                expected_task[2].id = 3;
                dbg!(&expected_task);

                assert_eq!(generated_tasks, expected_task);
                assert_eq!(generated_tasks.len(), 3);

                assert_eq!(generated_tasks[0].id, expected_task[0].id);
                assert_eq!(generated_tasks[1].id, expected_task[1].id);
                assert_eq!(generated_tasks[2].id, expected_task[2].id);

                assert_eq!(generated_tasks[0].duration, expected_task[0].duration);
                assert_eq!(generated_tasks[1].duration, expected_task[1].duration);
                assert_eq!(generated_tasks[2].duration, expected_task[2].duration);

                assert_eq!(generated_tasks[0].status, expected_task[0].status);
                assert_eq!(generated_tasks[1].status, expected_task[1].status);
                assert_eq!(generated_tasks[2].status, expected_task[2].status);
            }
        }

        mod generate_tasks {
            use chrono::Duration;

            use crate::models::{
                goal::Goal,
                slot::Slot,
                step::{Step, StepStatus},
            };

            #[test]
            fn test_duration_1_hr() {
                let duration: usize = 1;
                let mut counter: usize = 1;

                let goal_timeframe = Slot::mock(Duration::days(5), 2023, 6, 1, 0, 0);
                let mut goal = Goal::mock("1", "test", goal_timeframe.clone());
                goal.min_duration = Some(duration);
                dbg!(&goal);

                let tasks =
                    goal.generate_tasks(goal_timeframe.start, goal_timeframe.end, &mut counter);
                dbg!(&tasks);

                let expected_task = vec![Step::mock(
                    "test",
                    duration,
                    0,
                    StepStatus::ReadyToSchedule,
                    vec![goal_timeframe],
                    None,
                )];
                dbg!(&expected_task);

                assert_eq!(tasks, expected_task);
                assert_eq!(counter, 2);

                assert_eq!(tasks[0].id, expected_task[0].id);
                assert_eq!(tasks[0].duration, expected_task[0].duration);
                assert_eq!(tasks[0].status, expected_task[0].status);
            }

            #[test]
            fn test_duration_less_8_hrs() {
                let duration: usize = 6;
                let mut counter: usize = 1;

                let goal_timeframe = Slot::mock(Duration::days(5), 2023, 6, 1, 0, 0);
                let mut goal = Goal::mock("1", "test", goal_timeframe.clone());
                goal.min_duration = Some(duration);
                dbg!(&goal);

                let tasks =
                    goal.generate_tasks(goal_timeframe.start, goal_timeframe.end, &mut counter);
                dbg!(&tasks);

                let expected_task = vec![Step::mock(
                    "test",
                    duration,
                    0,
                    StepStatus::ReadyToSchedule,
                    vec![goal_timeframe],
                    None,
                )];
                dbg!(&expected_task);

                assert_eq!(tasks, expected_task);
                assert_eq!(counter, 2);

                assert_eq!(tasks[0].id, expected_task[0].id);
                assert_eq!(tasks[0].duration, expected_task[0].duration);
                assert_eq!(tasks[0].status, expected_task[0].status);
            }

            /// Test Goal::generate_tasks when goal.min_duration>8 hours
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
            /// expected_task = [
            ///    Task {
            ///        id: 1,
            ///        goal_id: "1",
            ///        title: "test",
            ///        duration: 8,
            ///        status: ReadyToSchedule,
            ///        flexibility: 0,
            ///        start: None,
            ///        deadline: None,
            ///        slots: [
            ///            Slot {
            ///                start:   2023-06-01 00,
            ///                 end:    2023-06-06 00,
            ///            },
            ///        ],
            ///        tags: [],
            ///        after_goals: None,
            ///    },
            ///    Task {
            ///        id: 2,
            ///        goal_id: "1",
            ///        title: "test",
            ///        duration: 1,
            ///        status: ReadyToSchedule,
            ///        flexibility: 0,
            ///        start: None,
            ///        deadline: None,
            ///        slots: [
            ///            Slot {
            ///                start:   2023-06-01 00,
            ///                 end:    2023-06-06 00,
            ///            },
            ///        ],
            ///        tags: [],
            ///        after_goals: None,
            ///    },
            ///    Task {
            ///        id: 3,
            ///        goal_id: "1",
            ///        title: "test",
            ///        duration: 1,
            ///        status: ReadyToSchedule,
            ///        flexibility: 0,
            ///        start: None,
            ///        deadline: None,
            ///        slots: [
            ///            Slot {
            ///                start:   2023-06-01 00,
            ///                 end:    2023-06-06 00,
            ///            },
            ///        ],
            ///        tags: [],
            ///        after_goals: None,
            ///    },
            ///]
            ///
            ///
            /// ```
            #[test]
            fn test_duration_more_8_hrs() {
                let duration: usize = 10;
                let mut counter: usize = 1;

                let goal_timeframe = Slot::mock(Duration::days(5), 2023, 6, 1, 0, 0);
                let mut goal = Goal::mock("1", "test", goal_timeframe.clone());
                goal.min_duration = Some(duration);
                dbg!(&goal);

                let tasks =
                    goal.generate_tasks(goal_timeframe.start, goal_timeframe.end, &mut counter);
                dbg!(&tasks);

                let mut expected_task = vec![
                    Step::mock(
                        "test",
                        8,
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
                expected_task[1].id = 2;
                expected_task[2].id = 3;
                dbg!(&expected_task);

                assert_eq!(tasks, expected_task);
                assert_eq!(counter, 2);

                assert_eq!(tasks[0].id, expected_task[0].id);
                assert_eq!(tasks[1].id, expected_task[1].id);
                assert_eq!(tasks[2].id, expected_task[2].id);

                assert_eq!(tasks[0].duration, expected_task[0].duration);
                assert_eq!(tasks[1].duration, expected_task[1].duration);
                assert_eq!(tasks[2].duration, expected_task[2].duration);

                assert_eq!(tasks[0].status, expected_task[0].status);
                assert_eq!(tasks[1].status, expected_task[1].status);
                assert_eq!(tasks[2].status, expected_task[2].status);
            }

            /// Test when a given Goal is not a leaf and goal.min_duration > 8
            /// So in this case, tasks will not be splitted
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
            ///    children: ["2"],
            ///    after_goals: None,
            ///}
            ///
            /// ===========================
            /// Output:
            /// expected_task = [
            ///    Task {
            ///        id: 1,
            ///        goal_id: "1",
            ///        title: "test",
            ///        duration: 10,
            ///        status: ReadyToSchedule,
            ///        flexibility: 0,
            ///        start: None,
            ///        deadline: None,
            ///        slots: [
            ///            Slot {
            ///                start:   2023-06-01 00,
            ///                 end:    2023-06-06 00,
            ///            },
            ///        ],
            ///        tags: [],
            ///        after_goals: None,
            ///    },
            ///]
            /// ```
            #[test]
            fn test_goal_is_not_leaf_duration_more_8_hrs() {
                let duration: usize = 10;
                let mut counter: usize = 1;

                let goal_timeframe = Slot::mock(Duration::days(5), 2023, 6, 1, 0, 0);
                let mut goal = Goal::mock("1", "test", goal_timeframe.clone());
                goal.min_duration = Some(duration);
                goal.children = Some(vec!["2".to_string()]);
                dbg!(&goal);

                let tasks =
                    goal.generate_tasks(goal_timeframe.start, goal_timeframe.end, &mut counter);
                dbg!(&tasks);

                let expected_task = vec![Step::mock(
                    "test",
                    10,
                    0,
                    StepStatus::ReadyToSchedule,
                    vec![goal_timeframe],
                    None,
                )];
                dbg!(&expected_task);

                assert_eq!(tasks, expected_task);
                assert_eq!(counter, 2);

                assert_eq!(tasks[0].id, expected_task[0].id);
                assert_eq!(tasks[0].duration, expected_task[0].duration);
                assert_eq!(tasks[0].status, expected_task[0].status);
            }
        }
    }
}
