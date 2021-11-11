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
    duration_to_schedule: usize,
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

#[derive(Debug)]
pub struct Goal {
    id: Uuid,
    pub title: String,
    estimated_duration: usize,
    effort_invested: usize,
    start: usize,
    finish: usize,
    start_time: u8,
    finish_time: u8,
    goal_type: GoalType,
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
        self.load_tasks_and_slots_from_goals();

        print!("Calendar after loading:{:#?}\n", self);

        let task_id_with_highest_scheduling_possibilities: usize =
            self.find_task_id_with_highest_scheduling_possibilities();

        let least_overlap_interval: (usize, usize) = self
            .find_least_overlap_interval_for_task(task_id_with_highest_scheduling_possibilities);
        print!(
            "least overlap for task_id {}:{}-{}\n",
            task_id_with_highest_scheduling_possibilities,
            least_overlap_interval.0,
            least_overlap_interval.1
        );
    }

    pub fn query(self, start: usize, finish: usize) -> () {
        for slot in self.slots.iter() {
            if slot.begin >= start && slot.end < finish {
                print!["found for {}..{}: {:#?}\n", start, finish, slot];
            }
        }
    }

    fn find_least_overlap_interval_for_task(&self, task_id: usize) -> (usize, usize) {
        for slot in self.slots.iter() {
            if slot.task_id == task_id {
                let mut lowest_overlap_so_far: usize = usize::MAX - 1;
                let mut offset_with_lowest_overlap: usize = 0;
                for slot_offset in
                    0..slot.end - slot.begin - self.tasks[task_id].duration_to_schedule + 1
                {
                    let overlap = self.find_overlap_number_for(
                        slot.begin + slot_offset,
                        slot.begin + slot_offset + self.tasks[task_id].duration_to_schedule,
                    );
                    print!(
                        "# overlaps for:{}-{}:{}\n",
                        slot.begin + slot_offset,
                        slot.begin + slot_offset + self.tasks[task_id].duration_to_schedule,
                        overlap
                    );
                    if overlap <= lowest_overlap_so_far + 1 {
                        lowest_overlap_so_far = overlap;
                        offset_with_lowest_overlap = slot_offset;
                    }
                }
            }
        }

        let _task_id = task_id;
        (0, 0)
    }

    fn find_overlap_number_for(&self, begin: usize, end: usize) -> usize {
        let mut result: usize = 0;
        for slot in self.slots.iter() {
            if slot.begin < end && slot.end > begin {
                result += 1;
            }
        }
        result
    }

    fn find_task_id_with_highest_scheduling_possibilities(&self) -> usize {
        let mut task_id_highest_scheduling_possibilities_prio: usize = 0;
        let mut highest_scheduling_possibilities_so_far: usize = 0;
        for (task_index, task) in self.tasks.iter().enumerate() {
            let mut scheduling_possibilities: usize = 0;
            for slot in self.slots.iter() {
                if slot.task_id == task.task_id {
                    let range: usize = slot.end - slot.begin;
                    scheduling_possibilities += range - task.duration_to_schedule + 1;
                }
            }
            if scheduling_possibilities > highest_scheduling_possibilities_so_far {
                print![
                    "Found task {} with scheduling_possibilities {}...higher than previous task {} with {}\n",
                    task_index, scheduling_possibilities, task_id_highest_scheduling_possibilities_prio, highest_scheduling_possibilities_so_far
                    ];
                task_id_highest_scheduling_possibilities_prio = task_index;
                highest_scheduling_possibilities_so_far = scheduling_possibilities;
            }
        }
        task_id_highest_scheduling_possibilities_prio
    }

    fn load_tasks_and_slots_from_goals(&mut self) -> () {
        for (goal_index, goal) in self.goals.iter().enumerate() {
            print!("Goal:{:#?}\n", goal);
            match goal.goal_type {
                GoalType::FIXED => {
                    let current_task_counter = self.tasks.len();
                    let task = Task {
                        goal_id: goal.id,
                        duration_to_schedule: goal.estimated_duration - goal.effort_invested,
                        task_id: current_task_counter as usize,
                        task_status: TaskStatus::UNSCHEDULED,
                    };
                    print!("Task:{:#?}\n", task);
                    self.tasks.push(task);
                    let slot = Slot {
                        begin: goal.start as usize + goal.start_time as usize,
                        end: goal.start as usize + goal.finish_time as usize,
                        task_id: current_task_counter as usize,
                    };
                    self.slots.push(slot);
                }

                GoalType::DAILY => {
                    let mut day_count = 0;
                    while day_count < (self.max_time_units / 24) {
                        let current_task_counter = self.tasks.len();
                        let task = Task {
                            goal_id: goal.id,
                            duration_to_schedule: goal.estimated_duration - goal.effort_invested,
                            task_id: current_task_counter as usize,
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
        write!(
            f,
            "Calendar:\nGoals:{:#?}\nTasks:{:#?}\nSlots:{:#?}\n",
            self.goals, self.tasks, self.slots
        )
    }
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
            estimated_duration: 1,
            effort_invested: 0,
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
            estimated_duration: 1,
            effort_invested: 0,
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
                estimated_duration: 1,
                effort_invested: 0,
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
            estimated_duration: 1,
            effort_invested: 0,
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
            estimated_duration: 1,
            effort_invested: 0,
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

    #[test]
    fn fixed_and_daily_goal_combined() {
        let mut calendar = Calendar::new(168, String::from("h"));

        let goal = Goal {
            id: Uuid::new_v4(),
            title: String::from("daily goal"),
            estimated_duration: 1,
            effort_invested: 0,
            start: 0,
            finish: 168,
            start_time: 12,
            finish_time: 18,
            goal_type: GoalType::DAILY,
        };

        let goal2 = Goal {
            id: Uuid::new_v4(),
            title: String::from("lunch meeting any day"),
            estimated_duration: 1,
            effort_invested: 0,
            start: 0,
            finish: 168,
            start_time: 12,
            finish_time: 13,
            goal_type: GoalType::FIXED,
        };
        calendar.add(goal);
        calendar.add(goal2);

        print!("Calendar:{:#?}\n", calendar);

        print!("\nexpect Calendar with two goals not overlapping\n");
        calendar.schedule();

        calendar.query(12, 14);
    }
}
