use super::calendar::Calendar;
use super::goal::Goal;
use crate::models::budget::Budget;
use crate::models::calendar::Hour;
use std::{
    fmt,
    rc::{Rc, Weak},
};

pub struct Activity {
    id: String,
    title: String,
    min_block_size: usize,
    max_block_size: usize,
    calendar_overlay: Vec<Option<Weak<Hour>>>,
    budget: Vec<Option<Budget>>,
    total_duration: usize,
    duration_left: usize,
    status: Status,
}
impl Activity {
    pub(crate) fn new_from(goal: Goal, calendar: &Calendar) -> Activity {
        let mut compatible_hours_overlay: Vec<Option<Weak<Hour>>> =
            Vec::with_capacity(calendar.hours.capacity());
        for hour in 0..calendar.hours.capacity() {
            //Todo make time filters compatible for multiple days using modulo 24
            if hour < goal.filters.before_time && hour >= goal.filters.after_time {
                compatible_hours_overlay.push(Some(Rc::downgrade(&calendar.hours[hour as usize])));
            //Todo add if for start end of goal filter
            } else {
                compatible_hours_overlay.push(None);
            }
        }

        //This is to not cut something like Sleep into pieces
        //Maybe better replaced by an if on title == 'Sleep'?
        //Is the default case that you allow splitting OK?
        let mut min_block_size = goal.min_duration.clone();
        if goal.min_duration > 8 {
            min_block_size = 1;
            todo!() //split into multiple activities so flexibilities are correct??
                    // or yield flex 1 or maximum of the set from activity.flex()?
        };

        Activity {
            id: goal.id,
            title: goal.title,
            min_block_size,
            max_block_size: goal.min_duration.clone(),
            calendar_overlay: compatible_hours_overlay,
            budget: vec![None],
            total_duration: goal.min_duration.clone(),
            duration_left: goal.min_duration,
            status: Status::Unprocessed,
        }
    }

    fn flex(&self) -> usize {
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
}

#[derive(Debug, PartialEq, Clone)]
enum Status {
    Unprocessed,
    Scheduled,
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
            match &self.calendar_overlay[hour_index] {
                None => (),
                Some(weak) => {
                    write!(
                        f,
                        "hours {:?}: {:?} claims but {:?}\n",
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
