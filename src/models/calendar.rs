use std::cmp::{max, min, PartialEq};
use std::collections::{BTreeMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::ops::{Add, Sub};
use std::rc::Rc;

use chrono::{Datelike, Days, Duration, NaiveDateTime, Weekday};
use serde::{Deserialize, Serialize};

use crate::models::activity::ActivityStatus::{BestEffort, Impossible, Scheduled};
use crate::models::activity::ActivityType::TopUpWeekBudget;
use crate::models::budget::TimeBudgetType::{Day, Week};
use crate::models::calendar_interval::CalIntStatus::Claimable;
use crate::models::calendar_interval::{CalIntStatus, CalendarInterval};
use crate::models::interval::Interval;

use super::activity::{Activity, ActivityStatus};
use super::budget::{get_time_budgets_from, Budget};
use super::goal::Goal;
use super::task::{DayTasks, FinalTasks, Task};

#[derive(Debug, PartialEq, Clone, Hash)]
pub enum Hour {
    Free,
    Occupied {
        activity_title: String,
        activity_goalid: String,
    }, //TODO: add goal id and budget id to occupied registration so budget object is not necessary anymore!
    Blocked,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ImpossibleActivity {
    pub id: String,
    pub hours_missing: usize,
    pub period_start_date_time: NaiveDateTime,
    pub period_end_date_time: Option<NaiveDateTime>,
}

pub struct Calendar {
    pub start_date_time: NaiveDateTime,
    pub end_date_time: NaiveDateTime,
    pub impossible_activities: Vec<ImpossibleActivity>,
    pub budgets: Vec<Budget>,
    pub intervals: Vec<CalendarInterval>,
    registered_act_index: usize,
}

impl Calendar {
    pub(crate) fn unregister(&mut self, interval: &Interval, act_index: usize) {
        for cal_int in &mut self.intervals {
            //occupied interval could be using multiple cal_ints
            let is_overlapping =
                interval.start < cal_int.interval.end && cal_int.interval.start < interval.end;
            if is_overlapping {
                match cal_int.status {
                    Claimable(ref mut claims) => {
                        claims.remove(&act_index);
                    }
                    CalIntStatus::Occupied(_, _) => {
                        //do nothing, there is no claim to unregister
                    }
                }
            }
        }
    }
}

impl Calendar {
    pub(crate) fn register_activities(&mut self, activities: &[Activity]) {
        for (act_index, activity) in activities.iter().enumerate() {
            if act_index < self.registered_act_index {
                continue;
            }

            for interval in &activity.compatible_intervals {
                println!(
                    "Registering activity {} with act_index {}",
                    activity.title, act_index
                );
                self.register(interval, act_index);
            }
            if activities[act_index].status != BestEffort {
                self.registered_act_index += 1;
            }
        }
    }
}

impl Calendar {
    pub(crate) fn reduce_budgets_for(
        &mut self,
        goal_id: &str,
        cal_index_start: usize,
        cal_index_end: usize,
    ) {
        //Budgets are usually placed block by block (min_block=1) - except for Sleep
        for budget in &mut self.budgets {
            budget.reduce_for_(goal_id, cal_index_start, cal_index_end);
        }
    }
}

impl Calendar {
    pub(crate) fn update_compatible_intervals(&self, activity: &mut Activity) {
        //check to see if still ok according to budgets
        if activity.status == Scheduled
            || activity.status == Impossible
            || activity.status == ActivityStatus::Processed
            || activity.compatible_intervals.is_empty()
        {
            //return - no need to update compatible_ints
            return;
        }

        //check if max_week reached
        for budget in &self.budgets {
            if budget.participating_goals.contains(&activity.goal_id) {
                for time_budget in &budget.time_budgets {
                    if time_budget.time_budget_type == Week
                        && time_budget.max_scheduled == time_budget.scheduled
                    {
                        activity.reset_compatible_intervals();
                        return;
                    }
                    if time_budget.time_budget_type == Day
                        && time_budget.max_scheduled == time_budget.scheduled
                    {
                        activity.remove_interval(&Interval {
                            start: time_budget.calendar_start_index,
                            end: time_budget.calendar_end_index,
                        });
                    }
                }
            }
        }

        let mut intervals_that_cant_fit_in_budget: Vec<Interval> = vec![];
        for act_int in &activity.compatible_intervals {
            for hour_index in act_int.start..act_int.end - activity.min_block_size {
                //is this even useful? Can't we assume they are valid?
                //does cutting leave invalid intervals?
                for offset in 0..activity.min_block_size {
                    //cycle through all relevant budgets and check this position
                    for budget in &self.budgets {
                        if !budget.participating_goals.contains(&activity.goal_id) {
                            continue; //budget not relevant for this activity
                        }
                        for time_budget in &budget.time_budgets {
                            let overlap_with_budget_start =
                                max(hour_index, time_budget.calendar_start_index);
                            let overlap_with_budget_end =
                                min(hour_index + offset, time_budget.calendar_end_index);
                            let budget_left_for_budget_interval =
                                time_budget.max_scheduled - time_budget.scheduled;
                            if overlap_with_budget_end > overlap_with_budget_start
                                && overlap_with_budget_end - overlap_with_budget_start
                                    > budget_left_for_budget_interval
                            {
                                // invalid - so remove_int(only_first_hour_int)
                                // as others might still be valid
                                intervals_that_cant_fit_in_budget.push(Interval {
                                    start: hour_index,
                                    end: hour_index + 1,
                                });
                            }
                        }
                    }
                }
            }
        }
        if !intervals_that_cant_fit_in_budget.is_empty() {
            dbg!(&intervals_that_cant_fit_in_budget);
            for interval in intervals_that_cant_fit_in_budget {
                println!(
                    "Removing interval {}-{} from activity{}",
                    interval.start, interval.end, activity.title
                );
                activity.remove_interval(&interval);
            }
        }
    }
}
impl PartialEq<&Interval> for CalendarInterval {
    fn eq(&self, other: &&Interval) -> bool {
        if self.interval.start == other.start && self.interval.end == other.end {
            return true;
        }
        false
    }
}

impl PartialEq<Interval> for CalendarInterval {
    fn eq(&self, other: &Interval) -> bool {
        if self.interval.start == other.start && self.interval.end == other.end {
            return true;
        }
        false
    }
}

impl Calendar {
    pub(crate) fn occupy(
        &mut self,
        interval: &Interval,
        act_index: usize,
        activities: &mut [Activity],
    ) {
        let mut impacted_act_indexes: HashSet<usize> = HashSet::new();
        for cal_interval in &mut self.intervals {
            let is_overlapping = interval.start < cal_interval.interval.end
                && cal_interval.interval.start < interval.end;
            if is_overlapping {
                #[cfg(debug_assertions)]
                assert!(
                    cal_interval.interval.end <= interval.end && cal_interval.interval.start >= interval.start,
                    "Assumption broken: If cal_interval and interval overlap, cal_interval should always be equal or subset of occupied interval."
                );

                if let Claimable(claims) = &mut cal_interval.status {
                    for act_index_in_claim in claims.iter() {
                        activities[*act_index_in_claim].remove_interval(interval); //will reset flex if incompatible intervals are generated
                        if *act_index_in_claim != act_index {
                            impacted_act_indexes.insert(*act_index_in_claim);
                        }
                    }
                }
                cal_interval.status =
                    CalIntStatus::Occupied(act_index, activities[act_index].goal_id.clone());
            } else if let Claimable(claims) = &mut cal_interval.status {
                if claims.contains(&act_index) && activities[act_index].status == Scheduled {
                    claims.remove(&act_index);
                    for act_index_in_claim in claims.iter() {
                        activities[*act_index_in_claim].flex_reset();
                    }
                }
            }
        }
        for act_index_impacted in &impacted_act_indexes {
            for incompatible_int in &activities[*act_index_impacted].incompatible_intervals {
                self.register(incompatible_int, *act_index_impacted);
                self.unregister(incompatible_int, *act_index_impacted);
            }
            activities[*act_index_impacted].incompatible_intervals = vec![];
        }
    }
}

impl PartialEq<Rc<Activity>> for Activity {
    fn eq(&self, other: &Rc<Activity>) -> bool {
        self.goal_id == other.goal_id
    }
}

impl PartialEq for Activity {
    fn eq(&self, other: &Activity) -> bool {
        self.goal_id == other.goal_id
    }
}

impl Calendar {
    pub(crate) fn register(&mut self, interval: &Interval, act_index: usize) {
        let mut result = self.intervals.clone();
        let mut inner_result = Vec::with_capacity(result.capacity());
        'cal_interval_loop: for cal_interval in &mut result {
            let is_overlapping = interval.start < cal_interval.interval.end
                && cal_interval.interval.start < interval.end;
            if !is_overlapping {
                inner_result.push(cal_interval.clone());
                continue 'cal_interval_loop;
            }
            // we have an overlap!
            let empty_begin = interval.start > cal_interval.interval.start;
            let empty_end = interval.end < cal_interval.interval.end;

            if !empty_begin && !empty_end {
                // total overlap
                // println!(
                //     "total overlap {}-{} while registering.",
                //     interval.start, interval.end
                // );
                cal_interval.claim_by(act_index);
                inner_result.push(cal_interval.clone());
            }
            if !empty_begin && empty_end {
                //overlap begin
                // println!(
                //     "overlap begin {}-{} while registering.",
                //     cal_interval.interval.start, interval.end
                // );
                inner_result.push(cal_interval.clone());
                inner_result.last_mut().unwrap().interval.end = interval.end;
                inner_result.last_mut().unwrap().claim_by(act_index);
                //empty end
                inner_result.push(cal_interval.clone());
                inner_result.last_mut().unwrap().interval.start = interval.end;
                //continue 'act_interval_loop; //since we reached end of overlap already
            }
            if empty_begin && !empty_end {
                //empty begin
                // println!(
                //     "overlap end {}-{} while registering.",
                //     interval.end, cal_interval.interval.end
                // );
                inner_result.push(cal_interval.clone());
                inner_result.last_mut().unwrap().interval.end = interval.start;
                //overlap end
                inner_result.push(cal_interval.clone());
                inner_result.last_mut().unwrap().interval.start = interval.start;
                inner_result.last_mut().unwrap().claim_by(act_index);
            }
            if empty_begin && empty_end {
                // println!(
                //     "overlap middle {}-{} while registering.",
                //     interval.start, interval.end
                // );
                //empty begin
                inner_result.push(cal_interval.clone());
                inner_result.last_mut().unwrap().interval.end = interval.start;
                //overlap end
                inner_result.push(cal_interval.clone());
                inner_result.last_mut().unwrap().interval.start = interval.start;
                inner_result.last_mut().unwrap().interval.end = interval.end;
                inner_result.last_mut().unwrap().claim_by(act_index);
                //empty end
                inner_result.push(cal_interval.clone());
                inner_result.last_mut().unwrap().interval.start = interval.end;
            }
        }
        self.intervals = inner_result.clone();
    }
}

impl PartialEq for CalIntStatus {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Claimable(_) => match other {
                Claimable(_) => return true,
                CalIntStatus::Occupied(_, _) => {}
            },
            CalIntStatus::Occupied(_, goal_id) => match other {
                Claimable(_) => {}
                CalIntStatus::Occupied(_, goal_id2) => {
                    if goal_id.eq(goal_id2) {
                        return true;
                    }
                }
            },
        }
        false
    }
}

