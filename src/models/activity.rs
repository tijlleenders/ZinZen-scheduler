use std::cmp::{max, min};
use std::vec;
use std::{fmt, ops::Add};

use chrono::{Datelike, Days, Duration, NaiveDateTime};
use serde::Deserialize;

use crate::models::activity::ActivityStatus::Impossible;
use crate::models::budget::TimeBudget;
use crate::models::calendar_interval::CalIntStatus;
use crate::models::interval::Interval;
use crate::services::interval_helper;

use super::goal::{Goal, Slot};
use super::{calendar::Calendar, goal::Filter};

#[derive(Clone)]
pub struct Activity {
    pub goal_id: String,
    pub activity_type: ActivityType,
    pub title: String,
    pub min_block_size: usize,
    pub max_block_size: usize,
    pub total_duration: usize,
    pub duration_left: usize,
    pub status: ActivityStatus,
    pub start: NaiveDateTime,
    pub deadline: Option<NaiveDateTime>,
    pub compatible_intervals: Vec<Interval>,
    pub incompatible_intervals: Vec<Interval>,
    pub flex: Option<usize>,
}
impl Activity {
    pub(crate) fn reset_compatible_intervals(&mut self) {
        self.compatible_intervals = vec![];
        self.flex = None;
    }
}
impl Activity {
    pub(crate) fn flex_read_only(&self) -> Option<usize> {
        self.flex
    }
}

impl Activity {
    pub(crate) fn remove_interval(&mut self, interval_to_remove: &Interval) {
        //Attention: also unclaim intervals with calendar separately!
        //This is done when recalculating flex - so suffice to register incompatible ones here
        let mut result: Vec<Interval> = Vec::with_capacity(self.compatible_intervals.len());
        for own_interval in &self.compatible_intervals {
            if interval_to_remove.end <= own_interval.start {
                //no more possible overlaps
                result.push(own_interval.clone());
                continue;
            }
            if interval_to_remove.start >= own_interval.end {
                //no overlap yet
                result.push(own_interval.clone());
                continue;
            }

            let overlap_start = max(interval_to_remove.start, own_interval.start);
            let overlap_end = min(interval_to_remove.end, own_interval.end);

            self.flex = None; //reset flex as this will be impacted by changing compatible intervals

            self.incompatible_intervals.push(Interval {
                //remove this - irrespective if anything unusable left over at beginning and/or end
                //merging this with possible unusable bits gives calendar extra work
                //keeping them separate will align with start/end of cal_intervals
                start: overlap_start,
                end: overlap_end,
            });

            if overlap_start == own_interval.start && overlap_end == own_interval.end {
                //total overlap
                continue;
            }

            //todo: many ifs could be collapsed a bit
            if overlap_start == own_interval.start {
                //something at end left over
                if own_interval.end - overlap_end >= self.min_block_size {
                    //something usable at end left over
                    result.push(Interval {
                        start: overlap_end,
                        end: own_interval.end,
                    });
                } else {
                    //nothing usable left over
                    self.incompatible_intervals.push(Interval {
                        start: overlap_end,
                        end: own_interval.end,
                    });
                }
                continue;
            }
            if overlap_end == own_interval.end {
                //something at beginning left over
                if overlap_start - own_interval.start >= self.min_block_size {
                    //something usable at the beginning left over
                    result.push(Interval {
                        start: own_interval.start,
                        end: overlap_start,
                    });
                } else {
                    //nothing usable left over - don't merge as calendar won't recognize them
                    self.incompatible_intervals.push(Interval {
                        start: own_interval.start,
                        end: overlap_start,
                    });
                }
                continue;
            }
            //only one option left : overlap in middle
            if overlap_start - own_interval.start >= self.min_block_size {
                //something usable at beginning left over
                result.push(Interval {
                    start: own_interval.start,
                    end: overlap_start,
                });
            } else {
                self.incompatible_intervals.push(Interval {
                    start: own_interval.start,
                    end: overlap_start,
                });
            }
            if own_interval.end - overlap_end >= self.min_block_size {
                //something usable at end left over
                result.push(Interval {
                    start: overlap_end,
                    end: own_interval.end,
                });
            } else {
                self.incompatible_intervals.push(Interval {
                    start: overlap_end,
                    end: own_interval.end,
                });
            }
        }
        self.compatible_intervals = result;
        if !self.incompatible_intervals.is_empty() {
            self.flex = None;
        }
    }
}

impl Activity {
    pub(crate) fn flex_reset(&mut self) {
        self.flex = None;
    }
}

