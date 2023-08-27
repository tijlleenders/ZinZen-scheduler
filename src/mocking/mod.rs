use chrono::{Duration, NaiveDate};

use rand::prelude::*;

use crate::models::{
    goal::Goal,
    slot::{iterator::SlotIterator, Slot},
    step::{Step, StepStatus},
    timeline::Timeline,
};

impl Slot {
    pub fn mock(
        duration: Duration,
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
    ) -> Slot {
        let start = NaiveDate::from_ymd_opt(year, month, day)
            .unwrap()
            .and_hms_opt(hour, minute, 0)
            .unwrap();
        let end = start + duration;

        Slot { start, end }
    }

    /// Get a sample of a slot for 1 day "2023-01-01"
    pub fn mock_sample() -> Slot {
        let start = NaiveDate::from_ymd_opt(2023, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let end = start + Duration::days(1);

        Slot { start, end }
    }
}

impl Timeline {
    /// Utility function to return a timeline list of slots splitted on daily basis
    pub fn mock_as_days(
        days_count: i64,
        start_year: i32,
        start_month: u32,
        start_day: u32,
    ) -> Timeline {
        if days_count < 1 {
            return Timeline::new();
        }
        let init_slot = Slot::mock(
            Duration::days(days_count),
            start_year,
            start_month,
            start_day,
            0,
            0,
        );

        let slot_iter = SlotIterator::new(init_slot, Duration::days(1));

        let mut slots_days: Vec<Slot> = vec![];
        for slot in slot_iter {
            slots_days.push(slot);
        }

        Timeline {
            slots: slots_days.into_iter().collect(),
        }
    }

    /// Utility function to return a timeline with a single slot with respect to duration
    pub fn mock(duration: Duration, year: i32, month: u32, day: u32) -> Timeline {
        let slot = Slot::mock(duration, year, month, day, 0, 0);
        Timeline {
            slots: vec![slot].into_iter().collect(),
        }
    }
}

impl Step {
    /// Mock a custom Step
    /// ```markdown
    /// Step {
    ///     id: 1,
    ///     goal_id: "1",
    ///     title: title,
    ///     duration: duration,
    ///     status: status,
    ///     flexibility: flexibility,
    ///     start: Some(timeframe.start),
    ///     deadline: Some(timeframe.end),
    ///     slots: slots,
    ///     tags: vec![],
    ///     after_goals: None
    ///}
    /// ```
    pub fn mock(
        title: &str,
        duration: usize,
        flexibility: usize,
        status: StepStatus,
        slots: Vec<Slot>,
        timeframe: Option<Slot>,
    ) -> Step {
        let start = timeframe.map(|time| time.start);
        let deadline = timeframe.map(|time| time.end);

        let id = rand::thread_rng().gen_range(0..10000);

        Step {
            id,
            title: title.to_string(),
            duration,
            status,
            flexibility,
            start,
            deadline,
            slots,
            tags: vec![],
            after_goals: None,
            goal_id: "1".to_string(),
        }
    }

    /// Mock a Scheduled Step
    /// ```markdown
    /// Step {
    ///     id: id,
    ///     goal_id: goal_id,
    ///     title: title,
    ///     duration: duration,
    ///     status: StepStatus::Scheduled,
    ///     flexibility: flexibility,
    ///     start: step_timing.start,
    ///     deadline: step_timing.end,
    ///     slots: vec![],
    ///     tags: vec![],
    ///     after_goals: None
    ///}
    /// ```
    pub fn mock_scheduled(
        id: usize,
        goal_id: &str,
        title: &str,
        duration: usize,
        flexibility: usize,
        step_timing: Slot,
    ) -> Step {
        Step {
            id,
            goal_id: goal_id.to_string(),
            title: title.to_string(),
            duration,
            status: StepStatus::Scheduled,
            flexibility,
            start: Some(step_timing.start),
            deadline: Some(step_timing.end),
            slots: vec![],
            tags: vec![],
            after_goals: None,
        }
    }
}

impl Goal {
    /// Mock a basic Goal
    /// ```markdown
    /// Goal {
    ///    id: id,
    ///    title: title,
    ///    min_duration: None,
    ///    max_duration: None,
    ///    budgets: None,
    ///    repeat: None,
    ///    start: None,
    ///    deadline: None,
    ///    tags: vec![],
    ///    filters: None,
    ///    children: None,
    ///    after_goals: None,
    /// }
    /// ```
    pub fn mock(id: &str, title: &str, goal_dates: Slot) -> Goal {
        Goal {
            id: id.to_string(),
            title: title.to_string(),
            min_duration: None,
            max_duration: None,
            budgets: None,
            repeat: None,
            start: Some(goal_dates.start),
            deadline: Some(goal_dates.end),
            tags: vec![],
            filters: None,
            children: None,
            after_goals: None,
        }
    }
}

#[cfg(test)]
mod tests {

