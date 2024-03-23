use chrono::{Datelike, Days, Duration, NaiveDateTime};
use serde::Deserialize;

use super::budget::Budget;
use super::goal::{Goal, Slot};
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
    pub deadline: NaiveDateTime,
}
impl Activity {
    pub fn get_compatible_hours_overlay(
        calendar: &Calendar,
        filter_option: Option<Filters>,
        adjusted_goal_start: NaiveDateTime,
        adjusted_goal_deadline: NaiveDateTime,
        not_on: Option<Vec<Slot>>,
    ) -> Vec<Option<Weak<Hour>>> {
        let mut compatible_hours_overlay: Vec<Option<Weak<Hour>>> =
            Vec::with_capacity(calendar.hours.capacity());
        for hour_index in 0..calendar.hours.capacity() {
            let mut compatible = true;

            if filter_option.is_some() {
                if let Some(filter_option) = filter_option.clone() {
                    let hour_of_day = hour_index % 24;
                    if filter_option.after_time < filter_option.before_time {
                        //normal case
                        if hour_of_day < filter_option.after_time {
                            compatible = false;
                        }
                        if hour_of_day >= filter_option.before_time {
                            compatible = false;
                        }
                    } else {
                        // special case where we know that compatible times cross the midnight boundary
                        if hour_of_day >= filter_option.before_time
                            && hour_of_day < filter_option.after_time
                        {
                            compatible = false;
                        }
                    }
                    if filter_option
                        .on_days
                        .contains(&calendar.get_week_day_of(hour_index))
                    {
                        // OK
                    } else {
                        compatible = false;
                    }
                }
            }

            let not_on = not_on.clone().unwrap_or_default();
            for slot in not_on.iter() {
                if hour_index >= calendar.get_index_of(slot.start)
                    && hour_index < calendar.get_index_of(slot.end)
                {
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
                    } else if let Some(ptr) = hour_pointer.upgrade() {
                        if ptr == Hour::Free.into() && self.min_block_size <= buffer {
                            flex += 1;
                        }
                    }
                }
            }
        }
        flex
    }

    pub fn get_best_scheduling_index_and_length(&self) -> Option<(usize, usize)> {
        let mut best_scheduling_index_and_conflicts: Option<(usize, usize, usize)> = None;
        for hour_index in 0..self.calendar_overlay.len() {
            let mut conflicts = 0;
            if self.calendar_overlay[hour_index].is_some() {
                //TODO: shouldn't this logic be in creating the activity and then set to min_block_size so we can just use that here?
                let offset_size: usize = match self.activity_type {
                    ActivityType::SimpleGoal => self.total_duration,
                    ActivityType::BudgetMinDay => self.min_block_size,
                    ActivityType::GetToMinWeekBudget => 1,
                    ActivityType::TopUpWeekBudget => 1,
                    ActivityType::SimpleFiller => self.total_duration,
                };
                for offset in 0..offset_size {
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
                            if offset == offset_size - 1 {
                                match best_scheduling_index_and_conflicts {
                                    None => {
                                        best_scheduling_index_and_conflicts =
                                            Some((hour_index, conflicts, offset_size));
                                    }
                                    Some((_, best_conflicts, _)) => {
                                        if conflicts < best_conflicts || conflicts == 0 {
                                            best_scheduling_index_and_conflicts =
                                                Some((hour_index, conflicts, offset_size));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        best_scheduling_index_and_conflicts.map(|(best_index, _, size)| (best_index, size))
    }

    pub(crate) fn get_activities_from_simple_goal(
        goal: &Goal,
        calendar: &Calendar,
        parent_goal: Option<Goal>,
    ) -> Vec<Activity> {
        if goal.children.is_some()
            || goal.filters.as_ref().is_some()
            || calendar.is_budget(goal.id.clone())
        {
            return vec![];
        }

        let (adjusted_goal_start, adjusted_goal_deadline) =
            goal.get_adj_start_deadline(calendar, parent_goal);
        let mut activities: Vec<Activity> = Vec::with_capacity(1);

        if let Some(activity_total_duration) = goal.min_duration {
            let mut min_block_size = activity_total_duration;
            if activity_total_duration > 8 {
                min_block_size = 1;
                //todo!() //split into multiple activities so flexibilities are correct??
                // or yield flex 1 or maximum of the set from activity.flex()?
            };

            let filters_option: Option<Filters> = calendar.get_filters_for(goal.id.clone());

            let compatible_hours_overlay = Activity::get_compatible_hours_overlay(
                calendar,
                filters_option,
                adjusted_goal_start,
                adjusted_goal_deadline,
                goal.not_on.clone(),
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
                duration_left: activity_total_duration,
                status: Status::Unprocessed,
                deadline: goal.deadline,
            };
            dbg!(&activity);
            activities.push(activity);
        }

        activities
    }

    pub(crate) fn get_activities_from_budget_goal(
        goal: &Goal,
        calendar: &Calendar,
    ) -> Vec<Activity> {
        if goal.filters.as_ref().is_none() {
            return vec![];
        }
        if let Some(config) = &goal.budget_config {
            if config.min_per_day == 0 {
                return vec![];
            }
        }
        let (adjusted_goal_start, adjusted_goal_deadline) =
            goal.get_adj_start_deadline(calendar, None);
        let mut activities: Vec<Activity> = Vec::with_capacity(1);

        for day in 0..(adjusted_goal_deadline - adjusted_goal_start).num_days() as u64 {
            if let Some(filter_option) = &goal.filters {
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

                let compatible_hours_overlay = Activity::get_compatible_hours_overlay(
                    calendar,
                    Some(filter_option.clone()),
                    activity_start,
                    activity_deadline,
                    goal.not_on.clone(),
                );

                if let Some(config) = &goal.budget_config {
                    //TODO: This is cutting something like Sleep into pieces
                    //Replace by an if on title == 'sleep' / "Sleep" / "Sleep 😴🌙"?
                    //Yes ... but what about translations? => better to match on goalid
                    let mut adjusted_min_block_size = 1;
                    if goal.title.contains("leep") {
                        adjusted_min_block_size = config.min_per_day;
                    }

                    let activity = Activity {
                        goal_id: goal.id.clone(),
                        activity_type: ActivityType::BudgetMinDay,
                        title: goal.title.clone(),
                        min_block_size: adjusted_min_block_size,
                        max_block_size: config.max_per_day,
                        calendar_overlay: compatible_hours_overlay,
                        time_budgets: vec![],
                        total_duration: adjusted_min_block_size,
                        duration_left: config.min_per_day,
                        status: Status::Unprocessed,
                        deadline: goal.deadline,
                    };
                    dbg!(&activity);
                    activities.push(activity);
                }
            }
        }
        activities
    }

    pub fn get_activities_to_get_min_week_budget(
        goal_to_use: &Goal,
        calendar: &Calendar,
        time_budget: &TimeBudget,
    ) -> Vec<Activity> {
        let mut activities: Vec<Activity> = vec![];

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
            goal_to_use.not_on.clone(),
        );
        let max_hours = time_budget.max_scheduled - time_budget.scheduled;

        activities.push(Activity {
            goal_id: goal_to_use.id.clone(),
            activity_type: ActivityType::GetToMinWeekBudget,
            title: goal_to_use.title.clone(),
            min_block_size: 1,
            max_block_size: max_hours,
            calendar_overlay: compatible_hours_overlay,
            time_budgets: vec![],
            total_duration: max_hours,
            duration_left: max_hours,
            status: Status::Unprocessed,
            deadline: goal_to_use.deadline,
        });

        activities
    }

    pub fn get_activities_to_top_up_week_budget(
        goal_to_use: &Goal,
        calendar: &Calendar,
        time_budget: &TimeBudget,
    ) -> Vec<Activity> {
        let mut activities: Vec<Activity> = vec![];

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
            goal_to_use.not_on.clone(),
        );

        let max_hours = time_budget.max_scheduled - time_budget.scheduled;

        activities.push(Activity {
            goal_id: goal_to_use.id.clone(),
            activity_type: ActivityType::TopUpWeekBudget,
            title: goal_to_use.title.clone(),
            min_block_size: 1,
            max_block_size: max_hours,
            calendar_overlay: compatible_hours_overlay,
            time_budgets: vec![],
            total_duration: max_hours,
            duration_left: max_hours,
            status: Status::Unprocessed,
            deadline: goal_to_use.deadline,
        });

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

        //check if block is lost/stolen or not - as current weak pointer state could be disposed/stale/dead
        for hour_index in 0..self.calendar_overlay.len() {
            if let Some(overlay) = &self.calendar_overlay[hour_index] {
                if self.calendar_overlay[hour_index].is_some() && overlay.upgrade().is_none() {
                    //block was stolen/lost to some other activity
                    self.calendar_overlay[hour_index] = None;
                }
            }
        }

        //Check if blocks are too small
        let mut block_size_found: usize = 0;
        for hour_index in 0..self.calendar_overlay.len() {
            match &self.calendar_overlay[hour_index] {
                None => {
                    if block_size_found < self.min_block_size {
                        // found block in calendar that is too small to fit min_block size
                        let mut start_index = hour_index;
                        if hour_index > block_size_found {
                            start_index -= block_size_found;
                        }
                        for index_to_set_to_none in start_index..hour_index {
                            self.calendar_overlay[index_to_set_to_none] = None;
                        }
                    }
                    block_size_found = 0;
                    continue;
                }
                Some(_) => {
                    block_size_found += 1;
                }
            }
        }
        // This is for if we reach the end of the overlay and a block is still building
        if block_size_found < self.min_block_size {
            // found block in calendar that is too small to fit min_block size
            for index_to_set_to_none in
                self.calendar_overlay.len() - block_size_found..self.calendar_overlay.len()
            {
                self.calendar_overlay[index_to_set_to_none] = None;
            }
        }

        //Check if hour is in at least one block that is allowed by all budgets
        let mut is_part_of_at_least_one_valid_block_placing_option: Vec<bool> =
            vec![false; self.calendar_overlay.len()];
        let mut is_activity_part_of_budget = false;
        for budget in budgets {
            //check if activity goal id is in the budget - else don't bother
            if budget.participating_goals.contains(&self.goal_id) {
                // great, process it
                is_activity_part_of_budget = true;
            } else {
                // budget not relevant to this activity
                continue;
            }

            //set hour_option to true for any hour inside a block that satisfies all budgets
            'outer: for index in 0..is_part_of_at_least_one_valid_block_placing_option.len() {
                //check if block under validation is large enough
                for offset in 0..self.min_block_size {
                    if self.calendar_overlay[index + offset].is_none() {
                        continue 'outer;
                    }
                }
                if budget.is_within_budget(index, self.min_block_size, self.activity_type.clone()) {
                    for offset in 0..self.min_block_size {
                        is_part_of_at_least_one_valid_block_placing_option[index + offset] = true;
                    }
                }
            }
        }
        if is_activity_part_of_budget {
            for (index, hour_option) in is_part_of_at_least_one_valid_block_placing_option
                .iter_mut()
                .enumerate()
            {
                if self.calendar_overlay[index].is_some() && !*hour_option {
                    self.calendar_overlay[index] = None;
                }
            }
        }

        if self.flex() == 0 {
            self.status = Status::Impossible;
        }
    }
    pub(crate) fn release_claims(&mut self) {
        let mut empty_overlay: Vec<Option<Weak<Hour>>> =
            Vec::with_capacity(self.calendar_overlay.capacity());
        for _ in 0..self.calendar_overlay.capacity() {
            empty_overlay.push(None);
        }
        self.calendar_overlay = empty_overlay;
    }

    pub(crate) fn get_filler_activities_from_simple_goal(
        goal: &Goal,
        calendar: &Calendar,
        parent_goal: Option<Goal>,
    ) -> Vec<Activity> {
        if goal.children.is_none() || goal.filters.as_ref().is_some() {
            return vec![];
        }
        let (adjusted_goal_start, adjusted_goal_deadline) =
            goal.get_adj_start_deadline(calendar, parent_goal);
        let mut activities: Vec<Activity> = Vec::with_capacity(1);

        if let Some(activity_total_duration) = goal.min_duration {
            let mut min_block_size = activity_total_duration;
            if activity_total_duration > 8 {
                min_block_size = 1;
                //todo!() //split into multiple activities so flexibilities are correct??
                // or yield flex 1 or maximum of the set from activity.flex()?
            };

            let filters_option: Option<Filters> = calendar.get_filters_for(goal.id.clone());

            let compatible_hours_overlay = Activity::get_compatible_hours_overlay(
                calendar,
                filters_option,
                adjusted_goal_start,
                adjusted_goal_deadline,
                goal.not_on.clone(),
            );

            let activity = Activity {
                goal_id: goal.id.clone(),
                activity_type: ActivityType::SimpleFiller,
                title: goal.title.clone(),
                min_block_size,
                max_block_size: min_block_size,
                calendar_overlay: compatible_hours_overlay,
                time_budgets: vec![],
                total_duration: activity_total_duration,
                duration_left: activity_total_duration,
                status: Status::Unprocessed,
                deadline: goal.deadline,
            };
            dbg!(&activity);
            activities.push(activity);
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
    BudgetMinDay,
    GetToMinWeekBudget,
    TopUpWeekBudget,
    SimpleFiller,
}

impl fmt::Debug for Activity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f)?;
        writeln!(f, "title: {:?}", self.title)?;
        writeln!(f, "status:{:?}", self.status)?;
        writeln!(f, "total duration: {:?}", self.total_duration)?;
        writeln!(f, "duration left: {:?}", self.duration_left)?;
        writeln!(f, "flex:{:?}", self.flex())?;
        for hour_index in 0..self.calendar_overlay.capacity() {
            let day_index = hour_index / 24;
            let hour_of_day = hour_index % 24;
            match &self.calendar_overlay[hour_index] {
                None => {
                    write!(f, "-")?;
                }
                Some(weak) => {
                    writeln!(
                        f,
                        "day {:?} - hour {:?} at index {:?}: {:?} claims but {:?}",
                        day_index,
                        hour_of_day,
                        hour_index,
                        weak.weak_count(),
                        weak.upgrade()
                    )?;
                }
            }
        }
        Ok(())
    }
}
