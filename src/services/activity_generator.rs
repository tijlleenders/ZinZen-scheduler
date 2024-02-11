use crate::models::{activity::Activity, budget::TimeBudgetType, calendar::Calendar, goal::Goal};
use std::slice::Iter;

pub fn generate_simple_goal_activities(calendar: &Calendar, goals: &[Goal]) -> Vec<Activity> {
    preprocess_goals(goals)
        .iter()
        .flat_map(|goal| Activity::get_activities_from_simple_goal(goal, calendar))
        .collect::<Vec<_>>()
}

pub fn preprocess_goals(goals: &[Goal]) -> Vec<Goal> {
    let mut out = vec![];
    let mut iter = goals.iter();
    while iter.clone().peekable().peek().is_some() {
        let _ = preprocess_goals_rec(&mut out, &mut iter, None);
    }
    out
}
pub fn preprocess_goals_rec(
    new_goals: &mut Vec<Goal>,
    iter: &mut Iter<Goal>,
    parent: Option<&Goal>,
) -> Option<usize> {
    if let Some(goal) = iter.next() {
        let mut goal = goal.clone();
        let mut min_duration = goal.min_duration;
        if let Some(parent) = parent {
            goal.start = parent.start;
            goal.deadline = parent.deadline;
        }
        if let Some(children) = &goal.children {
            for _child in children {
                if let Some(duration) = preprocess_goals_rec(new_goals, iter, Some(&goal)) {
                    min_duration = min_duration
                        .map(|md| md.checked_sub(duration))
                        .unwrap_or_default();
                    goal.min_duration = min_duration
                }
            }
        }
        new_goals.push(goal);
        min_duration
    } else {
        None
    }
}

pub fn generate_budget_goal_activities(calendar: &Calendar, goals: &[Goal]) -> Vec<Activity> {
    goals
        .iter()
        .flat_map(|goal| Activity::get_activities_from_budget_goal(goal, calendar))
        .collect::<Vec<_>>()
}

pub fn generate_get_to_week_min_budget_activities(
    calendar: &Calendar,
    goals: &[Goal],
) -> Option<Vec<Activity>> {
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
                .find(|g| g.id.eq(&budget.originating_goal_id))?;
            for time_budget in &budget.time_budgets {
                if time_budget.time_budget_type == TimeBudgetType::Day
                    && time_budget.scheduled == time_budget.min_scheduled
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
    dbg!(&get_to_week_min_budget_activities);
    Some(get_to_week_min_budget_activities)
}

pub fn generate_top_up_week_budget_activities(
    calendar: &Calendar,
    goals: &[Goal],
) -> Vec<Activity> {
    let mut top_up_activities = vec![];
    for budget in &calendar.budgets {
        if let Some(goal_to_use) = goals.iter().find(|g| g.id.eq(&budget.originating_goal_id)) {
            for time_budget in &budget.time_budgets {
                if time_budget.time_budget_type == TimeBudgetType::Day
                    && time_budget.min_scheduled < time_budget.max_scheduled
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