impl Activity {
    #[must_use]
    pub fn get_compatible_hours(
        calendar: &Calendar,
        filter_option: &Option<Filter>,
        adjusted_goal_start: NaiveDateTime,
        adjusted_activity_deadline: NaiveDateTime,
        not_on: &Option<Vec<Slot>>,
    ) -> Vec<bool> {
        let mut compatible_hours_overlay: Vec<bool> = Vec::with_capacity(calendar.hours());
        for hour_index in 0..calendar.hours() {
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
            for slot in &not_on {
                if hour_index >= calendar.get_index_of(slot.start)
                    && hour_index < calendar.get_index_of(slot.end)
                {
                    compatible = false;
                }
            }

            if hour_index < calendar.get_index_of(adjusted_goal_start) {
                compatible = false;
            }
            if hour_index >= calendar.get_index_of(adjusted_activity_deadline) {
                compatible = false;
            }

            //check if hour is already occupied by some other activity (for later rounds of scheduling partly occupied calendar)
            //Todo does this need optimization?
            for cal_int in &calendar.intervals {
                if hour_index >= cal_int.interval.start && hour_index < cal_int.interval.end {
                    match cal_int.status {
                        CalIntStatus::Claimable(_) => {
                            break;
                        }
                        CalIntStatus::Occupied(_, _) => {
                            compatible = false;
                            break;
                        }
                    }
                }
            }
            compatible_hours_overlay.push(compatible);
        }
        compatible_hours_overlay
    }

    pub fn mark_impossible(&mut self) {
        self.status = Impossible;
    }

    pub fn flex(&mut self) -> usize {
        if let Some(flex) = self.flex {
            println!("Flex {} from cache.", self.flex.unwrap());
            return flex;
        }

        //assume non-contiguous intervals
        let mut flex: usize = 0;
        for interval in &self.compatible_intervals {
            let interval_size = interval.end - interval.start;

            #[cfg(debug_assertions)]
            assert!(
                self.min_block_size <= interval_size,
                "Placer should remove+unregister INcompatible intervals before calling flex."
            );
            let interval_flex = interval_size - self.min_block_size + 1;
            flex += interval_flex;
        }
        self.flex = Some(flex);
        println!("Flex {} calculated.", flex);
        flex
    }

    pub(crate) fn get_simple_activities(
        goal: &Goal,
        calendar: &mut Calendar,
        duration_of_children: usize,
    ) -> Vec<Activity> {
        let (adjusted_goal_start, adjusted_goal_deadline) = goal.get_adj_start_deadline(calendar);
        let mut activities: Vec<Activity> = Vec::with_capacity(1);

        if let Some(mut activity_total_duration) = goal.min_duration {
            if activity_total_duration > duration_of_children {
                activity_total_duration -= duration_of_children;
            } else {
                panic!(
                    "duration of children goals ({}) exceeds duration {} of goal {} ",
                    duration_of_children, activity_total_duration, goal.title
                );
            }
            let mut min_block_size = activity_total_duration;
            if activity_total_duration > 8 {
                min_block_size = 1;
            };

            let filters_option: Option<Filter> = calendar.get_filters_for(&goal.id);

            let mut adjusted_activity_deadline =
                adjusted_goal_deadline.unwrap_or(calendar.end_date_time); //regular case
            if goal.deadline.is_none() && !calendar.is_participating_in_a_budget(&goal.id) {
                //special case for simple goals without a deadline
                //they are allowed to be scheduled on the 'edge', crossing the calendar week boundary
                adjusted_activity_deadline = adjusted_goal_deadline
                    .unwrap_or(calendar.end_date_time.add(Duration::hours(24)));
            };

            let compatible_intervals: Vec<Interval> = interval_helper::get_compatible_intervals(
                calendar,
                &filters_option,
                adjusted_goal_start,
                adjusted_activity_deadline,
                &goal.not_on.clone(),
            );

            dbg!(&compatible_intervals);
            let mut activity = Activity {
                goal_id: goal.id.clone(),
                activity_type: ActivityType::SimpleGoal,
                title: goal.title.clone(),
                min_block_size,
                max_block_size: min_block_size,
                total_duration: activity_total_duration,
                duration_left: activity_total_duration,
                status: ActivityStatus::Unprocessed,
                start: adjusted_goal_start,
                deadline: goal.deadline,
                compatible_intervals,
                incompatible_intervals: vec![],
                flex: None,
            };
            dbg!(&activity);
            for task in calendar.tasks_completed_today.iter_mut() {
                let mut task_start_index = 0;
                let mut task_end_index = 0;
                if activity.goal_id.eq(&task.goalid) {
                    task_start_index = usize::try_from(
                        (task.start
                            - calendar
                                .start_date_time
                                .checked_sub_days(Days::new(1))
                                .unwrap_or_default())
                        .num_hours(),
                    )
                    .unwrap();
                    task_end_index = usize::try_from(
                        (task.deadline
                            - calendar
                                .start_date_time
                                .checked_sub_days(Days::new(1))
                                .unwrap_or_default())
                        .num_hours(),
                    )
                    .unwrap();

                    for interval in &activity.compatible_intervals {
                        let mut overlap_time = 0;
                        if min(task_end_index, interval.end) > max(task_start_index, interval.start)
                        {
                            overlap_time = max(
                                0,
                                min(task_end_index, interval.end)
                                    - max(task_start_index, interval.start),
                            );
                        }
                        if overlap_time > 0 {
                            if activity.duration_left < overlap_time {
                                activity.duration_left -= overlap_time;
                            } else {
                                activity.duration_left = 0;
                                activity.status = ActivityStatus::Scheduled;
                            }
                        }
                    }
                }
            }
            activities.push(activity);
        }

        activities
    }

