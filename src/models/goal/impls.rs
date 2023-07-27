use std::collections::BTreeMap;

use crate::GOALS;

use super::{Day, Goal, TimeFilter, GOALS_INITIALIZED};
use log::info;

impl From<String> for Day {
    fn from(day: String) -> Self {
        info!("From<String> day-string: {:?}", day);

        match day.to_lowercase().as_str() {
            "fri" => Day::Fri,
            "sat" => Day::Sat,
            "sun" => Day::Sun,
            "mon" => Day::Mon,
            "tue" => Day::Tue,
            "wed" => Day::Wed,
            "thu" => Day::Thu,
            _ => panic!("Invalid day selection"),
        }
    }
}

impl From<Day> for String {
    fn from(day: Day) -> Self {
        info!("From<Days> day: {:?}", day);
        match day {
            Day::Fri => "Fri".into(),
            Day::Sat => "Sat".into(),
            Day::Sun => "Sun".into(),
            Day::Mon => "Mon".into(),
            Day::Tue => "Tue".into(),
            Day::Wed => "Wed".into(),
            Day::Thu => "Thu".into(),
        }
    }
}

// Todo 2023-05-05  | Check all these setters - Why are they needed? Why public?
impl Goal {
    pub fn new(id: usize) -> Self {
        Self {
            id: id.to_string(),
            title: String::from("Test"),
            ..Default::default()
        }
    }
}

// imple Disply for TimeFilter
impl std::fmt::Display for TimeFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TimeFilter [ after_time: {:?}, before_time: {:?}, on_days: {:?}, not_on: {:?} ]",
            self.after_time, self.before_time, self.on_days, self.not_on
        )
    }
}

/// Initialize static variable GOALS for one time.
/// - Static variable is called GOALS with type Vec<Goal>
/// - GOALS initialized once and can be accessed many times
/// - Note: to read the GOALS, use try_read to avoid blocking (deadlock)
pub fn initialize_goals_globally(goals: Vec<Goal>) {
    GOALS_INITIALIZED.call_once(|| {
        // Initialize the goals vector here if needed.
        // For example:
        let initial_goals: Vec<Goal> = goals.clone();
        let mut goals = GOALS.write().unwrap();
        goals.extend(initial_goals);
    });
}

// test module
#[cfg(test)]
mod tests {
    use crate::{
        models::{
            goal::{impls::initialize_goals_globally, Goal},
            slot::Slot,
        },
        GOALS,
    };

    #[test]
    fn test_initialize_goals_globally() {
        let mut parent_goal = Goal::mock("1", "parent goal", Slot::mock_sample());
        let child_goal = Goal::mock("2", "child goal", Slot::mock_sample());
        parent_goal.children = Some(vec![child_goal.id.to_string()]);
        let goals = vec![parent_goal.clone(), child_goal.clone()];

        // Use try_read to check if GOALS is empty without blocking (deadlock with initialize_goals_globally).
        match GOALS.try_read() {
            Ok(goals_lock) => {
                let is_empty = goals_lock.is_empty();
                assert!(is_empty);
            }
            Err(_) => assert!(false, "GOALS blocked(deadlock)"),
        };

        initialize_goals_globally(goals);

        match GOALS.try_read() {
            Ok(goals_lock) => {
                let is_empty = goals_lock.is_empty();
                assert!(!is_empty);
                assert_eq!(goals_lock.len(), 2);
                assert_eq!(*goals_lock[0].id, "1".to_string());
                assert_eq!(*goals_lock[1].id, "2".to_string());
            }
            Err(_) => assert!(false, "GOALS blocked(deadlock)"),
        };
    }
}
