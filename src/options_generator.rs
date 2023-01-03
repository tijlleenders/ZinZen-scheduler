use crate::{
    task::{ScheduleOption, Task},
    Slot,
};
use std::vec;

pub fn options_generator(mut tasks: Vec<Task>) -> Vec<Task> {
    &dbg!(&tasks);
    for i in 0..tasks.len() {
        if tasks[i].start_deadline_iterator().is_none() {
            continue;
        }
        let start_deadline_iterator = tasks[i].start_deadline_iterator().unwrap();
        'outer: for desired_time in start_deadline_iterator {
            'inner: for j in 0..tasks.len() {
                if tasks[i].id == tasks[j].id {
                    //don't check for conflicts with itself
                    continue 'inner;
                }
                //check for conflicts with the other task's actually scheduled time
                let slot = Slot {
                    start: tasks[j].confirmed_start.unwrap(),
                    end: tasks[j].confirmed_deadline.unwrap(),
                };
                if desired_time.conflicts_with(&slot) {
                    continue 'outer;
                }
            }
            //if we're here it means no conflicts found for this desired time
            //save it as an option
            let schedule_option = ScheduleOption {
                start: desired_time.start,
                deadline: desired_time.end,
            };
            match &mut tasks[i].options {
                None => {
                    tasks[i].options = Some(vec![schedule_option]);
                }
                Some(o) => {
                    if o.len() < 3 {
                        o.push(schedule_option);
                    }
                }
            }
        }
    }
    tasks
}