impl Calendar {
    pub fn new(start_date_time: NaiveDateTime, end_date_time: NaiveDateTime) -> Self {
        let number_of_days = (end_date_time - start_date_time).num_days(); //Todo use this later to stop limiting compatible
        println!(
            "Calendar of {:?} days, from {:?} to {:?}",
            &number_of_days, &start_date_time, &end_date_time
        );
        let number_of_hours_for_extended_calendar = 48 + number_of_days as usize * 24; // 48 extra for one day of buffer at front and back
        let intervals = vec![CalendarInterval {
            //fragment the first day already so it can be split off easily when printing calendar
            interval: Interval {
                start: 0,
                end: number_of_hours_for_extended_calendar,
            },
            status: Claimable(HashSet::new()),
        }];

        Self {
            start_date_time,
            end_date_time,
            impossible_activities: vec![],
            budgets: vec![],
            intervals,
            registered_act_index: 0,
        }
    }

    pub(crate) fn hours(&self) -> usize {
        self.intervals
            .last()
            .expect("when calling hours there should be at least one interval in calendar.")
            .interval
            .end
    }
    pub fn get_week_day_of(&self, index_to_test: usize) -> Weekday {
        #[cfg(debug_assertions)]
        assert!(index_to_test < self.hours(),
                "Can't request weekday for index {:?} outside of calendar capacity {:?}\nIndexes start at 0.\n",
                index_to_test,
                self.hours()
        );
        let date_time_of_index_to_test = self
            .start_date_time
            .sub(Days::new(1))
            .add(Duration::hours(index_to_test as i64));
        date_time_of_index_to_test.weekday()
    }

