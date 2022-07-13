// consider https://crates.io/crates/serde-wasm-bindgen
use std::{fmt, usize};

use serde::{Deserialize, Serialize};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use web_sys::console;

use crate::preprocessor::ProcessedInput;

mod preprocessor;

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
pub fn load_calendar(val: &JsValue) -> JsValue {
    let mut calendar: Calendar = val.into_serde().unwrap();
    console::log_2(&"Called load_calendar with:".into(), &val);
    calendar.schedule();
    JsValue::from_serde(&calendar).unwrap()
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
enum TaskStatus {
    #[default]
    UNSCHEDULED,
    SCHEDULED,
    IMPOSSIBLE,
    WAITING,
}

#[derive(Debug, PartialEq)]
pub enum CutOffType {
    NOCUT,
    CUTSTART,
    CUTEND,
    CUTMIDDLE,
    CUTWHOLE,
}

#[derive(Serialize, Deserialize, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Slot {
    task_id: usize,
    begin: usize,
    end: usize,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Calendar {
    pub max_time_units: usize,
    pub time_unit_qualifier: String,
    pub tasks: Vec<Task>,
    pub slots: Vec<Slot>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct Task {
    task_id: usize,
    duration_to_schedule: usize,
    duration_scheduled: usize,
    task_status: TaskStatus,
    goal_id: String,
}

impl Calendar {
    pub fn new(
        max_time_units: usize,
        time_unit_qualifier: String,
        preprocessor: ProcessedInput,
    ) -> Calendar {
        Calendar {
            max_time_units,
            time_unit_qualifier,
            tasks: preprocessor.tasks,
            slots: preprocessor.slots,
        }
    }

    pub fn schedule(&mut self) -> () {
        #[cfg(not(target_arch = "wasm32"))]
        log::info!("Calendar after loading:{:#?}\n", self);

        loop {
            let unscheduled_task_index_with_highest_scheduling_possibilities: Option<usize> =
                self.find_unscheduled_task_index_with_highest_scheduling_possibilities();
            match unscheduled_task_index_with_highest_scheduling_possibilities {
                Some(task_index_to_schedule) => {
                    let least_overlap_interval: Option<(usize, usize)> = self
                        .find_least_requested_slot_for_task(&self.tasks[task_index_to_schedule]);
                    //log::info!(
                    //     "least overlap for task_id {}:{}-{}\n",
                    //     task_id_to_schedule,
                    //     least_overlap_interval.0,
                    //     least_overlap_interval.1
                    // );
                    match least_overlap_interval {
                        None => {
                            self.slots.retain(|slot| {
                                let delete =
                                    { slot.task_id == self.tasks[task_index_to_schedule].task_id };
                                !delete
                            });
                            self.tasks[task_index_to_schedule].task_status = TaskStatus::IMPOSSIBLE;
                        }
                        Some(interval) => {
                            self.schedule_task(task_index_to_schedule, interval.0, interval.1);
                        }
                    }
                }
                None => break,
            }
        }
        self.slots.sort_by(|a, b| a.begin.cmp(&b.begin));
        #[cfg(not(target_arch = "wasm32"))]
        log::info!("Calendar after scheduling:{:#?}\n", self);
    }

    fn schedule_task(&mut self, task_index: usize, begin: usize, end: usize) -> () {
        let task_id = self.tasks[task_index].task_id;
        #[cfg(not(target_arch = "wasm32"))]
        log::info!("Scheduling task_id {}.\n", task_id);

        #[cfg(not(target_arch = "wasm32"))]
        log::info!("task {:#?}.\n", self.tasks[task_index]);

        //Todo: only remove all slots if duration to be scheduled has been exhausted
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
        self.tasks[task_index].task_status = TaskStatus::SCHEDULED;
        //Todo: if scheduled and 'before' some other task, remove TaskStatus::WAITING from the other tasks that have 'following' attribute
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

    fn find_least_requested_slot_for_task(&self, task: &Task) -> Option<(usize, usize)> {
        #[cfg(not(target_arch = "wasm32"))]
        log::info!(
            "Finding number of requests for possible slot for task_id:{}\n",
            task.task_id
        );
        let mut slot_with_least_requests: Option<(usize, usize)> = None;
        let mut lowest_number_of_requests_for_slot: Option<usize> = None;
        for slot in self.slots.iter() {
            if slot.task_id == task.task_id {
                let num_windows_in_slot =
                    (slot.end - slot.begin + 1).checked_sub(task.duration_to_schedule);
                #[cfg(not(target_arch = "wasm32"))]
                log::info!("num_windows_in_slot:{:#?}\n", num_windows_in_slot);
                match num_windows_in_slot {
                    None => continue,
                    Some(_) => {}
                }

                for slot_offset in 0..num_windows_in_slot.unwrap() {
                    let overlap = self.find_overlap_number_for(
                        slot.begin + slot_offset,
                        slot.begin + slot_offset + task.duration_to_schedule,
                    );
                    #[cfg(not(target_arch = "wasm32"))]
                    log::info!(
                        "# overlaps for:{}-{}:{}\n",
                        slot.begin + slot_offset,
                        slot.begin + slot_offset + task.duration_to_schedule,
                        overlap
                    );
                    match lowest_number_of_requests_for_slot {
                        None => {
                            lowest_number_of_requests_for_slot = Some(overlap);
                            slot_with_least_requests = Some((
                                slot.begin + slot_offset,
                                slot.begin + slot_offset + task.duration_to_schedule,
                            ))
                        }
                        Some(lowest_overlap) => {
                            if overlap == 1 {
                                return slot_with_least_requests;
                            }
                            if overlap < lowest_overlap {
                                slot_with_least_requests = Some((
                                    slot.begin + slot_offset,
                                    slot.begin + slot_offset + task.duration_to_schedule,
                                ));
                                lowest_number_of_requests_for_slot = Some(overlap);
                            }
                        }
                    }
                }
            }
        }
        slot_with_least_requests
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

    fn find_unscheduled_task_index_with_highest_scheduling_possibilities(&self) -> Option<usize> {
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
                TaskStatus::WAITING => {
                    continue;
                }
                TaskStatus::UNSCHEDULED => {
                    //Todo: skip if following another task
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
            "Calendar:\nTasks:{:#?}\nSlots:{:#?}\n",
            self.tasks, self.slots
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::preprocessor::{preprocessor, Input};

    use super::*;

    fn init_env_logger() {
        #[cfg(not(target_arch = "wasm32"))]
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn DELETE_THIS_TEST_DO_INT_TEST_INSTEAD() {
        let input: Input = serde_json::from_str(
            r#"
{
    "startDate": "2022-01-01T00:00:00Z",
    "endDate": "2022-01-02T00:00:00Z",
    "goals": [
        {
          "goalId": "1",
          "title" : "shopping",
          "duration": 1,
          "start": "2022-01-01T10:00:00Z",
          "deadline": "2022-01-01T13:00:00Z"
        },
        {
          "goalId": "2",
          "title": "dentist",
          "duration": 1,
          "start": "2022-01-01T10:00:00Z",
          "deadline": "2022-01-01T11:00:00Z"
        },
        {
          "goalId": "3",
          "title" : "exercise",
          "duration": 1,
          "start": "2022-01-01T10:00:00Z",
          "deadline": "2022-01-01T18:00:00Z"
        }
    ]
}
        "#,
        )
        .unwrap();

        let pre = preprocessor(input);
        let mut cal = Calendar::new(168, "h".into(), pre);
        cal.schedule();
        dbg!(cal);
    }

    #[test]
    fn at_subtract_overflow_panic() {
        init_env_logger();

        let mut calendar = Calendar {
            max_time_units: 168,
            time_unit_qualifier: String::from("h"),
            tasks: Vec::new(),
            slots: Vec::new(),
        };

        let task1 = Task {
            duration_scheduled: 0,
            duration_to_schedule: 1,
            task_id: 1,
            task_status: TaskStatus::UNSCHEDULED,
            goal_id: "5f39a726-641c-4c1a-aa54-2f28a3847ee8".to_string(),
        };

        let slot1 = Slot {
            task_id: 1,
            begin: 11,
            end: 12,
        };
        let slot2 = Slot {
            task_id: 1,
            begin: 35,
            end: 36,
        };

        calendar.tasks.push(task1);

        calendar.slots.push(slot1);
        calendar.slots.push(slot2);

        #[cfg(not(target_arch = "wasm32"))]
        log::info!("Calendar:{:#?}\n", calendar);

        calendar.schedule();

        #[cfg(not(target_arch = "wasm32"))]
        log::info!("Calendar:{:#?}\n", calendar);

        assert_eq!(168, calendar.max_time_units);
        assert_eq!("h", calendar.time_unit_qualifier);
        let mut s_vec: Vec<Slot> = Vec::new();
        let expected_slot1 = Slot {
            task_id: 1,
            begin: 11,
            end: 12,
        };
        s_vec.push(expected_slot1);
        assert_eq!(s_vec, calendar.slots);
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