    mod goal {
        use chrono::Duration;

        use crate::models::{goal::Goal, slot::Slot};

        #[test]
        fn test_mock() {
            let goal_dates = Slot::mock(Duration::days(15), 2023, 5, 1, 0, 0);
            let goal = Goal::mock("1", "goal sample", goal_dates);

            let expected_goal = Goal {
                id: "1".to_string(),
                title: "goal sample".to_string(),
                min_duration: None,
                max_duration: None,
                budgets: None,
                repeat: None,
                start: Some(goal_dates.start),
                deadline: Some(goal_dates.end),
                tags: vec![],
                filters: None,
                children: None,
                after_goals: None,
            };

            assert_eq!(goal, expected_goal);
        }
    }
    mod step {
        use chrono::Duration;

        use crate::models::{
            slot::Slot,
            step::{Step, StepStatus},
        };

        #[test]
        fn test_mock() {
            let slots = vec![Slot::mock(Duration::days(1), 2023, 5, 1, 0, 0)];

            let expected = Step {
                id: 1,
                title: "A sample step".to_string(),
                duration: 1,
                status: StepStatus::ReadyToSchedule,
                flexibility: 168,
                start: None,
                deadline: None,
                slots: slots.clone(),
                tags: vec![],
                after_goals: None,
                goal_id: "1".to_string(),
            };

            let result = Step::mock("test", 1, 168, StepStatus::ReadyToSchedule, slots, None);

            assert_eq!(expected, result);
        }

        #[test]
        fn test_mock_scheduled() {
            let expected_step_timing = Slot::mock(Duration::days(1), 2023, 5, 1, 0, 0);

            let expected = Step {
                id: 1,
                goal_id: "1".to_string(),
                title: "Sheduled Step".to_string(),
                duration: 1,
                status: StepStatus::Scheduled,
                flexibility: 168,
                start: Some(expected_step_timing.start),
                deadline: Some(expected_step_timing.end),
                slots: vec![],
                tags: vec![],
                after_goals: None,
            };

            let result = Step::mock_scheduled(
                1,
                "1",
                "Sheduled Step",
                1,
                168,
                Slot::mock(Duration::days(1), 2023, 5, 1, 0, 0),
            );

            assert_eq!(expected.status, result.status);
            assert_eq!(expected, result);
        }
    }
    use chrono::{Datelike, Timelike};

    use super::*;

    #[test]
    fn test_mock_slot() {
        let duration = Duration::hours(1);
        let year = 2023;
        let month = 5;
        let day = 1;
        let hour = 5;
        let minute = 0;

        let slot = Slot::mock(duration, year, month, day, hour, minute);

        assert_eq!(slot.start.year(), year);
        assert_eq!(slot.start.month(), month);
        assert_eq!(slot.start.day(), day);
        assert_eq!(slot.start.hour(), hour);
        assert_eq!(slot.start.minute(), minute);

        assert_eq!(slot.end.hour(), hour + 1);
        assert_eq!(slot.end.minute(), minute);
    }

    #[test]
    fn test_mock_slot_for_day() {
        let duration = Duration::days(1);
        let year = 2023;
        let month = 5;
        let day = 1;
        let hour = 5;
        let minute = 0;

        let slot = Slot::mock(duration, year, month, day, hour, minute);

        assert_eq!(slot.start.year(), year);
        assert_eq!(slot.start.month(), month);
        assert_eq!(slot.start.day(), day);
        assert_eq!(slot.start.hour(), hour);
        assert_eq!(slot.start.minute(), minute);

        assert_eq!(slot.end.day(), day + 1);
        assert_eq!(slot.end.hour(), hour);
        assert_eq!(slot.end.minute(), minute);
    }

    #[test]
    fn test_mock_as_days() {
        // Test for days_count = 0
        let timeline = Timeline::mock_as_days(0, 2023, 5, 1);
        assert_eq!(timeline, Timeline::new());

        // Test for days_count = 1
        let timeline = Timeline::mock_as_days(1, 2023, 5, 1);
        assert_eq!(
            timeline,
            Timeline {
                slots: vec![Slot::mock(Duration::days(1), 2023, 5, 1, 0, 0)]
                    .into_iter()
                    .collect()
            }
        );

        // Test for days_count = 3
        let timeline = Timeline::mock_as_days(3, 2023, 5, 1);
        assert_eq!(
            timeline,
            Timeline {
                slots: vec![
                    Slot::mock(Duration::days(1), 2023, 5, 1, 0, 0),
                    Slot::mock(Duration::days(1), 2023, 5, 2, 0, 0),
                    Slot::mock(Duration::days(1), 2023, 5, 3, 0, 0),
                ]
                .into_iter()
                .collect()
            }
        );
    }
}