    pub fn is_participating_in_a_budget(&self, goal_id: &String) -> bool {
        for budget in &self.budgets {
            if budget.participating_goals.contains(goal_id) {
                return true;
            }
        }
        false
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
        let index = (date_time
            - self
                .start_date_time
                .checked_sub_days(Days::new(1))
                .unwrap_or_default())
        .num_hours() as usize;
        // println!("got index of {}: {}", date_time, index);
        index
    }
    pub fn print_new(&mut self, activities: &Vec<Activity>) -> FinalTasks {
        println!("Printing new calendar:");
        dbg!(&self);
        println!("Now consolidating intervals and splitting on day boundaries...");
        consolidate_intervals_on_goal_id(&mut self.intervals);
        split_intervals_on_day_boundaries(&mut self.intervals);
        dbg!(&self);
        let mut scheduled: Vec<DayTasks> = transform_intervals_to_day_tasks(
            self.intervals.clone(),
            activities,
            self.start_date_time,
        );

        FinalTasks {
            scheduled: scheduled.drain(1..scheduled.len() - 1).collect::<Vec<_>>(), //skip the first leading 24 hours, and last trailing 24 hours
            impossible: self.impossible_activities.clone(),
        }
    }

    pub fn add_budgets_from(&mut self, goal_map: &mut BTreeMap<String, Goal>) {
        //fill goal_map and budget_ids
        let mut budget_ids: Vec<String> = vec![];
        for goal in goal_map.values() {
            if let Some(budget_config) = &goal.budget_config {
                //Check if budget_config is realistic

                //check 1
                let mut min_per_day_sum = 0;
                if let Some(filters) = &goal.filters {
                    for _ in &filters.on_days {
                        min_per_day_sum += budget_config.min_per_day;
                    }
                }
                #[cfg(debug_assertions)]
                assert!(
                    min_per_day_sum <= budget_config.min_per_week,
                    "Sum of min_per_day {:?} is higher than min_per_week {:?} for goal {:?}",
                    min_per_day_sum,
                    budget_config.min_per_week,
                    goal.title
                );

                //check 2
                #[cfg(debug_assertions)]
                assert!(
                    budget_config.max_per_day <= budget_config.max_per_week,
                    "max_per_day {:?} is higher than max_per_week {:?} for goal {:?}",
                    budget_config.max_per_day,
                    budget_config.max_per_week,
                    goal.title
                );
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
                if let Some(children) = &goal.children {
                    descendants.append(children.clone().as_mut());
                } else {
                    self.budgets.push(Budget {
                        originating_goal_id: budget_id.clone(),
                        participating_goals: descendants_added,
                        time_budgets: get_time_budgets_from(self, goal),
                        time_filters: goal.filters.clone().unwrap(),
                    });
                    continue;
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
                            time_filters: goal.filters.clone().unwrap(),
                        });
                        break;
                    }
                }
                if let Some(descendant_of_which_to_add_children) = descendants.pop() {
                    if let Some(goal) = goal_map.get(&descendant_of_which_to_add_children) {
                        if let Some(children) = &goal.children {
                            descendants.extend(children.clone());
                        }
                        descendants_added.push(descendant_of_which_to_add_children);
                    }
                }
            }
        }
    }

    pub fn log_impossible_activities(&mut self, activities: &Vec<Activity>) {
        for budget in &self.budgets {
            for time_budget in &budget.time_budgets {
                if time_budget.time_budget_type != Day {
                    // not good
                    continue;
                }
                if time_budget.scheduled < time_budget.min_scheduled {
                    self.impossible_activities.push(ImpossibleActivity {
                        id: budget.originating_goal_id.clone(),
                        hours_missing: time_budget.min_scheduled - time_budget.scheduled,
                        period_start_date_time: self
                            .start_date_time
                            .add(Duration::hours(time_budget.calendar_start_index as i64)),
                        period_end_date_time: Some(
                            self.start_date_time
                                .add(Duration::hours(time_budget.calendar_end_index as i64)),
                        ),
                    });
                }
            }
        }
        for activity in activities {
            if activity.status == Impossible
                && activity.deadline.is_some()
                && activity.activity_type != TopUpWeekBudget
            {
                self.impossible_activities.push(ImpossibleActivity {
                    id: activity.goal_id.clone(),
                    hours_missing: activity.duration_left,
                    period_start_date_time: activity.start,
                    period_end_date_time: activity.deadline,
                });
            }
        }
    }

    pub(crate) fn get_filters_for(&self, id: &str) -> Option<super::goal::Filters> {
        for budget in &self.budgets {
            if budget.participating_goals.iter().any(|s| s == id) {
                return Some(budget.time_filters.clone());
            }
        }
        None
    }
}

