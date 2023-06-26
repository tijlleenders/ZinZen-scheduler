use crate::models::goal::{Goal, Tag};
use crate::models::slots_iterator::TimeSlotsIterator;
use crate::models::step::{NewStep, Step, StepStatus};
use chrono::NaiveDateTime;

impl Step {
    /// Create new step based on NewStep object
    pub fn new(new_step: NewStep) -> Step {
        let start = new_step.timeframe.map(|time| time.start);
        let deadline = new_step.timeframe.map(|time| time.end);

        Step {
            id: new_step.step_id,
            goal_id: new_step.goal.id,
            title: new_step.title,
            duration: new_step.duration,
            status: new_step.status,
            flexibility: 0,
            start,
            deadline,
            slots: new_step.timeline.slots.into_iter().collect(),
            tags: new_step.goal.tags,
            after_goals: new_step.goal.after_goals,
        }
    }

    /// When a step duration exceeded threshold, it will be splitted
    /// into 1 hour steps
    pub fn apply_duration_threshold(&self) -> Vec<Step> {
        let threshold: usize = 8;
        let mut new_step = self.clone();

        if new_step.duration > 0 && new_step.duration < threshold {
            dbg!(&new_step);
            vec![new_step]
        } else {
            // Make first Step with duration as threshold,
            // then split steps into 1 hours steps till finish
            // remaining_duration

            let mut steps: Vec<Step> = Vec::new();
            let mut first_step = new_step.clone();
            first_step.duration = threshold;
            dbg!(&first_step);
            steps.push(first_step);
            dbg!(&steps);

            let mut remaining_duration = new_step.duration - threshold;
            new_step.duration = remaining_duration;
            let new_steps_splitted = new_step.split(&mut remaining_duration).unwrap();
            dbg!(&new_steps_splitted);
            steps.extend(new_steps_splitted);
            dbg!(&steps);

            steps
        }
    }
}

impl Goal {
    /// Generates a Steps from a Processed Goal
    /// **Caution!:*** This can only be done after the Goals have been pre-processed!
    /// Creates and splits the Goal Timeline into one or more segments, making a Step for each.
    /// Depending on the Goal Tag, Steps will also get Tags to help with scheduling order:
    /// - Optional Tag // Todo! add Regular Tag to simplify?
    /// - Filler Tag
    /// - FlexDur Tag
    /// - FlexNum Tag
    /// - Budget Tag
    pub fn generate_steps(
        self,
        calendar_start: NaiveDateTime,
        calendar_end: NaiveDateTime,
        counter: &mut usize,
    ) -> Vec<Step> {
        let mut steps: Vec<Step> = Vec::new();
        if self.tags.contains(&Tag::IgnoreStepGeneration) {
            return steps;
        }

        if self.tags.contains(&Tag::Budget) {
            return steps;
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
            let step_id = *counter;
            *counter += 1;

            if !timeline.slots.is_empty() && self.min_duration.is_some() {
                let title = self.title.clone();
                let duration = self.min_duration.unwrap();

                let new_step = NewStep {
                    step_id,
                    title,
                    duration,
                    goal: self.clone(),
                    timeline,
                    status: StepStatus::ReadyToSchedule,
                    timeframe: None,
                };

                let step = Step::new(new_step);
                dbg!(&step);
                // Apply split on threshold (8 hours) rule if goal is a leaf
                if self.children.is_none() {
                    let thresholded_steps = step.apply_duration_threshold();
                    dbg!(&thresholded_steps);

                    steps.extend(thresholded_steps);
                    dbg!(&steps);
                } else {
                    steps.push(step);
                    dbg!(&steps);
                }
            }
        }
        dbg!(&steps);
        steps
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

                let new_step = NewStep {
                    step_id: 1,
                    title: "test".to_string(),
                    duration,
                    goal: Goal::mock("1", "test", timeframe.clone()),
                    timeline: Timeline::new(),
                    status: StepStatus::ReadyToSchedule,
                    timeframe: Some(timeframe),
                };
                let new_step = Step::new(new_step);
                dbg!(&new_step);

                let generated_steps = new_step.apply_duration_threshold();
                dbg!(&generated_steps);

                let expected_step = Step::mock(
                    "test",
                    7,
                    0,
                    StepStatus::ReadyToSchedule,
                    vec![timeframe],
                    None,
                );

                assert_eq!(generated_steps, vec![expected_step.clone()]);
                assert_eq!(generated_steps[0].id, expected_step.id);
                assert_eq!(generated_steps[0].duration, expected_step.duration);
                assert_eq!(generated_steps[0].status, expected_step.status);
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
            ///    Step {
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
            ///    Step {
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

                let new_step = NewStep {
                    step_id: 1,
                    title: "test".to_string(),
                    duration,
                    goal: Goal::mock("1", "test", timeframe.clone()),
                    timeline,
                    status: StepStatus::ReadyToSchedule,
                    timeframe: None,
                };
                let new_step = Step::new(new_step);
                dbg!(&new_step);
                let generated_steps = new_step.apply_duration_threshold();
                dbg!(&generated_steps);

                let mut expected_steps = vec![
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
                expected_steps[1].id = 2;
                expected_steps[2].id = 3;
                dbg!(&expected_steps);

                assert_eq!(generated_steps, expected_steps);
                assert_eq!(generated_steps.len(), 3);

                assert_eq!(generated_steps[0].id, expected_steps[0].id);
                assert_eq!(generated_steps[1].id, expected_steps[1].id);
                assert_eq!(generated_steps[2].id, expected_steps[2].id);

                assert_eq!(generated_steps[0].duration, expected_steps[0].duration);
                assert_eq!(generated_steps[1].duration, expected_steps[1].duration);
                assert_eq!(generated_steps[2].duration, expected_steps[2].duration);

                assert_eq!(generated_steps[0].status, expected_steps[0].status);
                assert_eq!(generated_steps[1].status, expected_steps[1].status);
                assert_eq!(generated_steps[2].status, expected_steps[2].status);
            }
        }

