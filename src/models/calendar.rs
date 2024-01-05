use super::activity::Activity;
use super::task::{DayTasks, FinalTasks, Task};
use chrono::{Days, Duration, NaiveDateTime, Timelike};
use std::fmt::{Debug, Formatter};
use std::ops::{Add, Sub};
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
        let mut hours = Vec::with_capacity(24 + 48); // TODO: Fix magic number offset everywhere in code
        for _ in 0..hours.capacity() {
            hours.push(Rc::new(Hour::Free));
        }
        Self {
            start_date_time,
            end_date_time,
            hours,
        }
    }

    pub fn get_index_of(&self, date_time: NaiveDateTime) -> usize {
        if date_time < self.start_date_time.sub(Duration::hours(24))
            || date_time >= self.end_date_time.add(Duration::hours(24))
        {
            // TODO: Fix magic number offset everywhere in code
            panic!(
                "can't request an index outside of calendar bounds for date {:?}\nCalendar starts at {:?}", self.start_date_time,
                date_time
            )
        }
        (date_time - self.start_date_time.checked_sub_days(Days::new(1)).unwrap()).num_hours()
            as usize
    }

    pub fn get_tasks(&self, activities: Vec<Activity>) -> FinalTasks {
        //TODO Fix this mess below - it works somehow but not readable at all...
        let mut scheduled: Vec<DayTasks> = vec![];
        let mut impossible: Vec<DayTasks> = vec![];
        let mut day_tasks = DayTasks {
            day: self.start_date_time.date(),
            tasks: Vec::with_capacity(1),
        };
        impossible.push(day_tasks.clone());
        let mut task_counter = 0 as usize;
        let mut current_task = Task {
            taskid: task_counter,
            goalid: "free".to_string(),
            title: "free".to_string(),
            duration: 0,
            start: self.start_date_time.clone(),
            deadline: self.start_date_time.clone(), //just for init; will be overwritten
        };
        for hour_offset in 24..(self.hours.capacity() - 24) {
            if hour_offset % 24 == 0 && hour_offset != 24 {
                // day boundary reached
                println!("found day boundary at offset :{:?}", hour_offset);
                // - push current to dayTasks and increase counter
                current_task.deadline = current_task
                    .start
                    .add(Duration::hours(current_task.duration as i64));
                if current_task.duration > 0 {
                    day_tasks.tasks.push(current_task.clone());
                    task_counter += 1;
                }
                // - push dayTasks copy to scheduled
                scheduled.push(day_tasks);
                // - update dayTasks for current day and reset Tasks vec
                day_tasks = DayTasks {
                    day: self.start_date_time.date(),
                    tasks: Vec::with_capacity(1),
                };
                // - reset current_task and empty title to force new Task in loop
                current_task.title = "".to_string();
            }
            match *self.hours[hour_offset] {
                Hour::Free => {
                    if current_task.title.eq(&"free".to_string()) {
                        current_task.duration += 1;
                    } else {
                        current_task.deadline = current_task
                            .start
                            .add(Duration::hours(current_task.duration as i64));
                        if current_task.duration > 0 {
                            day_tasks.tasks.push(current_task.clone());
                            task_counter += 1;
                        }
                        current_task.title = "free".to_string();
                        current_task.goalid = "free".to_string();
                        current_task.duration = 1;
                        current_task.start = self
                            .start_date_time
                            .add(Duration::hours(hour_offset as i64 - 24)); // TODO: Fix magic number offset everywhere in code
                        current_task.taskid = task_counter;
                    }
                }
                Hour::Occupied { activity_index } => {
                    if current_task.title.eq(&"free".to_string())
                        || current_task.title.ne(&activities[activity_index].title)
                    {
                        if current_task.duration > 0 {
                            current_task.deadline = current_task
                                .start
                                .add(Duration::hours(current_task.duration as i64));
                            // TODO is this necessary?
                            day_tasks.tasks.push(current_task.clone());
                            task_counter += 1;
                        }
                        current_task.duration = 1;
                        current_task.goalid = activities[activity_index].id.clone();
                        current_task.title = activities[activity_index].title.clone();
                        current_task.start = self
                            .start_date_time
                            .add(Duration::hours(hour_offset as i64 - 24)); // TODO: Fix magic number offset everywhere in code
                        current_task.taskid = task_counter;
                    } else {
                        current_task.duration += 1;
                    }
                }
            }
        }
        current_task.deadline = current_task
            .start
            .add(Duration::hours(current_task.duration as i64));
        if current_task.duration > 0 {
            // TODO is this necessary?
            day_tasks.tasks.push(current_task);
        }
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
