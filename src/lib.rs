use std::str::FromStr;
use uuid::Uuid;

const MAX_CALENDAR_UNITS: i32 = 168;
const CALENDAR_UNIT: &str = "h";

pub struct ParseGoalError;

#[derive(Debug, PartialEq)]
pub struct Goal {
    id: Uuid,
    pub title: String,
}

impl Goal {
    /// Construct a new default Goal
    ///
    /// # Example
    /// ```
    /// # use zinzen_scheduler::Goal;
    /// let goal : Goal = Goal::new();
    ///
    /// assert_eq!(
    ///     goal.title,
    ///     String::from("test")
    /// );
    /// ```
    pub fn new() -> Goal {
        Goal {
            id: Uuid::new_v4(),
            title: String::from("test"),
        }
    }

    /// Construct a new Goal from str
    ///
    /// # Example
    /// ```
    /// # use zinzen_scheduler::Goal;
    /// let goal : Goal = Goal::parse_from_str("test from str");
    ///
    /// assert_eq!(
    ///     goal.title,
    ///     String::from("test from str")
    /// );
    /// ```
    pub fn parse_from_str(s: &str) -> Goal {
        Goal {
            id: Uuid::new_v4(),
            title: String::from(s),
        }
    }
}

impl FromStr for Goal {
    type Err = ParseGoalError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Goal {
            id: Uuid::new_v4(),
            title: String::from(s),
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works2() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
