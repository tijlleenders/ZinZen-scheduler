use chrono::{Datelike, Days, Duration, DurationRound, NaiveDateTime, Timelike};

use super::goal::Goal;
use super::{calendar::Calendar, goal::Filters};
use crate::models::budget::Budget;
use crate::models::calendar::Hour;
use core::num;
use std::{
    f32::consts::LN_10,
    fmt,
    ops::{Add, Sub},
    rc::{Rc, Weak},
};

pub struct Activity {
    pub id: String,
    pub title: String,
    pub min_block_size: usize,
    pub max_block_size: usize,
    pub calendar_overlay: Vec<Option<Weak<Hour>>>,
    pub budget: Option<Budget>,
    pub total_duration: usize,
    pub duration_left: usize,
    pub status: Status,
}
impl Activity {
    pub(crate) fn new_from(goal: Goal, calendar: &Calendar) -> Vec<Activity> {
        if goal.children.is_some() {
            return vec![];
        };
        let mut activities: Vec<Activity> = Vec::with_capacity(1);
        let mut adjusted_goal_start = goal.start;
        if goal.start.year() == 1970 {
            adjusted_goal_start = calendar.start_date_time.add(Duration::days(1));
        }
        let mut adjusted_goal_deadline = goal.deadline;
        if goal.deadline.year() == 1970 {
            adjusted_goal_deadline = calendar.end_date_time.sub(Duration::days(1));
        }
        let filter_option = goal.filters.clone();
        if filter_option.is_some() {
            if filter_option.clone().unwrap().after_time
                < filter_option.clone().unwrap().before_time
            {
                //normal case
            } else {
                // special case where we know that compatible times cross the midnight boundary
                println!(
                    "Special case adjusting start from {:?}",
                    &adjusted_goal_start
                );
                adjusted_goal_start = adjusted_goal_start.sub(Duration::hours(
                    24 - filter_option.clone().unwrap().after_time as i64,
                ));
                println!("... to {:?}", &adjusted_goal_start);
                adjusted_goal_deadline = adjusted_goal_deadline.add(Duration::hours(
                    filter_option.clone().unwrap().before_time as i64,
                ));
            }
        }

        //This is to not cut something like Sleep into pieces
        //Maybe better replaced by an if on title == 'Sleep'?
        //Is the default case that you allow splitting OK?
        let mut activity_total_duration = 1;
        match goal.min_duration.clone() {
            Some(min_duration) => {
                activity_total_duration = min_duration;
            }
            None => {
                activity_total_duration = goal.budget_config.unwrap().min_per_day;
            }
        }
        let mut min_block_size = activity_total_duration;
        if activity_total_duration > 8 {
            min_block_size = 1;
            todo!() //split into multiple activities so flexibilities are correct??
                    // or yield flex 1 or maximum of the set from activity.flex()?
        };

        let mut number_of_activites = 1;
        if filter_option.is_some() {
            number_of_activites =
                (adjusted_goal_deadline - adjusted_goal_start).num_days() as u64 + 1;
            println!("num_activities: {:?}", &number_of_activites);
            adjusted_goal_deadline = adjusted_goal_start.add(Days::new(number_of_activites));
        }
        for _ in 0..number_of_activites {
            let compatible_hours_overlay = Activity::get_compatible_hours_overlay(
                &calendar,
                filter_option.clone(),
                adjusted_goal_start.clone(),
                adjusted_goal_deadline.clone(),
            );

            let activity = Activity {
                id: goal.id.clone(),
                title: goal.title.clone(),
                min_block_size,
                max_block_size: min_block_size,
                calendar_overlay: compatible_hours_overlay,
                budget: None,
                total_duration: activity_total_duration,
                duration_left: min_block_size, //TODO: Correct this - is it even necessary to have duration_left?
                status: Status::Unprocessed,
            };
            dbg!(&activity);
            activities.push(activity);
        }
        activities
    }

    fn get_compatible_hours_overlay(
        calendar: &Calendar,
        filter_option: Option<Filters>,
        adjusted_goal_start: NaiveDateTime,
        adjusted_goal_deadline: NaiveDateTime,
    ) -> Vec<Option<Weak<Hour>>> {
        let mut compatible_hours_overlay: Vec<Option<Weak<Hour>>> =
            Vec::with_capacity(calendar.hours.capacity());
        for hour_index in 0..calendar.hours.capacity() {
            let mut compatible = true;

            if filter_option.is_some() {
                if filter_option.clone().unwrap().after_time
                    < filter_option.clone().unwrap().before_time
                {
                    //normal case
                    let hour_of_day = hour_index % 24;
                    if hour_of_day < filter_option.clone().unwrap().after_time {
                        compatible = false;
                    }
                    if hour_of_day >= filter_option.clone().unwrap().before_time {
                        compatible = false;
                    }
                } else {
                    // special case where we know that compatible times cross the midnight boundary
                    let hour_of_day = hour_index % 24;
                    println!("Hour of day: {:?}", hour_of_day);
                    if hour_of_day >= filter_option.clone().unwrap().before_time
                        && hour_of_day < filter_option.clone().unwrap().after_time
                    {
                        compatible = false;
                    }
                }
            }
            println!("After filters:{:?}", &compatible);

            if hour_index < calendar.get_index_of(adjusted_goal_start) {
                compatible = false;
            }
            if hour_index >= calendar.get_index_of(adjusted_goal_deadline) {
                compatible = false;
            }
            println!("After start/deadline:{:?}", &compatible);

            if compatible == true {
                compatible_hours_overlay
                    .push(Some(Rc::downgrade(&calendar.hours[hour_index as usize])));
            } else {
                compatible_hours_overlay.push(None);
            }
        }
        compatible_hours_overlay
    }

