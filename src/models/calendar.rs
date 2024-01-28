use super::budget::{get_time_budgets_from, Budget, TimeBudgetType};
use super::goal::Goal;
use super::task::{DayTasks, FinalTasks, Task};
use chrono::{Datelike, Days, Duration, NaiveDateTime, Weekday};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::ops::{Add, Deref, Sub};
use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
pub enum Hour {
    Free,
    Occupied {
        activity_index: usize,
        activity_title: String,
        activity_goalid: String,
    }, //TODO: add goal id and budget id to occupied registration so budget object is not necessary anymore!
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ImpossibleActivity {
    pub id: String,
    pub hours_missing: usize,
    pub period_start_date_time: NaiveDateTime,
    pub period_end_date_time: NaiveDateTime,
}

pub struct Calendar {
    pub start_date_time: NaiveDateTime,
    pub end_date_time: NaiveDateTime,
    pub hours: Vec<Rc<Hour>>,
    pub impossible_activities: Vec<ImpossibleActivity>,
    pub budgets: Vec<Budget>,
}

impl Calendar {
    pub fn new(start_date_time: NaiveDateTime, end_date_time: NaiveDateTime) -> Self {
        let number_of_days = (end_date_time - start_date_time).num_days(); //Todo use this later to stop limiting compatible
        println!(
            "Calendar of {:?} days, from {:?} to {:?}",
            &number_of_days, &start_date_time, &end_date_time
        );
        let mut hours = Vec::with_capacity(48 + number_of_days as usize * 24);
        for _ in 0..hours.capacity() {
            hours.push(Rc::new(Hour::Free));
        }
        Self {
            start_date_time,
            end_date_time,
            hours,
            impossible_activities: vec![],
            budgets: vec![],
        }
    }

    pub fn get_week_day_of(&self, index_to_test: usize) -> Weekday {
        if index_to_test > self.hours.capacity() - 1 {
            panic!(
                "Can't request weekday for index {:?} outside of calendar capacity {:?}\nIndexes start at 0.\n",
                index_to_test,
                self.hours.capacity()
            );
        }
        let date_time_of_index_to_test = self
            .start_date_time
            .sub(Days::new(1))
            .add(Duration::hours(index_to_test as i64));
        date_time_of_index_to_test.weekday()
    }

    pub fn get_index_of(&self, date_time: NaiveDateTime) -> usize {
        if date_time < self.start_date_time.sub(Duration::days(1))
            || date_time > self.end_date_time.add(Duration::days(1))
        {
            // TODO: Fix magic number offset everywhere in code
            panic!(
                "can't request an index more than 1 day outside of calendar bounds for date {:?}\nCalendar starts at {:?} and ends at {:?}", date_time, self.start_date_time, self.end_date_time
            )
        }
        (date_time
            - self
                .start_date_time
                .checked_sub_days(Days::new(1))
                .unwrap_or_default())
        .num_hours() as usize
    }

    pub fn print(&self) -> FinalTasks {
        //TODO Fix this mess below - it works somehow but not readable at all...
        let mut scheduled: Vec<DayTasks> = vec![];
        let mut day_tasks = DayTasks {
            day: self.start_date_time.date(),
            tasks: Vec::with_capacity(1),
        };
        let mut task_counter = 0;
        let mut current_task = Task {
            taskid: task_counter,
            goalid: "free".to_string(),
            title: "free".to_string(),
            duration: 0,
            start: self.start_date_time,
            deadline: self.start_date_time, //just for init; will be overwritten
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
                }
                task_counter += 1;
                current_task.taskid = task_counter;
                // - push dayTasks copy to scheduled
                scheduled.push(day_tasks);
                // - update dayTasks for current day and reset Tasks vec
                day_tasks = DayTasks {
                    day: self
                        .start_date_time
                        .date()
                        .add(Duration::days(hour_offset as i64 / 24 - 1)),
                    tasks: Vec::with_capacity(1),
                };
                // - reset current_task and empty title to force new Task in loop
                current_task.title = "".to_string();
                current_task.duration = 0;
            }
            match self.hours[hour_offset].clone().deref() {
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
                Hour::Occupied {
                    activity_index: _,
                    activity_title,
                    activity_goalid,
                } => {
                    if current_task.title.eq(&"free".to_string())
                        || current_task.title.ne(activity_title)
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
                        current_task.goalid = activity_goalid.clone();
                        current_task.title = activity_title.clone();
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
            scheduled,
            impossible: self.impossible_activities.clone(),
        }
    }