impl Debug for Calendar {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        writeln!(f)?;
        writeln!(
            f,
            "{:?} impossible activities",
            self.impossible_activities.len()
        )?;
        for budget in &self.budgets {
            writeln!(f, "{:?}", &budget)?;
        }
        for interval in &self.intervals {
            writeln!(f, "{:?}", &interval)?;
        }
        Ok(())
    }
}

fn consolidate_intervals_on_goal_id(cal_ints: &mut Vec<CalendarInterval>) {
    if cal_ints.is_empty() {
        return;
    }

    let mut write_index = 0;
    for read_index in 1..cal_ints.len() {
        if cal_ints[read_index].status == cal_ints[write_index].status {
            // Extend the current interval
            cal_ints[write_index].interval.end = cal_ints[read_index].interval.end;
        } else {
            // Move to the next slot and copy the new interval
            write_index += 1;
            cal_ints[write_index] = cal_ints[read_index].clone();
        }
    }

    // Truncate the vector to remove any unused elements
    cal_ints.truncate(write_index + 1);
}

fn split_intervals_on_day_boundaries(intervals: &mut Vec<CalendarInterval>) {
    let mut i = 0;
    while i < intervals.len() {
        let start = intervals[i].interval.start;
        let end = intervals[i].interval.end;
        let first_multiple = ((start + 23) / 24) * 24;

        if first_multiple < end {
            // Split needed
            let mut new_intervals = Vec::new();

            // First part (if exists)
            if start < first_multiple {
                new_intervals.push(CalendarInterval {
                    interval: Interval {
                        start,
                        end: first_multiple,
                    },
                    status: intervals[i].status.clone(),
                });
            }

            // Middle parts
            let mut current = first_multiple;
            while current + 24 < end {
                new_intervals.push(CalendarInterval {
                    interval: Interval {
                        start: current,
                        end: current + 24,
                    },
                    status: intervals[i].status.clone(),
                });
                current += 24;
            }

            // Last part
            new_intervals.push(CalendarInterval {
                interval: Interval {
                    start: current,
                    end,
                },
                status: intervals[i].status.clone(),
            });

            // Replace the original interval with the new splits
            intervals.splice(i..=i, new_intervals.clone());
            i += new_intervals.len();
        } else {
            i += 1;
        }
    }
}