    pub fn flex(&self) -> usize {
        let mut flex = 0;
        let mut buffer = 0;
        for hour_index in 0..self.calendar_overlay.len() {
            match &self.calendar_overlay[hour_index] {
                None => {
                    buffer = 0;
                }
                Some(hour_pointer) => {
                    //if free and buffer size > duration : add to flex and buffer
                    buffer += 1;
                    if hour_pointer.upgrade().is_none() {
                        buffer = 0;
                    } else if hour_pointer.upgrade().unwrap() == Hour::Free.into()
                        && self.min_block_size <= buffer
                    {
                        flex += 1;
                    }
                }
            }
        }
        flex
    }

    pub fn get_best_scheduling_index(&self) -> Option<usize> {
        let mut best_scheduling_index_and_conflicts: Option<(usize, usize)> = None;
        for hour_index in 0..self.calendar_overlay.len() {
            let mut conflicts = 0;
            match &self.calendar_overlay[hour_index] {
                None => {
                    continue;
                }
                Some(_) => {
                    for offset in 0..self.total_duration {
                        match &self.calendar_overlay[hour_index + offset] {
                            None => {
                                continue;
                            }
                            Some(weak) => {
                                if weak.upgrade().is_none() {
                                    conflicts = 0;
                                    break;
                                }
                                conflicts += weak.weak_count();
                                //if last position check if best so far - or so little we can break
                                if offset == self.min_block_size - 1 {
                                    match best_scheduling_index_and_conflicts {
                                        None => {
                                            best_scheduling_index_and_conflicts =
                                                Some((hour_index, conflicts));
                                        }
                                        Some((_, best_conflicts)) => {
                                            if conflicts < best_conflicts || conflicts == 0 {
                                                best_scheduling_index_and_conflicts =
                                                    Some((hour_index, conflicts));
                                            }
                                        }
                                    }
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        }
        match best_scheduling_index_and_conflicts {
            None => return None,
            Some((best_index, _)) => return Some(best_index),
        }
    }

    pub(crate) fn release_claims(&mut self) -> () {
        let mut empty_overlay: Vec<Option<Weak<Hour>>> =
            Vec::with_capacity(self.calendar_overlay.capacity());
        for _ in 0..self.calendar_overlay.capacity() {
            empty_overlay.push(None);
        }
        self.calendar_overlay = empty_overlay;
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Status {
    Unprocessed,
    Scheduled,
    Impossible,
}

#[derive(Debug)]
enum CalendarFilter {
    StartDateTime,
    Deadline,
    DaysOfTheWeek,
    HoursOfTheDay,
}

#[derive(Debug)]
enum BudgetInput {
    HoursPerDay,
    HoursPerWeek,
}

struct HoursPerDay {
    min_per_day: usize,
    max_per_day: usize,
}

struct HoursPerWeek {
    min_per_week: usize,
    max_per_week: usize,
}

impl fmt::Debug for Activity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n").unwrap();
        write!(f, "title: {:?}\n", self.title).unwrap();
        write!(f, "status:{:?}\n", self.status).unwrap();
        write!(f, "total duration: {:?}\n", self.total_duration).unwrap();
        write!(f, "duration left: {:?}\n", self.duration_left).unwrap();
        write!(f, "flex:{:?}\n", self.flex()).unwrap();
        for hour_index in 0..self.calendar_overlay.capacity() {
            let day_index = hour_index / 24;
            let hour_of_day = hour_index % 24;
            match &self.calendar_overlay[hour_index] {
                None => {
                    write!(f, "-").unwrap();
                }
                Some(weak) => {
                    write!(
                        f,
                        "day {:?} - hour {:?} at index {:?}: {:?} claims but {:?}\n",
                        day_index,
                        hour_of_day,
                        hour_index,
                        weak.weak_count(),
                        weak.upgrade().unwrap()
                    )
                    .unwrap();
                }
            }
        }
        Ok(())
    }
}
