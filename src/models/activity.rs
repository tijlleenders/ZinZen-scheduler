use super::calendar::Calendar;
use super::goal::Goal;
use crate::models::budget::Budget;
use crate::models::calendar::Hour;
use std::rc::{Rc, Weak};
#[derive(Debug)]
pub struct Activity {
    id: String,
    title: String,
    min_block_size: usize,
    max_block_size: usize,
    calendar_overlay: Vec<Option<Weak<Hour>>>,
    budget: Vec<Option<Budget>>,
    total_duration: usize,
    duration_left: usize,
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
        }
    }
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