    pub(crate) fn get_budget_min_day_activities(
        goal: &Goal,
        calendar: &mut Calendar,
    ) -> Vec<Activity> {
        if goal.filters.as_ref().is_none() {
            return vec![];
        }
        if let Some(config) = &goal.budget_config {
            if config.min_per_day == 0 {
                return vec![];
            }
        }
        let (adjusted_goal_start, adjusted_goal_deadline) = goal.get_adj_start_deadline(calendar);
        let mut activities: Vec<Activity> = Vec::with_capacity(1);

        for day in 0..(adjusted_goal_deadline.unwrap_or(calendar.end_date_time)
            - adjusted_goal_start)
            .num_days() as u64
        {
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

                let compatible_intervals: Vec<Interval> = interval_helper::get_compatible_intervals(
                    calendar,
                    &Some(filter_option.clone()),
                    activity_start,
                    activity_deadline,
                    &goal.not_on.clone(),
                );

                if let Some(config) = &goal.budget_config {
                    //TODO: This is cutting something like Sleep into pieces
                    //Replace by an if on title == 'sleep' / "Sleep" / "Sleep 😴🌙"?
                    //Yes ... but what about translations? => better to match on goal_id
                    let mut adjusted_min_block_size = 1;
                    if goal.title.contains("leep") {
                        adjusted_min_block_size = config.min_per_day;
                    }

                    #[cfg(debug_assertions)]
                    assert!(
                        adjusted_min_block_size <= config.max_per_day,
                        "Assumption broken: We don't check budgets when constructing activities as we assume at starting time budgets will not impact the activities' compatible intervals. This assumption is broken if min_block_size > max_per_day"
                    );

                    let mut activity = Activity {
                        goal_id: goal.id.clone(),
                        activity_type: ActivityType::BudgetMinDay,
                        title: goal.title.clone(),
                        min_block_size: adjusted_min_block_size,
                        max_block_size: config.max_per_day,
                        total_duration: adjusted_min_block_size,
                        duration_left: config.min_per_day,
                        status: ActivityStatus::Unprocessed,
                        start: adjusted_goal_start.add(Duration::days(day as i64)),
                        deadline: Some(adjusted_goal_start.add(Duration::days(day as i64 + 1))),
                        compatible_intervals,
                        incompatible_intervals: vec![],
                        flex: None,
                    };
                    dbg!(&activity);

                    for task in calendar.tasks_completed_today.iter_mut() {
                        let mut task_start_index = 0;
                        let mut task_end_index = 0;
                        if activity.goal_id.eq(&task.goalid) {
                            task_start_index = usize::try_from(
                                (task.start
                                    - calendar
                                        .start_date_time
                                        .checked_sub_days(Days::new(1))
                                        .unwrap_or_default())
                                .num_hours(),
                            )
                            .unwrap();
                            task_end_index = usize::try_from(
                                (task.deadline
                                    - calendar
                                        .start_date_time
                                        .checked_sub_days(Days::new(1))
                                        .unwrap_or_default())
                                .num_hours(),
                            )
                            .unwrap();
                            for interval in &activity.compatible_intervals {
                                let mut overlap_time = 0;
                                if min(task_end_index, interval.end)
                                    > max(task_start_index, interval.start)
                                {
                                    overlap_time = max(
                                        0,
                                        min(task_end_index, interval.end)
                                            - max(task_start_index, interval.start),
                                    );
                                }
                                if overlap_time > 0 {
                                    if activity.duration_left < overlap_time {
                                        activity.duration_left -= overlap_time;
                                    } else {
                                        activity.duration_left = 0;
                                        activity.status = ActivityStatus::Scheduled;
                                    }
                                }
                            }
                        }
                    }
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

        let (adjusted_goal_start, adjusted_goal_deadline) =
            goal_to_use.get_adj_start_deadline(calendar);
        let max_hours = time_budget.max_scheduled - time_budget.scheduled;

        let compatible_intervals: Vec<Interval> = interval_helper::get_compatible_intervals(
            calendar,
            &goal_to_use.filters.clone(),
            adjusted_goal_start,
            adjusted_goal_start.add(Duration::days(7)),
            &goal_to_use.not_on.clone(),
        );

        activities.push(Activity {
            goal_id: goal_to_use.id.clone(),
            activity_type: ActivityType::GetToMinWeekBudget,
            title: goal_to_use.title.clone(),
            min_block_size: 1,
            max_block_size: max_hours,
            total_duration: max_hours,
            duration_left: max_hours,
            status: ActivityStatus::Unprocessed,
            start: adjusted_goal_start,
            deadline: adjusted_goal_deadline,
            compatible_intervals,
            incompatible_intervals: vec![],
            flex: None,
        });

        activities
    }

    pub fn get_activities_to_top_up_week_budget(
        goal_to_use: &Goal,
        calendar: &Calendar,
        time_budget: &TimeBudget,
        max_per_week: usize,
    ) -> Vec<Activity> {
        let mut activities: Vec<Activity> = vec![];

        let max_hours = min(
            time_budget.max_scheduled - time_budget.scheduled,
            max_per_week,
        );

        if max_hours == 0 {
            return activities;
        }

        let adjusted_start = calendar.start_date_time.add(Duration::hours(
            time_budget.calendar_start_index as i64 - 24,
        ));
        let adjusted_end = calendar
            .start_date_time
            .add(Duration::hours(time_budget.calendar_start_index as i64));

        let compatible_intervals: Vec<Interval> = interval_helper::get_compatible_intervals(
            calendar,
            &goal_to_use.filters.clone(),
            adjusted_start,
            adjusted_end,
            &goal_to_use.not_on.clone(),
        );

        activities.push(Activity {
            goal_id: goal_to_use.id.clone(),
            activity_type: ActivityType::TopUpWeekBudget,
            title: goal_to_use.title.clone(),
            min_block_size: 1,
            max_block_size: max_hours,
            total_duration: max_hours,
            duration_left: max_hours,
            status: ActivityStatus::Unprocessed,
            start: adjusted_start,
            deadline: Some(adjusted_end),
            compatible_intervals,
            incompatible_intervals: vec![],
            flex: None,
        });

        activities
    }

    pub(crate) fn get_simple_filler_activities(goal: &Goal, calendar: &Calendar) -> Vec<Activity> {
        //TODO figure out if simple_filler works now - function is not used
        if goal.children.is_none() || goal.filters.as_ref().is_some() {
            return vec![];
        }
        let (adjusted_goal_start, _adjusted_goal_deadline) = goal.get_adj_start_deadline(calendar);
        let mut activities: Vec<Activity> = Vec::with_capacity(1);

        if let Some(activity_total_duration) = goal.min_duration {
            let mut min_block_size = activity_total_duration;
            if activity_total_duration > 8 {
                min_block_size = 1;
                //todo!() //split into multiple activities so flexibilities are correct??
                // or yield flex 1 or maximum of the set from activity.flex()?
            };

            let activity = Activity {
                goal_id: goal.id.clone(),
                activity_type: ActivityType::SimpleFiller,
                title: goal.title.clone(),
                min_block_size,
                max_block_size: min_block_size,
                total_duration: activity_total_duration,
                duration_left: activity_total_duration,
                status: ActivityStatus::Unprocessed,
                start: adjusted_goal_start,
                deadline: goal.deadline,
                compatible_intervals: vec![],
                incompatible_intervals: vec![],
                flex: None,
            };
            dbg!(&activity);
            activities.push(activity);
        }

        activities
    }
}

#[derive(Debug, PartialEq, Clone, Deserialize, Hash)]
pub enum ActivityStatus {
    Unprocessed,
    Processed,
    Scheduled,
    Impossible,
    Postponed,
    BestEffort,
}

#[derive(Clone, Debug, PartialEq, Hash)]
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
        write!(f, "flex:{:?}", self.flex_read_only())?;
        for interval in &self.compatible_intervals {
            write!(f, "\nInterval :{:?}", interval)?;
        }
        Ok(())
    }
}
