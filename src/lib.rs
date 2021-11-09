use uuid::Uuid;

const MAX_CALENDAR_UNITS: i32 = 168;
const CALENDAR_UNIT: &str = "h";

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
