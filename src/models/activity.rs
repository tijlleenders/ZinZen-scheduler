use std::cmp::{max, min};
use std::vec;
use std::{fmt, ops::Add};

use chrono::{Duration, NaiveDateTime};
use serde::Deserialize;

use crate::models::activity::ActivityStatus::Impossible;
use crate::models::budget::TimeBudget;
use crate::models::calendar_interval::CalIntStatus;
use crate::models::interval::Interval;
use crate::services::interval_helper;

use super::goal::Goal;
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
            if goal.children.is_some() {
                if activity_total_duration > duration_of_children {
                    //then the activity from this goal is a filler activity
                    activity_total_duration -= duration_of_children;
                } else {
                    panic!(
                        "duration of children goals ({}) exceeds duration {} of goal {} ",
                        duration_of_children, activity_total_duration, goal.title
                    );
                }
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

            let mut already_placed_for_goal_id: usize = 0;
            for cal_interval in &calendar.intervals {
                match &cal_interval.status {
                    CalIntStatus::Claimable(_) => {}
                    CalIntStatus::Occupied(_, goal_id) => {
                        if goal.id.eq(goal_id) {
                            already_placed_for_goal_id +=
                                cal_interval.interval.end - cal_interval.interval.start;
                        }
                    }
                }
            }

            if already_placed_for_goal_id >= activity_total_duration {
                return vec![];
            }
            activity_total_duration -= already_placed_for_goal_id;

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
            activities.push(activity);
        }

        activities
    }

    pub(crate) fn get_activities_to_get_to_min_day_budget(
        goal_to_use: &Goal,
        calendar: &Calendar,
        time_budget: &TimeBudget,
    ) -> Vec<Activity> {
        let mut activities: Vec<Activity> = Vec::with_capacity(1);
        let (adjusted_goal_start, adjusted_goal_deadline) =
            goal_to_use.get_adj_start_deadline(calendar);

        let hours_to_schedule = time_budget.min_scheduled - time_budget.scheduled;

        let compatible_intervals: Vec<Interval> = interval_helper::get_compatible_intervals(
            calendar,
            &goal_to_use.filters.clone(),
            adjusted_goal_start,
            adjusted_goal_deadline.unwrap_or(adjusted_goal_start.add(Duration::hours(24))),
            &goal_to_use.not_on.clone(),
        );

        activities.push(Activity {
            goal_id: goal_to_use.id.clone(),
            activity_type: ActivityType::GetToMinDayBudget,
            title: goal_to_use.title.clone(),
            min_block_size: 1,
            max_block_size: hours_to_schedule,
            total_duration: hours_to_schedule,
            duration_left: hours_to_schedule,
            status: ActivityStatus::Unprocessed,
            start: adjusted_goal_start,
            deadline: adjusted_goal_deadline,
            compatible_intervals,
            incompatible_intervals: vec![],
            flex: None,
        });

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
        //The time_budget is of type day - so we need to find room left till max_scheduled for that day
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
    GetToMinDayBudget,
    GetToMinWeekBudget,
    TopUpWeekBudget,
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
