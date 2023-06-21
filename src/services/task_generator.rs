use crate::models::goal::{Goal, Tag};
use crate::models::slots_iterator::TimeSlotsIterator;
use crate::models::task::{NewTask, Task, TaskStatus};
use chrono::NaiveDateTime;

impl NewTask {
    /// Generate tasks based on a given NewTask and apply business rules
    /// - Rule: task.duration must be greater than 1 and less than 8,
    /// if more than  8, will be splitted
    pub fn generate_tasks(&self) -> Vec<Task> {
        let threshold: usize = 8;
        if self.duration > 1 && self.duration < threshold {
            let task = Task::new(self.clone());
            dbg!(&task);
            vec![task]
        } else {
            let mut total_duration: usize = self.duration;
            let mut tasks: Vec<Task> = Vec::new();
            // =========== First way to calculate new tasks
            // - Deduct from total_duration 8 hours (or less for last task) each time until total_duration = 0
            let mut counter = 0;
            while total_duration > 0 {
                let new_duration: usize;

                if total_duration >= threshold {
                    total_duration -= threshold;
                    new_duration = threshold;
                } else {
                    new_duration = total_duration;
                    total_duration = 0;
                }

                let mut new_task = self.clone();
                new_task.duration = new_duration;
                new_task.task_id += counter;
                tasks.push(Task::new(new_task));
                counter += 1;
                dbg!(&tasks);
            }

            tasks
        }
    }
}

impl Task {
    /// Create new task based on NewTask object
    /// - Note: in case you want to create new tasks based on business rule, kindly use NewTask::generate_tasks
    pub fn new(new_task: NewTask) -> Task {
        let start = new_task.timeframe.map(|time| time.start);
        let deadline = new_task.timeframe.map(|time| time.end);

        Task {
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
    ) -> Vec<Task> {
        let mut tasks: Vec<Task> = Vec::new();
        if self.tags.contains(&Tag::IgnoreForTaskGeneration) {
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

                let new_task = NewTask {
                    task_id,
                    title,
                    duration,
                    goal: self.clone(),
                    timeline,
                    status: TaskStatus::ReadyToSchedule,
                    timeframe: None,
                };

                // let task = Task::new(new_task);
                // dbg!(&task);

                let new_tasks = new_task.generate_tasks();
                dbg!(&new_tasks);

                tasks.extend(new_tasks);
                dbg!(&tasks);
            }
        }
        dbg!(&tasks);
        tasks
    }
}

#[cfg(test)]
mod tests {

    mod new_task {
        use crate::models::{
            goal::Goal,
            slot::Slot,
            task::{NewTask, Task, TaskStatus},
            timeline::Timeline,
        };
        use chrono::Duration;

        #[test]
        fn test_duration_is_less_8_hrs() {
            let duration: usize = 7;
            let timeframe = Slot::mock(Duration::days(5), 2023, 6, 1, 0, 0);

            let new_task = NewTask {
                task_id: 1,
                title: "test".to_string(),
                duration,
                goal: Goal::mock("1", "test", timeframe.clone()),
                timeline: Timeline::new(),
                status: TaskStatus::ReadyToSchedule,
                timeframe: Some(timeframe),
            };

            let expected_task =
                Task::mock("test", 7, 0, TaskStatus::ReadyToSchedule, vec![timeframe]);

            let generated_task = new_task.generate_tasks();

            assert_eq!(generated_task, vec![expected_task.clone()]);
            assert_eq!(generated_task[0].id, expected_task.id);
            assert_eq!(generated_task[0].duration, expected_task.duration);
            assert_eq!(generated_task[0].status, expected_task.status);
        }

        #[test]
        fn test_duration_is_more_8_hrs() {
            let duration: usize = 10;

            let timeframe = Slot::mock(Duration::days(5), 2023, 6, 1, 0, 0);

            let new_task = NewTask {
                task_id: 1,
                title: "test".to_string(),
                duration,
                goal: Goal::mock("1", "test", timeframe.clone()),
                timeline: Timeline::new(),
                status: TaskStatus::ReadyToSchedule,
                timeframe: Some(timeframe),
            };
            dbg!(&new_task);

            let mut expected_tasks = vec![
                Task::mock("test", 8, 0, TaskStatus::ReadyToSchedule, vec![timeframe]),
                Task::mock("test", 2, 0, TaskStatus::ReadyToSchedule, vec![timeframe]),
            ];
            expected_tasks[1].id = 2;
            dbg!(&expected_tasks);

            let tasks = new_task.generate_tasks();
            dbg!(&tasks);

            assert_eq!(tasks, expected_tasks);

            assert_eq!(tasks[0].id, expected_tasks[0].id);
            assert_eq!(tasks[1].id, expected_tasks[1].id);

            assert_eq!(tasks[0].duration, expected_tasks[0].duration);
            assert_eq!(tasks[1].duration, expected_tasks[1].duration);

            assert_eq!(tasks[0].status, expected_tasks[0].status);
            assert_eq!(tasks[1].status, expected_tasks[1].status);
        }
    }

    mod goal_tasks {
        use chrono::Duration;

        use crate::models::{
            goal::Goal,
            slot::Slot,
            task::{Task, TaskStatus},
        };

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
        ///        duration: 2,
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

            let tasks = goal.generate_tasks(goal_timeframe.start, goal_timeframe.end, &mut counter);
            dbg!(&tasks);

            let mut expected_task = vec![
                Task::mock(
                    "test",
                    8,
                    0,
                    TaskStatus::ReadyToSchedule,
                    vec![goal_timeframe],
                ),
                Task::mock(
                    "test",
                    2,
                    0,
                    TaskStatus::ReadyToSchedule,
                    vec![goal_timeframe],
                ),
            ];
            expected_task[1].id = 2;
            dbg!(&expected_task);

            assert_eq!(tasks, expected_task);
            assert_eq!(counter, 2);

            assert_eq!(tasks[0].id, expected_task[0].id);
            assert_eq!(tasks[1].id, expected_task[1].id);

            assert_eq!(tasks[0].duration, expected_task[0].duration);
            assert_eq!(tasks[1].duration, expected_task[1].duration);

            assert_eq!(tasks[0].status, expected_task[0].status);
            assert_eq!(tasks[1].status, expected_task[1].status);
        }
    }
}
