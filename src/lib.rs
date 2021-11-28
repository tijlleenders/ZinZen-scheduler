use serde::{Deserialize, Serialize}; // consider https://crates.io/crates/serde-wasm-bindgen
use std::str::FromStr;
use std::{fmt, usize};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use web_sys::console;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn starts_as_soon_as_module_is_loaded() -> Result<(), JsValue> {
    // print pretty errors in wasm https://github.com/rustwasm/console_error_panic_hook
    // This is not needed for tracing_wasm to work, but it is a common tool for getting proper error line numbers for panics.
    console_error_panic_hook::set_once();

    tracing_wasm::set_as_global_default();

    //will have to debug using web-sys log statements... :/ https://rustwasm.github.io/book/reference/debugging.html#using-a-debugger

    Ok(())
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn fixed_and_daily_goal_combined() -> String {
    console::log_1(&"Hello using web-sys".into());

    let js: JsValue = 4.into();
    console::log_2(&"Logging arbitrary values looks like".into(), &js);

    let mut calendar = Calendar::new(720, String::from("h"));

    let goal = Goal {
        id: 1,
        title: String::from("daily goal"),
        estimated_duration: 1,
        effort_invested: 0,
        start: 0,
        finish: 8760, //one year
        start_time: 12,
        finish_time: 18,
        goal_type: GoalType::DAILY,
    };

    let goal2 = Goal {
        id: 2,
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

    // log::info!("Calendar:{:#?}\n", calendar);

    // log::info!("\nexpect Calendar with two goals not overlapping\n");
    calendar.schedule();

    let serialized_calendar = serde_json::to_string(&calendar).unwrap();
    // console::log_1(&serialized_calendar.into());
    // calendar.print_slots_for_range(12, 14);

    // log::info!("Calendar:{:#?}\n", calendar);
    serialized_calendar
}

#[derive(Serialize, Deserialize, Debug)]
enum TaskStatus {
    UNSCHEDULED,
    SCHEDULED,
    IMPOSSIBLE,
}

#[derive(Debug, PartialEq)]
pub enum CutOffType {
    NOCUT,
    CUTSTART,
    CUTEND,
    CUTMIDDLE,
    CUTWHOLE,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GoalType {
    FIXED,
    DAILY,
    WEEKLY,
    MONTHLY,
    YEARLY,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Slot {
    task_id: usize,
    begin: usize,
    end: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    task_id: usize,
    goal_id: usize,
    duration_to_schedule: usize,
    task_status: TaskStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Calendar {
    pub max_time_units: usize,
    pub time_unit_qualifier: String,
    pub goals: Vec<Goal>,
    pub tasks: Vec<Task>,
    pub slots: Vec<Slot>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Goal {
    pub id: usize,
    pub title: String,
    pub estimated_duration: usize,
    pub effort_invested: usize,
    pub start: usize,
    pub finish: usize,
    pub start_time: u8,
    pub finish_time: u8,
    pub goal_type: GoalType,
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

        #[cfg(not(target_arch = "wasm32"))]
        log::info!("Calendar after loading:{:#?}\n", self);

        loop {
            let unscheduled_task_id_with_highest_scheduling_possibilities: Option<usize> =
                self.find_unscheduled_task_id_with_highest_scheduling_possibilities();

            match unscheduled_task_id_with_highest_scheduling_possibilities {
                Some(task_id_to_schedule) => {
                    let least_overlap_interval: Option<(usize, usize)> =
                        self.find_least_overlap_interval_for_task(task_id_to_schedule);
                    //log::info!(
                    //     "least overlap for task_id {}:{}-{}\n",
                    //     task_id_to_schedule,
                    //     least_overlap_interval.0,
                    //     least_overlap_interval.1
                    // );
                    match least_overlap_interval {
                        None => {
                            self.slots.retain(|slot| {
                                let delete = { slot.task_id == task_id_to_schedule };
                                !delete
                            });
                            self.tasks[task_id_to_schedule].task_status = TaskStatus::IMPOSSIBLE;
                        }
                        Some(interval) => {
                            self.schedule_task(task_id_to_schedule, interval.0, interval.1);
                        }
                    }
                }
                None => break,
            }
        }
    }

    fn schedule_task(&mut self, task_id: usize, begin: usize, end: usize) -> () {
        //log::info!("Scheduling task_id {}.\n", task_id);
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
                    if end != slot.end {
                        new_slots.push(Slot {
                            task_id: slot.task_id,
                            begin: end,
                            end: slot.end,
                        });
                    }
                }
                CutOffType::CUTMIDDLE => {
                    if slot.begin != begin {
                        new_slots.push(Slot {
                            task_id: slot.task_id,
                            begin: slot.begin,
                            end: begin,
                        });
                    }
                    if end != slot.end {
                        new_slots.push(Slot {
                            task_id: slot.task_id,
                            begin: end,
                            end: slot.end,
                        });
                    }
                }
                CutOffType::CUTEND => {
                    if slot.begin != begin {
                        new_slots.push(Slot {
                            task_id: slot.task_id,
                            begin: slot.begin,
                            end: begin,
                        });
                    }
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
        //log::info!(
        //     "Calendar right after scheduling task_id {}:{:#?}\n",
        //     task_id,
        //     self
        // );
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
                // log::info!["found for {}..{}: {:#?}\n", start, finish, slot];
            }
        }
    }

    fn find_least_overlap_interval_for_task(&self, task_id: usize) -> Option<(usize, usize)> {
        #[cfg(not(target_arch = "wasm32"))]
        log::info!("Finding least overlap interval for task_id:{}\n", task_id);
        let mut slot_with_lowest_overlap: Option<(usize, usize)> = None;
        let mut lowest_overlap_so_far: Option<usize> = None;
        for slot in self.slots.iter() {
            if slot.task_id == task_id {
                let num_windows_in_slot = slot
                    .end
                    .checked_sub(slot.begin - self.tasks[task_id].duration_to_schedule + 1);
                #[cfg(not(target_arch = "wasm32"))]
                log::info!("num_windows_in_slot:{:#?}\n", num_windows_in_slot);
                match num_windows_in_slot {
                    None => continue,
                    Some(_) => {}
                }

                for slot_offset in 0..num_windows_in_slot.unwrap() {
                    let overlap = self.find_overlap_number_for(
                        slot.begin + slot_offset,
                        slot.begin + slot_offset + self.tasks[task_id].duration_to_schedule,
                    );
                    #[cfg(not(target_arch = "wasm32"))]
                    log::info!(
                        "# overlaps for:{}-{}:{}\n",
                        slot.begin + slot_offset,
                        slot.begin + slot_offset + self.tasks[task_id].duration_to_schedule,
                        overlap
                    );
                    match lowest_overlap_so_far {
                        None => {
                            lowest_overlap_so_far = Some(overlap);
                            slot_with_lowest_overlap = Some((
                                slot.begin + slot_offset,
                                slot.begin + slot_offset + self.tasks[task_id].duration_to_schedule,
                            ))
                        }
                        Some(lowest_overlap) => {
                            if overlap > lowest_overlap {
                                slot_with_lowest_overlap = Some((
                                    slot.begin + slot_offset,
                                    slot.begin
                                        + slot_offset
                                        + self.tasks[task_id].duration_to_schedule,
                                ));
                                lowest_overlap_so_far = Some(overlap);
                            }
                        }
                    }
                }
            }
        }
        slot_with_lowest_overlap
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
        #[cfg(not(target_arch = "wasm32"))]
        log::info!("Searching for new task to process...\n");
        let mut task_id_highest_scheduling_possibilities_prio: Option<usize> = None;
        let mut highest_scheduling_possibilities_so_far: usize = 0;
        for (task_index, task) in self.tasks.iter().enumerate() {
            match task.task_status {
                TaskStatus::IMPOSSIBLE => {
                    continue;
                }
                TaskStatus::SCHEDULED => {
                    continue;
                }
                TaskStatus::UNSCHEDULED => {
                    let mut scheduling_possibilities: usize = 0;
                    for slot in self.slots.iter() {
                        if slot.task_id == task.task_id {
                            let range: usize = slot.end - slot.begin;

                            #[cfg(not(target_arch = "wasm32"))]
                            log::info!(
                                "scheduling_possibilities before:{}\n",
                                scheduling_possibilities
                            );

                            let num_to_add_option = range.checked_sub(task.duration_to_schedule);
                            match num_to_add_option {
                                Some(num) => scheduling_possibilities += num,
                                None => {
                                    #[cfg(not(target_arch = "wasm32"))]
                                    log::info!(
                                        "task duration cannot be subtracted from slot.end-slot.begin"
                                    )
                                }
                            }

                            #[cfg(not(target_arch = "wasm32"))]
                            log::info!(
                                "scheduling_possibilities after:{}\n",
                                scheduling_possibilities
                            );
                        }
                    }

                    match task_id_highest_scheduling_possibilities_prio {
                        Some(highest_num) => {
                            if scheduling_possibilities > highest_num {
                                #[cfg(not(target_arch = "wasm32"))]
                                log::info![
                                "Found task {} with scheduling_possibilities {}...higher than previous task {:#?} with {:#?}\n",
                                task_index, scheduling_possibilities, task_id_highest_scheduling_possibilities_prio, highest_scheduling_possibilities_so_far
                                ];
                                task_id_highest_scheduling_possibilities_prio = Some(task_index);
                                highest_scheduling_possibilities_so_far = scheduling_possibilities;
                            }
                        }
                        None => {
                            task_id_highest_scheduling_possibilities_prio = Some(task_index);
                            highest_scheduling_possibilities_so_far = scheduling_possibilities;
                        }
                    }
                }
            }
        }
        task_id_highest_scheduling_possibilities_prio
    }

    fn load_tasks_and_slots_from_goals(&mut self) -> () {
        for (goal_index, goal) in self.goals.iter().enumerate() {
            // log::info!("Goal:{:#?}\n", goal);
            match goal.goal_type {
                GoalType::FIXED => {
                    let current_task_counter = self.tasks.len();
                    let task = Task {
                        goal_id: goal.id,
                        duration_to_schedule: goal.estimated_duration - goal.effort_invested,
                        task_id: current_task_counter as usize,
                        task_status: TaskStatus::UNSCHEDULED,
                    };
                    // log::info!("Task:{:#?}\n", task);
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
                    // log::info!("Ignoring all but fixed + daily goal types.");
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
            id: 1,
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
            id: 1,
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
                id: 1,
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
    use super::*;

    fn init_env_logger() {
        #[cfg(not(target_arch = "wasm32"))]
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn create_and_print_calendar() {
        init_env_logger();
        let calendar = Calendar {
            max_time_units: 168,
            time_unit_qualifier: String::from("h"),
            goals: vec![Goal::new()],
            tasks: Vec::new(),
            slots: Vec::new(),
        };
        #[cfg(not(target_arch = "wasm32"))]
        log::info!("Calendar:{:#?}\n", calendar);
    }

    #[test]
    fn find_unscheduled_task_id_with_highest_scheduling_possibilities() {
        init_env_logger();

        let goal = Goal {
            id: 1,
            title: String::from("daily goal"),
            estimated_duration: 1,
            effort_invested: 0,
            start: 0,
            finish: 168,
            start_time: 12,
            finish_time: 13,
            goal_type: GoalType::DAILY,
        };
        let goal2 = Goal {
            id: 2,
            title: String::from("daily imp goal"),
            estimated_duration: 1,
            effort_invested: 0,
            start: 0,
            finish: 168,
            start_time: 12,
            finish_time: 13,
            goal_type: GoalType::DAILY,
        };

        let mut calendar = Calendar::new(168, String::from("h"));
        calendar.add(goal);
        calendar.add(goal2);
        calendar.load_tasks_and_slots_from_goals();

        assert_eq!(0, calendar.slots[0].task_id);
        assert_eq!(12, calendar.slots[0].begin);
        assert_eq!(13, calendar.slots[0].end);

        assert_eq!(13, calendar.slots[13].task_id);
        assert_eq!(156, calendar.slots[13].begin);
        assert_eq!(157, calendar.slots[13].end);

        #[cfg(not(target_arch = "wasm32"))]
        log::info!("Calendar:{:#?}\n", calendar);

        assert_eq!(
            Some(0),
            calendar.find_unscheduled_task_id_with_highest_scheduling_possibilities()
        );
    }

    #[test]
    fn add_goal_to_empty_calendar() {
        init_env_logger();
        let goal = Goal::new();
        let mut calendar = Calendar::new(168, String::from("h"));
        calendar.add(goal);
        // log::info!("\nexpect Calendar with a goal\n");
        // log::info!("Calendar:{:#?}\n", calendar);
    }

    #[test]
    fn add_goal_to_empty_calendar_and_schedule() {
        init_env_logger();
        let goal = Goal::new();
        let mut calendar = Calendar::new(168, String::from("h"));
        calendar.add(goal);
        // log::info!("\nexpect Calendar with a goal\n");
        // log::info!("Calendar:{:#?}\n", calendar);
        calendar.schedule();
        // log::info!("Calendar:{:#?}\n", calendar);
    }

    #[test]
    fn add_daily_goal_to_empty_calendar_and_schedule() {
        init_env_logger();
        let goal = Goal {
            id: 1,
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
        // log::info!("\nexpect Calendar with a goal\n");
        calendar.schedule();
        // log::info!("Calendar:{:#?}\n", calendar);
    }

    #[test]
    fn add_daily_goal_to_empty_calendar_and_schedule_and_query() {
        init_env_logger();
        let goal = Goal {
            id: 1,
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
        // log::info!("\nexpect Calendar with a goal\n");
        calendar.schedule();
        // log::info!("Calendar:{:#?}\n", calendar);
        calendar.print_slots_for_range(0, 42);
    }

    #[test]
    fn possible_and_impossible_goal() {
        init_env_logger();

        let goal = Goal {
            id: 1,
            title: String::from("daily goal"),
            estimated_duration: 1,
            effort_invested: 0,
            start: 0,
            finish: 168,
            start_time: 12,
            finish_time: 13,
            goal_type: GoalType::DAILY,
        };
        let goal2 = Goal {
            id: 2,
            title: String::from("daily imp goal"),
            estimated_duration: 1,
            effort_invested: 0,
            start: 0,
            finish: 168,
            start_time: 12,
            finish_time: 13,
            goal_type: GoalType::DAILY,
        };

        let mut calendar = Calendar::new(168, String::from("h"));
        calendar.add(goal);
        calendar.add(goal2);
        calendar.schedule();

        #[cfg(not(target_arch = "wasm32"))]
        log::info!("Calendar:{:#?}\n", calendar);

        calendar.print_slots_for_range(0, 42);
    }

    #[test]
    fn fixed_and_daily_goal_combined() {
        init_env_logger();
        // RUST_LOG=info cargo test --package zinzen_scheduler --lib -- tests::fixed_and_daily_goal_combined --exact --nocapture
        // RUST_LOG=error cargo test --package zinzen_scheduler --lib -- tests::fixed_and_daily_goal_combined --exact --nocapture

        init_env_logger();

        let mut calendar = Calendar::new(720, String::from("h"));

        let goal = Goal {
            id: 1,
            title: String::from("daily goal"),
            estimated_duration: 1,
            effort_invested: 0,
            start: 0,
            finish: 8760, //one year
            start_time: 12,
            finish_time: 18,
            goal_type: GoalType::DAILY,
        };

        let goal2 = Goal {
            id: 2,
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

        // log::info!("Calendar:{:#?}\n", calendar);

        // log::info!("\nexpect Calendar with two goals not overlapping\n");
        calendar.schedule();

        calendar.print_slots_for_range(12, 14);
        #[cfg(not(target_arch = "wasm32"))]
        log::info!("Calendar:{:#?}\n", calendar);
    }

    #[test]
    fn test_find_cut_off_type() {
        init_env_logger();
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
