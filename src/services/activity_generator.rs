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
            let goal_to_use: &Goal = goals
                .iter()
                .find(|g| g.id.eq(&budget.originating_goal_id))
                .unwrap();
            for time_budget in &budget.time_budgets {
                if time_budget.time_budget_type == TimeBudgetType::Day {
                    if time_budget.scheduled == time_budget.min_scheduled
                        && time_budget.max_scheduled > time_budget.min_scheduled
                    {
                        get_to_week_min_budget_activities.extend(
                            Activity::get_activities_to_get_min_week_budget(
                                goal_to_use,
                                calendar,
                                time_budget,
                            ),
                        );
                    }
                }
            }
        }
    }
    dbg!(&get_to_week_min_budget_activities);
    get_to_week_min_budget_activities
}

pub fn generate_top_up_week_budget_activities(
    calendar: &Calendar,
    goals: &Vec<Goal>,
) -> Vec<Activity> {
    let mut top_up_activities = vec![];
    for budget in &calendar.budgets {
        let goal_to_use: &Goal = goals
            .iter()
            .find(|g| g.id.eq(&budget.originating_goal_id))
            .unwrap();
        for time_budget in &budget.time_budgets {
            if time_budget.time_budget_type == TimeBudgetType::Day {
                if time_budget.min_scheduled < time_budget.max_scheduled
                    && time_budget.scheduled < time_budget.max_scheduled
                {
                    top_up_activities.extend(Activity::get_activities_to_top_up_week_budget(
                        goal_to_use,
                        calendar,
                        time_budget,
                    ));
                }
            }
        }
    }
    dbg!(&top_up_activities);
    top_up_activities
}
