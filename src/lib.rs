use std::str::FromStr;
use std::{fmt, usize};
use uuid::Uuid;

#[derive(Debug)]
enum TaskStatus {
    UNSCHEDULED,
    SCHEDULED,
}

#[derive(Debug, PartialEq)]
pub enum CutOffType {
    NOCUT,
    CUTSTART,
    CUTEND,
    CUTMIDDLE,
    CUTWHOLE,
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

        loop {
            let unscheduled_task_id_with_highest_scheduling_possibilities: Option<usize> =
                self.find_unscheduled_task_id_with_highest_scheduling_possibilities();

            match unscheduled_task_id_with_highest_scheduling_possibilities {
                Some(task_id_to_schedule) => {
                    let least_overlap_interval: (usize, usize) =
                        self.find_least_overlap_interval_for_task(task_id_to_schedule);
                    print!(
                        "least overlap for task_id {}:{}-{}\n",
                        task_id_to_schedule, least_overlap_interval.0, least_overlap_interval.1
                    );
                    self.schedule_task(
                        task_id_to_schedule,
                        least_overlap_interval.0,
                        least_overlap_interval.1,
                    );
                }
                None => break,
            }
        }
    }

    fn schedule_task(&mut self, task_id: usize, begin: usize, end: usize) -> () {
        print!("Scheduling task_id {}.\n", task_id);
        self.slots.retain(|slot| {
            let delete = { slot.task_id == task_id };
            !delete
        });
        let scheduled_slot = Slot {
            task_id,
            begin,
            end,
        };
        let mut new_slots: Vec<Slot> = Vec::with_capacity(self.slots.capacity());
        for slot in self.slots.iter() {
            let cut_off_type = Calendar::find_cut_off_type(&slot, begin, end);
            match cut_off_type {
                CutOffType::CUTSTART => {
                    new_slots.push(Slot {
                        task_id: slot.task_id,
                        begin: end,
                        end: slot.end,
                    });
                }
                CutOffType::CUTMIDDLE => {
                    new_slots.push(Slot {
                        task_id: slot.task_id,
                        begin: slot.begin,
                        end: begin,
                    });
                    new_slots.push(Slot {
                        task_id: slot.task_id,
                        begin: end,
                        end: slot.end,
                    });
                }
                CutOffType::CUTEND => {
                    new_slots.push(Slot {
                        task_id: slot.task_id,
                        begin: slot.begin,
                        end: begin,
                    });
                }
                CutOffType::NOCUT => {
                    new_slots.push(Slot {
                        task_id: slot.task_id,
                        begin: slot.begin,
                        end: slot.end,
                    });
                }
                CutOffType::CUTWHOLE => {}
            }
        }
        self.slots = new_slots;
        self.slots.push(scheduled_slot);
        self.tasks[task_id].task_status = TaskStatus::SCHEDULED;
        print!(
            "Calendar right after scheduling task_id {}:{:#?}\n",
            task_id, self
        );
        ()
    }

    pub fn find_cut_off_type(slot: &Slot, begin: usize, end: usize) -> CutOffType {
        if slot.begin >= begin && slot.begin < end {
            return CutOffType::CUTSTART;
        }
        if slot.begin < begin && slot.end > end {
            return CutOffType::CUTMIDDLE;
        }
        if slot.begin < begin && slot.end > begin {
            return CutOffType::CUTEND;
        }
        if slot.begin >= begin && slot.end <= end {
            return CutOffType::CUTWHOLE;
        }
        CutOffType::NOCUT
    }

    pub fn print_slots_for_range(&self, start: usize, finish: usize) -> () {
        for slot in self.slots.iter() {
            if slot.end > start && slot.begin < finish {
                print!["found for {}..{}: {:#?}\n", start, finish, slot];
            }
        }
    }

    fn find_least_overlap_interval_for_task(&self, task_id: usize) -> (usize, usize) {
        print!("Finding least overlap interval for task_id:{}\n", task_id);
        let mut lowest_overlap_so_far: usize = usize::MAX - 1;
        let mut slot_begin_with_lowest_overlap: usize = 0;
        for slot in self.slots.iter() {
            if slot.task_id == task_id {
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
                    if overlap < lowest_overlap_so_far {
                        lowest_overlap_so_far = overlap;
                        slot_begin_with_lowest_overlap = slot.begin + slot_offset;
                    }
                }
            }
        }
        (
            slot_begin_with_lowest_overlap,
            slot_begin_with_lowest_overlap + self.tasks[task_id].duration_to_schedule,
        )
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

    fn find_unscheduled_task_id_with_highest_scheduling_possibilities(&self) -> Option<usize> {
        print!("Searching for new task to process...\n");
        let mut result: Option<usize> = None;
        let mut task_id_highest_scheduling_possibilities_prio: usize = 0;
        let mut highest_scheduling_possibilities_so_far: usize = 0;
        for (task_index, task) in self.tasks.iter().enumerate() {
            match task.task_status {
                TaskStatus::SCHEDULED => {
                    continue;
                }
                TaskStatus::UNSCHEDULED => {}
            }
            let mut scheduling_possibilities: usize = 0;
            for slot in self.slots.iter() {
                if slot.task_id == task.task_id {
                    let range: usize = slot.end - slot.begin;
                    print!("range:{}\n", range);
                    print!(
                        "scheduling_possibilities before:{}\n",
                        scheduling_possibilities
                    );
                    scheduling_possibilities += range - task.duration_to_schedule + 1;
                    print!(
                        "scheduling_possibilities after:{}\n",
                        scheduling_possibilities
                    );
                }
            }
            if scheduling_possibilities > highest_scheduling_possibilities_so_far {
                print![
                    "Found task {} with scheduling_possibilities {}...higher than previous task {} with {}\n",
                    task_index, scheduling_possibilities, task_id_highest_scheduling_possibilities_prio, highest_scheduling_possibilities_so_far
                    ];
                task_id_highest_scheduling_possibilities_prio = task_index;
                highest_scheduling_possibilities_so_far = scheduling_possibilities;
                result = Some(task_id_highest_scheduling_possibilities_prio)
            }
        }
        result
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
    use crate::CutOffType;
    use crate::Goal;
    use crate::GoalType;
    use crate::Slot;
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
        calendar.print_slots_for_range(0, 42);
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

        calendar.print_slots_for_range(12, 14);
        print!("Calendar:{:#?}\n", calendar);
    }

    #[test]
    fn test_find_cut_off_type() {
        let slot = Slot {
            task_id: 0,
            begin: 10,
            end: 15,
        };
        let cut_off = Calendar::find_cut_off_type(&slot, 0, 12);
        assert_eq!(cut_off, CutOffType::CUTSTART);

        let cut_off = Calendar::find_cut_off_type(&slot, 10, 12);
        assert_eq!(cut_off, CutOffType::CUTSTART);

        let cut_off = Calendar::find_cut_off_type(&slot, 11, 12);
        assert_eq!(cut_off, CutOffType::CUTMIDDLE);

        let cut_off = Calendar::find_cut_off_type(&slot, 11, 15);
        assert_eq!(cut_off, CutOffType::CUTEND);

        let cut_off = Calendar::find_cut_off_type(&slot, 11, 20);
        assert_eq!(cut_off, CutOffType::CUTEND);

        let cut_off = Calendar::find_cut_off_type(&slot, 20, 22);
        assert_eq!(cut_off, CutOffType::NOCUT);
    }
}