        mod generate_steps {
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

                let steps =
                    goal.generate_steps(goal_timeframe.start, goal_timeframe.end, &mut counter);
                dbg!(&steps);

                let expected_steps = vec![Step::mock(
                    "test",
                    duration,
                    0,
                    StepStatus::ReadyToSchedule,
                    vec![goal_timeframe],
                    None,
                )];
                dbg!(&expected_steps);

                assert_eq!(steps, expected_steps);
                assert_eq!(counter, 2);

                assert_eq!(steps[0].id, expected_steps[0].id);
                assert_eq!(steps[0].duration, expected_steps[0].duration);
                assert_eq!(steps[0].status, expected_steps[0].status);
            }

            #[test]
            fn test_duration_less_8_hrs() {
                let duration: usize = 6;
                let mut counter: usize = 1;

                let goal_timeframe = Slot::mock(Duration::days(5), 2023, 6, 1, 0, 0);
                let mut goal = Goal::mock("1", "test", goal_timeframe.clone());
                goal.min_duration = Some(duration);
                dbg!(&goal);

                let steps =
                    goal.generate_steps(goal_timeframe.start, goal_timeframe.end, &mut counter);
                dbg!(&steps);

                let expected_steps = vec![Step::mock(
                    "test",
                    duration,
                    0,
                    StepStatus::ReadyToSchedule,
                    vec![goal_timeframe],
                    None,
                )];
                dbg!(&expected_steps);

                assert_eq!(steps, expected_steps);
                assert_eq!(counter, 2);

                assert_eq!(steps[0].id, expected_steps[0].id);
                assert_eq!(steps[0].duration, expected_steps[0].duration);
                assert_eq!(steps[0].status, expected_steps[0].status);
            }

            /// Test Goal::generate_steps when goal.min_duration>8 hours
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
            ///    Step {
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
            ///    Step {
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

                let steps =
                    goal.generate_steps(goal_timeframe.start, goal_timeframe.end, &mut counter);
                dbg!(&steps);

                let mut expected_steps = vec![
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
                expected_steps[1].id = 2;
                expected_steps[2].id = 3;
                dbg!(&expected_steps);

                assert_eq!(steps, expected_steps);
                assert_eq!(counter, 2);

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

            /// Test when a given Goal is not a leaf and goal.min_duration > 8
            /// So in this case, steps will not be splitted
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
            /// expected_step = [
            ///    Step {
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

                let steps =
                    goal.generate_steps(goal_timeframe.start, goal_timeframe.end, &mut counter);
                dbg!(&steps);

                let expected_steps = vec![Step::mock(
                    "test",
                    10,
                    0,
                    StepStatus::ReadyToSchedule,
                    vec![goal_timeframe],
                    None,
                )];
                dbg!(&expected_steps);

                assert_eq!(steps, expected_steps);
                assert_eq!(counter, 2);

                assert_eq!(steps[0].id, expected_steps[0].id);
                assert_eq!(steps[0].duration, expected_steps[0].duration);
                assert_eq!(steps[0].status, expected_steps[0].status);
            }
        }
    }
}
