use crate::models::{activity::Activity, budget::TimeBudgetType, calendar::Calendar, goal::Goal};

pub fn generate_simple_goal_activities(calendar: &Calendar, goals: &[Goal]) -> Vec<Activity> {
    goals
        .iter()
        .flat_map(|goal| {
            Activity::get_activities_from_simple_goal(goal, calendar, goal.get_parent_goal(goals))
        })
        .collect::<Vec<_>>()
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

pub fn adjust_parent_activities(activities: &[Activity], goals: &[Goal]) -> Vec<Activity> {
    let mut activities_to_return = vec![];
    // >>>

    let mut child_activities: Vec<Activity> = vec![];

    // Get activities for parent and child goals
    let mut parent_activities: Vec<Activity> = goals
        .iter()
        .filter_map(|goal| {
            if let Some(children) = &goal.children {
                let activities_for_parent: Vec<Activity> = activities
                    .iter()
                    .filter(|activity| activity.goal_id == goal.id)
                    .cloned()
                    .collect();
                let child_activities_for_parent: Vec<Activity> = children
                    .iter()
                    .filter_map(|child_id| {
                        activities
                            .iter()
                            .find(|activity| activity.goal_id == *child_id)
                            .cloned()
                    })
                    .collect();
                child_activities.extend(child_activities_for_parent);
                return Some(activities_for_parent);
            }
            None
        })
        .flatten()
        .collect();

    if parent_activities.is_empty() {
        return activities.to_owned();
    }

    // For each parent_activity
    parent_activities.iter_mut().for_each(|parent_activity| {
        let mut parent_duraton = parent_activity.total_duration;

        // For each child_activity
        // get total_duration of child goals
        // deduct it from parent_activity.total_duration
        child_activities.iter().for_each(|child_activity| {
            let child_duration = child_activity.total_duration;
            parent_duraton -= child_duration;
        });
        parent_activity.total_duration = parent_duraton;
        parent_activity.min_block_size = parent_duraton;
        parent_activity.max_block_size = parent_duraton;
        parent_activity.duration_left = parent_duraton;
    });

    // Unify parent_activities and child_activities based on Activity.id into single list which is activities_to_return
    activities_to_return.extend(parent_activities);
    activities_to_return.extend(child_activities);

    // Sort activities_to_return like the incoming parameter activities
    // activities_to_return.sort_by_key(|activity| activities.iter().position(|&x| x == activity));

    // <<<
    activities_to_return
}
