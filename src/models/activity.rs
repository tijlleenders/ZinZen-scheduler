use chrono::{Datelike, Days, Duration, NaiveDateTime};
use serde::Deserialize;

use super::budget::Budget;
use super::goal::Goal;
use super::{calendar::Calendar, goal::Filters};
use crate::models::budget::TimeBudget;
use crate::models::calendar::Hour;
use std::vec;
use std::{
    fmt,
    ops::{Add, Sub},
    rc::{Rc, Weak},
};

#[derive(Clone)]
pub struct Activity {
    pub goal_id: String,
    pub activity_type: ActivityType,
    pub title: String,
    pub min_block_size: usize,
    pub max_block_size: usize,
    pub calendar_overlay: Vec<Option<Weak<Hour>>>,
    pub time_budgets: Vec<TimeBudget>,
    pub total_duration: usize,
    pub duration_left: usize,
    pub status: Status,
}
impl Activity {
    pub fn get_compatible_hours_overlay(
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
                    if hour_of_day >= filter_option.clone().unwrap().before_time
                        && hour_of_day < filter_option.clone().unwrap().after_time
                    {
                        compatible = false;
                    }
                }
                if filter_option
                    .as_ref()
                    .unwrap()
                    .on_days
                    .contains(&calendar.get_week_day_of(hour_index))
                {
                    // OK
                } else {
                    compatible = false;
                }
            }

            if hour_index < calendar.get_index_of(adjusted_goal_start) {
                compatible = false;
            }
            if hour_index >= calendar.get_index_of(adjusted_goal_deadline) {
                compatible = false;
            }

            //check if hour is already occupied by some other activity (for later rounds of scheduling partly occupied calendar)
            match &*calendar.hours[hour_index] {
                Hour::Free => {}
                Hour::Occupied {
                    activity_index: _,
                    activity_title: _,
                    activity_goalid: _activity_goalid,
                } => {
                    compatible = false;
                }
            }

            if compatible {
                compatible_hours_overlay.push(Some(Rc::downgrade(&calendar.hours[hour_index])));
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
                                // panic!("Does this ever happen?");
                                //      Yes in algorithm_challenge test case
                                //      TODO: do we need to mark all from hour_index till offset as None?"
                                continue;
                            }
                            Some(weak) => {
                                if weak.upgrade().is_none() {
                                    break; // this will reset conflicts too
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
        best_scheduling_index_and_conflicts.map(|(best_index, _)| best_index)
    }

    pub(crate) fn release_claims(&mut self) {
        let mut empty_overlay: Vec<Option<Weak<Hour>>> =
            Vec::with_capacity(self.calendar_overlay.capacity());
        for _ in 0..self.calendar_overlay.capacity() {
            empty_overlay.push(None);
        }
        self.calendar_overlay = empty_overlay;
    }

    pub(crate) fn get_activities_from_budget_goal(
        goal: &Goal,
        calendar: &Calendar,
    ) -> Vec<Activity> {
        if goal.children.is_some() || goal.filters.as_ref().is_none() {
            return vec![];
        }
        let (adjusted_goal_start, adjusted_goal_deadline) = goal.get_adj_start_deadline(calendar);
        let mut activities: Vec<Activity> = Vec::with_capacity(1);
        let filter_option = goal.filters.clone().unwrap();

        //TODO: This is cutting something like Sleep into pieces
        //Replace by an if on title == 'sleep' / "Sleep" / "Sleep 😴🌙"?
        //Yes ... but what about translations? => better to match on goalid
        let number_of_activities = goal.budget_config.as_ref().unwrap().min_per_day;

        for day in 0..(adjusted_goal_deadline - adjusted_goal_start).num_days() as u64 {
            if filter_option
                .on_days
                .contains(&adjusted_goal_start.add(Days::new(day)).weekday())
            {
                // OK
            } else {
                // This day is not allowed
                continue;
            }
            let activity_start = adjusted_goal_start.add(Days::new(day));
            let activity_deadline = adjusted_goal_start.add(Days::new(day + 1));
            for _ in 0..number_of_activities {
                let compatible_hours_overlay = Activity::get_compatible_hours_overlay(
                    calendar,
                    Some(filter_option.clone()),
                    activity_start,
                    activity_deadline,
                );

                let activity = Activity {
                    goal_id: goal.id.clone(),
                    activity_type: ActivityType::Budget,
                    title: goal.title.clone(),
                    min_block_size: 1,
                    max_block_size: 1,
                    calendar_overlay: compatible_hours_overlay,
                    time_budgets: vec![],
                    total_duration: 1,
                    duration_left: 0, //TODO: Correct this - is it even necessary to have duration_left?
                    status: Status::Unprocessed,
                };
                dbg!(&activity);
                activities.push(activity);
            }
        }
        activities
    }

    pub(crate) fn get_activities_from_simple_goal(
        goal: &Goal,
        calendar: &Calendar,
    ) -> Vec<Activity> {
        if goal.children.is_some() || goal.filters.as_ref().is_some() {
            return vec![];
        }
        let (adjusted_goal_start, adjusted_goal_deadline) = goal.get_adj_start_deadline(calendar);
        let mut activities: Vec<Activity> = Vec::with_capacity(1);

        let activity_total_duration = goal.min_duration.unwrap();
        let mut min_block_size = activity_total_duration;
        if activity_total_duration > 8 {
            min_block_size = 1;
            //todo!() //split into multiple activities so flexibilities are correct??
            // or yield flex 1 or maximum of the set from activity.flex()?
        };

        let compatible_hours_overlay = Activity::get_compatible_hours_overlay(
            calendar,
            goal.filters.clone(),
            adjusted_goal_start,
            adjusted_goal_deadline,
        );

        let activity = Activity {
            goal_id: goal.id.clone(),
            activity_type: ActivityType::SimpleGoal,
            title: goal.title.clone(),
            min_block_size,
            max_block_size: min_block_size,
            calendar_overlay: compatible_hours_overlay,
            time_budgets: vec![],
            total_duration: activity_total_duration,
            duration_left: min_block_size, //TODO: Correct this - is it even necessary to have duration_left?
            status: Status::Unprocessed,
        };
        dbg!(&activity);
        activities.push(activity);

        activities
    }

    pub fn update_overlay_with(&mut self, budgets: &Vec<Budget>) {
        if self.status == Status::Scheduled
            || self.status == Status::Impossible
            || self.status == Status::Processed
        {
            //return - no need to update overlay
            return;
        }
        if self.flex() == 0 {
            self.status = Status::Impossible;
            return;
        }
        for budget in budgets {
            //check if activity goal id is in the budget - else don't bother
            if budget.participating_goals.contains(&self.goal_id) {
                // great, process it
            } else {
                // budget not relevant to this activity
                continue;
            }
            //check my overlay for valid placing options, with a loop like get_best_scheduling_index
            for hour_index in 0..self.calendar_overlay.len() {
                match &self.calendar_overlay[hour_index] {
                    None => {
                        continue;
                    }
                    Some(_) => {
                        for offset in 0..self.total_duration {
                            match &self.calendar_overlay[hour_index + offset] {
                                None => {
                                    //empty block found < min_block_size
                                    self.set_overlay_to_none(hour_index);
                                    continue;
                                }
                                Some(weak) => {
                                    if weak.upgrade().is_none() {
                                        //empty block found < min_block_size
                                        self.set_overlay_to_none(hour_index);
                                        break;
                                    }
                                    //if last position check if best so far - or so little we can break
                                    if offset == self.min_block_size - 1 {
                                        //check if not allowed by budgets
                                        let is_allowed = budget.is_within_budget(
                                            hour_index,
                                            offset,
                                            self.activity_type.clone(),
                                        );
                                        if is_allowed {
                                            // Cool!
                                        } else {
                                            self.set_overlay_to_none(hour_index);
                                            //Todo: why not use offset here to set multiple blocks to None?
                                        }
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        if self.flex() == 0 {
            self.status = Status::Impossible;
        }
    }

    fn set_overlay_to_none(&mut self, start_index: usize) {
        for index in start_index..start_index + 1 {
            self.calendar_overlay[index] = None;
        }
    }

    pub fn get_activities_to_get_min_week_budget(
        goal_to_use: &Goal,
        calendar: &Calendar,
        time_budget: &TimeBudget,
    ) -> Vec<Activity> {
        let mut activities: Vec<Activity> = vec![];

        for _ in 0..time_budget.max_scheduled - time_budget.scheduled {
            let compatible_hours_overlay = Activity::get_compatible_hours_overlay(
                calendar,
                goal_to_use.filters.clone(),
                calendar
                    .start_date_time
                    .sub(Duration::hours(24)) //TODO: fix magic number
                    .add(Duration::hours(time_budget.calendar_start_index as i64)),
                calendar
                    .start_date_time
                    .sub(Duration::hours(24)) //TODO: fix magic number
                    .add(Duration::hours(time_budget.calendar_end_index as i64)),
            );
            activities.push(Activity {
                goal_id: goal_to_use.id.clone(),
                activity_type: ActivityType::GetToMinWeekBudget,
                title: goal_to_use.title.clone(),
                min_block_size: 1,
                max_block_size: 1,
                calendar_overlay: compatible_hours_overlay,
                time_budgets: vec![],
                total_duration: 1,
                duration_left: 0,
                status: Status::Unprocessed,
            });
        }

        activities
    }

    pub fn get_activities_to_top_up_week_budget(
        goal_to_use: &Goal,
        calendar: &Calendar,
        time_budget: &TimeBudget,
    ) -> Vec<Activity> {
        let mut activities: Vec<Activity> = vec![];
        for _ in 0..time_budget.max_scheduled - time_budget.scheduled {
            let compatible_hours_overlay = Activity::get_compatible_hours_overlay(
                calendar,
                goal_to_use.filters.clone(),
                calendar
                    .start_date_time
                    .sub(Duration::hours(24)) //TODO: fix magic number
                    .add(Duration::hours(time_budget.calendar_start_index as i64)),
                calendar
                    .start_date_time
                    .sub(Duration::hours(24)) //TODO: fix magic number
                    .add(Duration::hours(time_budget.calendar_end_index as i64)),
            );
            activities.push(Activity {
                goal_id: goal_to_use.id.clone(),
                activity_type: ActivityType::TopUpWeekBudget,
                title: goal_to_use.title.clone(),
                min_block_size: 1,
                max_block_size: 1,
                calendar_overlay: compatible_hours_overlay,
                time_budgets: vec![],
                total_duration: 1,
                duration_left: 0,
                status: Status::Unprocessed,
            });
        }
        activities
    }
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub enum Status {
    Unprocessed,
    Processed,
    Scheduled,
    Impossible,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ActivityType {
    SimpleGoal,
    Budget,
    GetToMinWeekBudget,
    TopUpWeekBudget,
}

impl fmt::Debug for Activity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f).unwrap();
        writeln!(f, "title: {:?}", self.title).unwrap();
        writeln!(f, "status:{:?}", self.status).unwrap();
        writeln!(f, "total duration: {:?}", self.total_duration).unwrap();
        writeln!(f, "duration left: {:?}", self.duration_left).unwrap();
        writeln!(f, "flex:{:?}", self.flex()).unwrap();
        for hour_index in 0..self.calendar_overlay.capacity() {
            let day_index = hour_index / 24;
            let hour_of_day = hour_index % 24;
            match &self.calendar_overlay[hour_index] {
                None => {
                    write!(f, "-").unwrap();
                }
                Some(weak) => {
                    writeln!(
                        f,
                        "day {:?} - hour {:?} at index {:?}: {:?} claims but {:?}",
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