fn transform_intervals_to_day_tasks(
    intervals: Vec<CalendarInterval>,
    activities: &Vec<Activity>,
    calendar_start: NaiveDateTime,
) -> Vec<DayTasks> {
    let mut task_counter: usize = 0;
    let mut day_tasks: Vec<DayTasks> = Vec::new();
    let mut current_day = 0;
    let mut current_day_tasks = Vec::new();

    #[allow(clippy::explicit_counter_loop)]
    for interval in intervals {
        let day_start = (interval.interval.start / 24) * 24;
        let day_end = day_start + 24;

        if day_start > current_day {
            // Start a new day
            if !current_day_tasks.is_empty() {
                day_tasks.push(DayTasks {
                    day: calendar_start
                        .add(Duration::hours(current_day as i64 - 24))
                        .into(),
                    tasks: current_day_tasks,
                });
            }
            current_day = day_start;
            current_day_tasks = Vec::new();
        }

        let start = interval.interval.start % 24;
        let end = min(interval.interval.end, day_end) % 24;
        let duration = if end > start {
            end - start
        } else {
            (24 - start) + end
        };
        let task = Task {
            taskid: task_counter,
            goalid: match interval.status {
                CalIntStatus::Occupied(.., ref goal_id) => goal_id.clone(),
                Claimable(_) => "free".to_string(),
            },
            title: match interval.status {
                CalIntStatus::Occupied(act_index, ..) => activities[act_index].title.clone(),
                Claimable(_) => "free".to_string(),
            },
            duration,
            start: calendar_start.add(Duration::hours(day_start as i64 + start as i64 - 24)),
            deadline: calendar_start.add(Duration::hours(
                day_start as i64 + start as i64 + duration as i64 - 24,
            )),
        };

        if day_start > 0 {
            //don't increment for first (leading) day - as that will be removed anyway
            task_counter += 1;
        }

        current_day_tasks.push(task);

        if interval.interval.end > day_end {
            // Interval spans multiple days, create a new interval for the next day
            let remaining_interval = CalendarInterval {
                interval: Interval {
                    start: day_end,
                    end: interval.interval.end,
                },
                status: interval.status,
            };
            // Recursively process the remaining interval
            day_tasks.extend(transform_intervals_to_day_tasks(
                vec![remaining_interval],
                activities,
                calendar_start,
            ));
        }
    }

    // Add the last day's tasks
    if !current_day_tasks.is_empty() {
        day_tasks.push(DayTasks {
            day: calendar_start
                .add(Duration::hours(current_day as i64))
                .into(),
            tasks: current_day_tasks,
        });
    }

    day_tasks
}
