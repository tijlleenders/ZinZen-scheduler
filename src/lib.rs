use std::fmt;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug)]
enum TaskStatus {
    UNSCHEDULED,
    SCHEDULED,
}

#[derive(Debug)]
enum GoalType {
    FIXED,
    DAILY,
    WEEKLY,
    MONTHLY,
    YEARLY,
}

#[derive(Debug)]
pub struct Slot {
    task_id: u32,
    begin: u32,
    end: u32,
}

#[derive(Debug)]
pub struct Task {
    task_id: u32,
    goal_id: Uuid,
    task_status: TaskStatus,
}

#[derive(Debug)]
pub struct Calendar {
    pub max_time_units: u32,
    pub time_unit_qualifier: String,
    pub goals: Vec<Goal>,
    pub tasks: Vec<Task>,
    pub slots: Vec<Slot>,
}

impl Calendar {
    pub fn new(max_time_units: u32, time_unit_qualifier: String) -> Calendar {
        Calendar {
            max_time_units,
            time_unit_qualifier,
            goals: Vec::new(),
            tasks: Vec::new(),
            slots: Vec::new(),
        }
    }

    pub fn add(&mut self, goal: Goal) -> () {
        self.goals.push(goal);
    }

    pub fn schedule(&mut self) -> () {
        for (index, goal) in self.goals.iter().enumerate() {
            print!("Goal:{:#?}", goal);
            match goal.goal_type {
                GoalType::FIXED => {
                    let task = Task {
                        goal_id: goal.id,
                        task_id: index as u32,
                        task_status: TaskStatus::UNSCHEDULED,
                    };
                    print!("Task:{:#?}", task);
                    self.tasks.push(task);
                    let slot = Slot {
                        begin: goal.start_time as u32, //Need to use goal.start modulo +
                        end: goal.finish_time as u32,  //Need to use goal.start modulo +
                        task_id: index as u32,
                    };
                    self.slots.push(slot);
                }
                _ => {
                    print!("Ignoring task for non-fixed goal type.");
                }
            }
        }
    }
}

pub struct ParseGoalError;

impl fmt::Display for ParseGoalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Not a valid string to construct a Goal from\n")
    }
}

impl fmt::Display for Calendar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Calendar goals:{:#?}\n", self.goals)
    }
}

#[derive(Debug)]
pub struct Goal {
    id: Uuid,
    pub title: String,
    duration: u32,
    start: u32,
    finish: u32,
    start_time: u8,
    finish_time: u8,
    goal_type: GoalType,
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
            duration: 1,
            start: 0,
            finish: 24,
            start_time: 12,
            finish_time: 18,
            goal_type: GoalType::FIXED,
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
            duration: 1,
            start: 0,
            finish: 24,
            start_time: 12,
            finish_time: 18,
            goal_type: GoalType::FIXED,
        }
    }
}

impl FromStr for Goal {
    type Err = ParseGoalError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Err(ParseGoalError),
            _ => Ok(Goal {
                id: Uuid::new_v4(),
                title: String::from(s),
                duration: 1,
                start: 0,
                finish: 24,
                start_time: 12,
                finish_time: 18,
                goal_type: GoalType::FIXED,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Calendar;
    use crate::Goal;

    #[test]
    fn create_and_print_calendar() {
        let calendar = Calendar {
            max_time_units: 168,
            time_unit_qualifier: String::from("h"),
            goals: vec![Goal::new()],
            tasks: Vec::new(),
            slots: Vec::new(),
        };
        print!("\nexpect Calendar with a goal\n");
        print!("Calendar:{:#?}\n", calendar);
    }

    #[test]
    fn add_goal_to_empty_calendar() {
        let goal = Goal::new();
        let mut calendar = Calendar::new(168, String::from("h"));
        calendar.add(goal);
        print!("\nexpect Calendar with a goal\n");
        print!("Calendar:{:#?}\n", calendar);
    }

    #[test]
    fn add_goal_to_empty_calendar_and_schedule() {
        let goal = Goal::new();
        let mut calendar = Calendar::new(168, String::from("h"));
        calendar.add(goal);
        print!("\nexpect Calendar with a goal\n");
        print!("Calendar:{:#?}\n", calendar);
        calendar.schedule();
        print!("Calendar:{:#?}\n", calendar);
    }
}
