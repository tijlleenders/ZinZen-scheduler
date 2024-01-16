use std::ops::{Add, Sub};

use chrono::Duration;

use crate::models::{
    activity::{Activity, ActivityType, Status},
    budget::TimeBudgetType,
    calendar::Calendar,
    goal::Goal,
};

pub fn generate_simple_goal_activities(calendar: &Calendar, goals: &Vec<Goal>) -> Vec<Activity> {
    dbg!(&goals);
    let mut activities: Vec<Activity> = Vec::with_capacity(goals.capacity());
    for goal in goals {
        let mut goal_activities = Activity::get_activities_from_simple_goal(goal, calendar);
        dbg!(&goal_activities);
        activities.append(&mut goal_activities);
    }
    activities
}

pub fn generate_budget_goal_activities(calendar: &Calendar, goals: &Vec<Goal>) -> Vec<Activity> {
    dbg!(&goals);
    let mut activities: Vec<Activity> = Vec::with_capacity(goals.capacity());
    for goal in goals {
        let mut goal_activities = Activity::get_activities_from_budget_goal(goal, calendar);
        dbg!(&goal_activities);
        activities.append(&mut goal_activities);
    }
    activities
}

pub fn generate_get_to_week_min_budget_activities(
    calendar: &Calendar,
    goals: &Vec<Goal>,
) -> Vec<Activity> {
    //TODO: check if min/week has been reached for all budgets
    //          If not, for the days where min/day was reached AND there is room till max,
    //              make get_to_week_min_budget activities for that difference
    //              (for example, 'hobby project' and 'family time' in default_budgets test case)
    let mut get_to_week_min_budget_activities = vec![];
    for budget in &calendar.budgets {
        let mut is_min_week_reached = true;
        for time_budget in &budget.time_budgets {
            if time_budget.time_budget_type == TimeBudgetType::Week { //TODO: Assuming only one week time_budget per budget - need to make multi-week compatilble
                 // Good
            } else {
                continue;
            }
            if time_budget.scheduled < time_budget.min_scheduled {
                is_min_week_reached = false;
            }
        }
        if is_min_week_reached {
            //Fine
            continue;
        } else {
            for time_budget in &budget.time_budgets {
                if time_budget.time_budget_type == TimeBudgetType::Day {
                    if time_budget.scheduled == time_budget.min_scheduled
                        && time_budget.max_scheduled > time_budget.min_scheduled
                    {
                        let goal_to_use: &Goal = goals
                            .iter()
                            .find(|g| g.id.eq(&budget.originating_goal_id))
                            .unwrap();

                        //push get_to_min_budget activity to vec

                        //TODO : make this a function on Activity
                        let compatible_hours_overlay = Activity::get_compatible_hours_overlay(
                            &calendar,
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
                        get_to_week_min_budget_activities.push(Activity {
                            goal_id: goal_to_use.id.clone(),
                            activity_type: ActivityType::GetToMinWeekBudget,
                            title: goal_to_use.title.clone(),
                            min_block_size: 1,
                            max_block_size: 1,
                            calendar_overlay: compatible_hours_overlay,
                            time_budgets: vec![],
                            total_duration: 1, //TODO: iterate to make time_budget.max_scheduled - time_budget.min_scheduled activities,
                            duration_left: 0,
                            status: Status::Unprocessed,
                        })
                    }
                }
            }
        }
    }
    get_to_week_min_budget_activities
}
