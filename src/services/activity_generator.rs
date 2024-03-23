use crate::models::{activity::Activity, budget::TimeBudgetType, calendar::Calendar, goal::Goal};
use chrono::Datelike;

pub fn generate_simple_goal_activities(calendar: &Calendar, goals: &[Goal]) -> Vec<Activity> {
    goals
        .iter()
        .flat_map(|goal| Activity::get_simple_activities(goal, calendar))
        .collect::<Vec<_>>()
}

pub fn generate_simple_filler_goal_activities(
    calendar: &Calendar,
    goals: &[Goal],
) -> Vec<Activity> {
    let mut activities = goals
        .iter()
        .flat_map(|goal| Activity::get_simple_filler_activities(goal, calendar))
        .collect::<Vec<_>>();
    for activity in &mut activities {
        if let Some(goal) = goals.iter().find(|g| g.id == activity.goal_id) {
            let children: Vec<&Goal> = goals
                .iter()
                .filter(|child| goal.children.clone().unwrap().contains(&child.id))
                .collect();
            for c in children {
                activity.min_block_size -= c.min_duration.unwrap();
                activity.max_block_size -= c.min_duration.unwrap();
                activity.total_duration -= c.min_duration.unwrap();
                activity.duration_left -= c.min_duration.unwrap();
            }
        }
    }
    activities
}

pub fn generate_budget_goal_activities(calendar: &Calendar, goals: &[Goal]) -> Vec<Activity> {
    goals
        .iter()
        .flat_map(|goal| Activity::get_budget_min_day_activities(goal, calendar))
        .collect::<Vec<_>>()
}

pub fn get_budget_min_week_activities(calendar: &Calendar, goals: &[Goal]) -> Vec<Activity> {
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
    get_to_week_min_budget_activities
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

pub(crate) fn get_base_activities(calendar: &Calendar, goals: &[Goal]) -> Vec<Activity> {
    let mut activities: Vec<Activity> = vec![];
    for goal in goals {
        if goal.children.is_none() {
            if goal.budget_config.is_some() {
                activities.append(&mut Activity::get_budget_min_day_activities(goal, calendar));
            } else {
                activities.append(&mut Activity::get_simple_activities(goal, calendar));
            }
        } else if goal.budget_config.is_none() {
            let mut temp_activities = Activity::get_simple_filler_activities(goal, calendar);
            for activity in &mut temp_activities {
                let children: Vec<&Goal> = goals
                    .iter()
                    .filter(|child| goal.children.clone().unwrap().contains(&child.id))
                    .collect();
                for c in children {
                    activity.min_block_size -= c.min_duration.unwrap();
                    activity.max_block_size -= c.min_duration.unwrap();
                    activity.total_duration -= c.min_duration.unwrap();
                    activity.duration_left -= c.min_duration.unwrap();
                }
            }
            activities.append(&mut temp_activities);
        }
    }
    activities
}

pub(crate) fn generate_activities(calendar: &Calendar, goals: &[Goal]) -> Vec<Activity> {
    //generate and place simple goal activities
    let mut simple_goal_activities = generate_simple_goal_activities(calendar, goals);
    dbg!(&simple_goal_activities);

    let mut simple_goal_activities_without_deadline: Vec<Activity> = vec![];
    for activity in simple_goal_activities.iter() {
        if activity.deadline.year() == 1970 {
            simple_goal_activities_without_deadline.push(activity.clone());
        }
    }
    simple_goal_activities.retain(|a| a.deadline.year() != 1970);

    let simple_filler_activities = generate_simple_filler_goal_activities(calendar, goals);
    dbg!(&simple_filler_activities);

    //generate and place budget goal activities
    let budget_goal_activities: Vec<Activity> = generate_budget_goal_activities(calendar, goals);
    dbg!(&budget_goal_activities);
    dbg!(&calendar);

    [
        simple_goal_activities,
        simple_filler_activities,
        budget_goal_activities,
        simple_goal_activities_without_deadline,
    ]
    .concat()
}