    pub fn add_budgets_from(&mut self, goals: &[Goal]) {
        //fill goal_map and budget_ids
        let mut goal_map: HashMap<String, Goal> = HashMap::new();
        let mut budget_ids: Vec<String> = vec![];
        for goal in goals {
            goal_map.insert(goal.id.clone(), goal.clone());
            if let Some(budget_config) = &goal.budget_config {
                //Check if budget_config is realistic

                //check 1
                let mut min_per_day_sum = 0;
                if let Some(filters) = &goal.filters {
                    for _ in &filters.on_days {
                        min_per_day_sum += budget_config.min_per_day;
                    }
                }
                if min_per_day_sum > budget_config.min_per_week {
                    panic!(
                        "Sum of min_per_day {:?} is higher than min_per_week {:?} for goal {:?}",
                        min_per_day_sum, budget_config.min_per_week, goal.title
                    );
                }

                //check 2
                if budget_config.max_per_day > budget_config.max_per_week {
                    panic!(
                        "max_per_day {:?} is higher than max_per_week {:?} for goal {:?}",
                        budget_config.max_per_day, budget_config.max_per_week, goal.title
                    );
                }
                budget_ids.push(goal.id.clone());
            }
        }

        for budget_id in budget_ids {
            //TODO: extract in function get_all_descendants
            //get all descendants
            let mut descendants_added: Vec<String> = vec![budget_id.clone()];
            //get the first children if any
            let mut descendants: Vec<String> = vec![];
            if let Some(goal) = goal_map.get(&budget_id) {
                match &goal.children {
                    Some(children) => {
                        descendants.append(children.clone().as_mut());
                    }
                    None => {
                        self.budgets.push(Budget {
                            originating_goal_id: budget_id.clone(),
                            participating_goals: descendants_added,
                            time_budgets: get_time_budgets_from(self, goal),
                        });
                        continue;
                    }
                }
            }

            loop {
                //add children of each descendant until no more found
                if descendants.is_empty() {
                    if let Some(goal) = goal_map.get(&budget_id) {
                        self.budgets.push(Budget {
                            originating_goal_id: budget_id.clone(),
                            participating_goals: descendants_added,
                            time_budgets: get_time_budgets_from(self, goal),
                        });
                        break;
                    }
                }
                if let Some(descendant_of_which_to_add_children) = descendants.pop() {
                    if let Some(goal) = goal_map.get(&descendant_of_which_to_add_children) {
                        if let Some(children) = &goal.children {
                            descendants.extend(children.clone());
                            descendants_added.push(descendant_of_which_to_add_children);
                        }
                    }
                }
            }
        }
    }

    pub fn update_budgets_for(&mut self, goal: &str, duration_offset: usize) {
        let iterator = self.budgets.iter_mut();
        for budget in iterator {
            budget.reduce_for_(goal, duration_offset);
        }
    }

    pub fn log_impossible_min_day_budgets(&mut self) {
        let impossible_activities = self.impossible_activities();
        self.impossible_activities.extend(impossible_activities);
    }

    pub fn log_impossible_min_week_budgets(&mut self) {
        //TODO: merge with log_imossible_min_day_budgets, passing budget type as param
        let impossible_activities = self.impossible_activities();
        self.impossible_activities.extend(impossible_activities);
    }

    fn impossible_activities(&mut self) -> Vec<ImpossibleActivity> {
        let mut impossible_activities = vec![];
        for budget in &self.budgets {
            for time_budget in &budget.time_budgets {
                if time_budget.time_budget_type == TimeBudgetType::Day {
                    // Good
                } else {
                    continue;
                }
                if time_budget.scheduled < time_budget.min_scheduled {
                    impossible_activities.push(ImpossibleActivity {
                        id: budget.originating_goal_id.clone(),
                        hours_missing: time_budget.min_scheduled - time_budget.scheduled,
                        period_start_date_time: self
                            .start_date_time
                            .add(Duration::hours(time_budget.calendar_start_index as i64)),
                        period_end_date_time: self
                            .start_date_time
                            .add(Duration::hours(time_budget.calendar_end_index as i64)),
                    });
                }
            }
        }
        impossible_activities
    }
}
impl Debug for Calendar {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        writeln!(f)?;
        for index in 0..self.hours.capacity() {
            write!(f, "{:?} ", self.get_week_day_of(index))?;
            let mut index_string = index.to_string();
            if index > 23 {
                index_string = index.to_string() + " " + &(index % 24).to_string();
            }
            if self.hours[index] == Rc::new(Hour::Free) {
                if Rc::weak_count(&self.hours[index]) == 0 {
                    writeln!(f, "{} -", index_string)?;
                } else {
                    writeln!(
                        f,
                        "{} {:?} claims",
                        index_string,
                        Rc::weak_count(&self.hours[index])
                    )?;
                }
            } else {
                writeln!(f, "{} {:?}", index_string, self.hours[index])?;
            }
        }
        writeln!(
            f,
            "{:?} impossible activities",
            self.impossible_activities.len()
        )?;
        for budget in &self.budgets {
            writeln!(f, "{:?}", &budget)?;
        }
        Ok(())
    }
}
