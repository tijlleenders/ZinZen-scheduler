use std::fmt;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug)]
enum TaskStatus {
    UNSCHEDULED,
    SCHEDULED,
}

#[derive(Debug)]
pub enum GoalType {
    FIXED,
    DAILY,
    WEEKLY,
    MONTHLY,
    YEARLY,
}

#[derive(Debug)]
pub struct Slot {
    task_id: usize,
    begin: usize,
    end: usize,
}

#[derive(Debug)]
pub struct Task {
    task_id: usize,
    goal_id: Uuid,
    task_status: TaskStatus,
}

#[derive(Debug)]
pub struct Calendar {
    pub max_time_units: usize,
    pub time_unit_qualifier: String,
    pub goals: Vec<Goal>,
    pub tasks: Vec<Task>,
    pub slots: Vec<Slot>,
}

impl Calendar {
    pub fn new(max_time_units: usize, time_unit_qualifier: String) -> Calendar {
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
        for (goal_index, goal) in self.goals.iter().enumerate() {
            print!("Goal:{:#?}\n", goal);
            match goal.goal_type {
                GoalType::FIXED => {
                    let current_task_counter = self.tasks.len() + 1;
                    let task = Task {
                        goal_id: goal.id,
                        task_id: current_task_counter as usize,
                        task_status: TaskStatus::UNSCHEDULED,
                    };
                    print!("Task:{:#?}", task);
                    self.tasks.push(task);
                    let slot = Slot {
                        begin: goal.start as usize + goal.start_time as usize,
                        end: goal.start as usize + goal.finish_time as usize,
                        task_id: goal_index as usize,
                    };
                    self.slots.push(slot);
                }

                GoalType::DAILY => {
                    let mut day_count = 0;
                    while day_count < (self.max_time_units / 24) {
                        let current_task_counter = self.tasks.len();
                        let task = Task {
                            goal_id: goal.id,
                            task_id: (current_task_counter as usize),
                            task_status: TaskStatus::UNSCHEDULED,
                        };
                        self.tasks.push(task);
                        let slot = Slot {
                            begin: day_count * 24 + goal.start_time as usize,
                            end: day_count * 24 + goal.finish_time as usize,
                            task_id: current_task_counter as usize,
                        };
                        self.slots.push(slot);
                        day_count += 1;
                    }
                }

                _ => {
                    print!("Ignoring all but fixed + daily goal types.");
                }
            }

            // find highest scheduling_possibilities
            let mut task_id_highest_scheduling_possibilities_prio: usize = 0;
            let mut highest_scheduling_possibilities_so_far: usize = 0;
            for (task_index, task) in self.tasks.iter().enumerate() {
                let mut scheduling_possibilities: usize = 0;
                for slot in self.slots.iter() {
                    if slot.task_id == task.task_id {
                        let range: usize = slot.end - slot.begin;
                        scheduling_possibilities += range - goal.duration + 1;
                    }
                }
                if scheduling_possibilities > highest_scheduling_possibilities_so_far {
                    print![
                        "Found task {} with scheduling_possibilities {}...higher than previous task {} with {}\n",
                        task_index, scheduling_possibilities, task_id_highest_scheduling_possibilities_prio, highest_scheduling_possibilities_so_far
                    ];
                    task_id_highest_scheduling_possibilities_prio = task_index;
                }
            }

            // find least overlap for task with highest scheduling_possibilities
        }
    }

    pub fn query(self, start: usize, finish: usize) -> () {
        for slot in self.slots.iter() {
            if slot.begin >= start && slot.end < finish {
                print!["found for {}..{}: {:#?}\n", start, finish, slot];
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
    duration: usize,
    start: usize,
    finish: usize,
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
    use crate::GoalType;
    use crate::Uuid;

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

    #[test]
    fn add_daily_goal_to_empty_calendar_and_schedule() {
        let goal = Goal {
            id: Uuid::new_v4(),
            title: String::from("daily goal"),
            duration: 1,
            start: 0,
            finish: 168,
            start_time: 12,
            finish_time: 18,
            goal_type: GoalType::DAILY,
        };
        let mut calendar = Calendar::new(168, String::from("h"));
        calendar.add(goal);
        print!("\nexpect Calendar with a goal\n");
        calendar.schedule();
        print!("Calendar:{:#?}\n", calendar);
    }

    #[test]
    fn add_daily_goal_to_empty_calendar_and_schedule_and_query() {
        let goal = Goal {
            id: Uuid::new_v4(),
            title: String::from("daily goal"),
            duration: 1,
            start: 0,
            finish: 168,
            start_time: 12,
            finish_time: 18,
            goal_type: GoalType::DAILY,
        };
        let mut calendar = Calendar::new(168, String::from("h"));
        calendar.add(goal);
        print!("\nexpect Calendar with a goal\n");
        calendar.schedule();
        print!("Calendar:{:#?}\n", calendar);
        calendar.query(0, 42);
    }
}
