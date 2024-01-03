use super::activity::Activity;
use super::task::{DayTasks, FinalTasks, Task};
use chrono::{Duration, NaiveDateTime, Timelike};
use std::fmt::{Debug, Formatter};
use std::ops::Add;
use std::rc::Rc;
use std::thread::current;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Hour {
    Free,
    Occupied { activity_index: usize },
}

pub struct Calendar {
    pub start_date_time: NaiveDateTime,
    pub end_date_time: NaiveDateTime,
    pub hours: Vec<Rc<Hour>>,
}

impl Calendar {
    pub fn new(start_date_time: NaiveDateTime, end_date_time: NaiveDateTime) -> Self {
        let mut hours = Vec::with_capacity(24);
        for _ in 0..24 {
            hours.push(Rc::new(Hour::Free));
        }
        Self {
            start_date_time,
            end_date_time,
            hours,
        }
    }

    pub fn get_tasks(&self, activities: Vec<Activity>) -> FinalTasks {
        let mut scheduled: Vec<DayTasks> = vec![];
        let mut impossible: Vec<DayTasks> = vec![];
        let starting_hour = self.start_date_time.hour() as usize;
        let mut day_tasks = DayTasks {
            day: self.start_date_time.date(),
            tasks: Vec::with_capacity(1),
        };
        let mut task_counter = 0 as usize;
        let mut current_task = Task {
            taskid: task_counter,
            goalid: "free".to_string(),
            title: "free".to_string(),
            duration: 0,
            start: self.start_date_time.clone(),
            deadline: self.start_date_time.clone(),
        };
        for hour_offset in 0..self.hours.capacity() {
            match *self.hours[starting_hour + hour_offset] {
                Hour::Free => {
                    if current_task.title.eq(&"free".to_string()) {
                        current_task.duration += 1;
                        current_task.goalid = "free".to_string();
                        current_task.deadline = current_task
                            .start
                            .add(Duration::hours(current_task.duration as i64));
                    } else {
                        current_task.deadline = current_task
                            .start
                            .add(Duration::hours(current_task.duration as i64));
                        day_tasks.tasks.push(current_task.clone());
                        current_task.title = "free".to_string();
                        current_task.duration = 1;
                        current_task.start = self
                            .start_date_time
                            .add(Duration::hours(hour_offset as i64));
                        task_counter += 1;
                        current_task.taskid = task_counter;
                    }
                }
                Hour::Occupied { activity_index } => {
                    if current_task.title.eq(&"free".to_string())
                        || current_task.title.ne(&activities[activity_index].title)
                    {
                        current_task.deadline = current_task
                            .start
                            .add(Duration::hours(current_task.duration as i64));
                        day_tasks.tasks.push(current_task.clone());
                        current_task.duration = 1;
                        current_task.goalid = activities[activity_index].id.clone();
                        current_task.title = activities[activity_index].title.clone();
                        current_task.start = self
                            .start_date_time
                            .add(Duration::hours(hour_offset as i64));
                        task_counter += 1;
                        current_task.taskid = task_counter;
                    } else {
                        current_task.duration += 1;
                    }
                }
            }
        }
        day_tasks.tasks.push(current_task);
        scheduled.push(day_tasks);
        FinalTasks {
            scheduled: scheduled,
            impossible: impossible,
        }
    }
}

impl Debug for Calendar {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        for index in 0..self.hours.capacity() {
            if self.hours[index] == Rc::new(Hour::Free) {
                if Rc::weak_count(&self.hours[index]) == 0 {
                    write!(f, "{} -\n", index).unwrap();
                } else {
                    write!(
                        f,
                        "{} {:?} claims\n",
                        index,
                        Rc::weak_count(&self.hours[index])
                    )
                    .unwrap();
                }
            } else {
                write!(f, "{} {:?}\n", index, self.hours[index]).unwrap();
            }
        }
        Ok(())
    }
}
