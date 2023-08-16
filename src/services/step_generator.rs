use crate::models::goal::{Goal, Tag};
use crate::models::slots_iterator::TimeSlotsIterator;
use crate::models::step::{NewStep, Step, StepStatus};
use crate::models::utils::TimingScenario;
use chrono::{Duration, NaiveDateTime};

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
        let mut start = self.start.unwrap_or(calendar_start);
        let mut deadline = self.deadline.unwrap_or(calendar_end);

        if let Some(filter) = self.filters.clone() {
            let timing_scenario = filter.determine_timing_scenario();

            if timing_scenario == TimingScenario::Overflow {
                //- set `start`: subtract a few hours (difference between after and 0:00) from start
                let after_time = filter.after_time.unwrap();
                let diff = 24 - after_time;
                start -= Duration::hours(diff as i64);

                //- set `deadline`: add a few hours (before_time) to deadline
                let before_time = filter.before_time.unwrap();
                deadline += Duration::hours(before_time as i64);
            }
        }

        let time_slots_iterator = TimeSlotsIterator::new(
            start,
            deadline,
            self.repeat,
            self.filters.clone(),
            // Todo! add self.before_time filter
        );

        for timeline in time_slots_iterator {
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

                // Apply split on threshold (8 hours) rule if goal is a leaf, or if goal is a filler goal
                if self.children.is_none() || self.tags.contains(&Tag::Filler) {
                    let thresholded_steps = step.apply_duration_threshold();

                    steps.extend(thresholded_steps);

                    let _i = 0;
                } else {
                    steps.push(step);
                }
            }
        }

        steps
    }
}

#[cfg(test)]
mod tests {
    mod goal {
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
                let mut goal = Goal::mock("1", "test", goal_timeframe);
                goal.min_duration = Some(duration);

                let steps =
                    goal.generate_steps(goal_timeframe.start, goal_timeframe.end, &mut counter);

                let expected_steps = vec![Step::mock(
                    "test",
                    duration,
                    0,
                    StepStatus::ReadyToSchedule,
                    vec![goal_timeframe],
                    None,
                )];

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
                let mut goal = Goal::mock("1", "test", goal_timeframe);
                goal.min_duration = Some(duration);

                let steps =
                    goal.generate_steps(goal_timeframe.start, goal_timeframe.end, &mut counter);

                let expected_steps = vec![Step::mock(
                    "test",
                    duration,
                    0,
                    StepStatus::ReadyToSchedule,
                    vec![goal_timeframe],
                    None,
                )];

                assert_eq!(steps, expected_steps);
                assert_eq!(counter, 2);

                assert_eq!(steps[0].id, expected_steps[0].id);
                assert_eq!(steps[0].duration, expected_steps[0].duration);
                assert_eq!(steps[0].status, expected_steps[0].status);
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
                let mut goal = Goal::mock("1", "test", goal_timeframe);
                goal.min_duration = Some(duration);
                goal.children = Some(vec!["2".to_string()]);

                let steps =
                    goal.generate_steps(goal_timeframe.start, goal_timeframe.end, &mut counter);

                let expected_steps = vec![Step::mock(
                    "test",
                    10,
                    0,
                    StepStatus::ReadyToSchedule,
                    vec![goal_timeframe],
                    None,
                )];

                assert_eq!(steps, expected_steps);
                assert_eq!(counter, 2);

                assert_eq!(steps[0].id, expected_steps[0].id);
                assert_eq!(steps[0].duration, expected_steps[0].duration);
                assert_eq!(steps[0].status, expected_steps[0].status);
            }
        }
    }
}
