use crate::models::{activity::Activity, budget::TimeBudgetType, calendar::Calendar, goal::Goal};
use std::collections::BTreeMap;

pub fn add_budget_min_week_activities(
    calendar: &Calendar,
    goals: &BTreeMap<String, Goal>,
    activities: &mut Vec<Activity>,
) {
    let mut get_to_week_min_budget_activities = vec![];
    for budget in &calendar.budgets {
        //TODO Simplify this loop - don't need the if/else
        let mut is_min_week_reached = true;
        for time_budget in &budget.time_budgets {
            if time_budget.time_budget_type != TimeBudgetType::Week {
                //TODO: Assuming only one week time_budget per budget - need to make multi-week compatilble
                //not good
                continue;
            }
            if time_budget.scheduled < time_budget.min_scheduled {
                is_min_week_reached = false;
            }
        }
        if !is_min_week_reached {
            let goal_to_use: &Goal = goals
                .values()
                .find(|g| g.id.eq(&budget.originating_goal_id))
                .unwrap();
            for time_budget in &budget.time_budgets {
                if time_budget.time_budget_type == TimeBudgetType::Week
                    && time_budget.scheduled < time_budget.min_scheduled
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
    activities.extend(get_to_week_min_budget_activities);
}

pub fn add_budget_top_up_week_activities(
    calendar: &Calendar,
    goals: &BTreeMap<String, Goal>,
    activities: &mut Vec<Activity>,
) {
    let mut top_up_activities = vec![];
    for budget in &calendar.budgets {
        if let Some(goal_to_use) = goals
            .values()
            .find(|g| g.id.eq(&budget.originating_goal_id))
        {
            let mut max_per_week = 0;
            for time_budget in &budget.time_budgets {
                if time_budget.time_budget_type == TimeBudgetType::Week
                    && time_budget.scheduled != time_budget.max_scheduled
                {
                    max_per_week = time_budget.max_scheduled;
                }
            }
            for time_budget in &budget.time_budgets {
                if time_budget.time_budget_type == TimeBudgetType::Day
                    && time_budget.min_scheduled < time_budget.max_scheduled
                    && time_budget.scheduled < time_budget.max_scheduled
                {
                    top_up_activities.extend(Activity::get_activities_to_top_up_week_budget(
                        goal_to_use,
                        calendar,
                        time_budget,
                        max_per_week,
                    ));
                }
            }
        }
    }
    dbg!(&top_up_activities);
    activities.extend(top_up_activities);
}

pub(crate) fn add_simple_activities(
    calendar: &Calendar,
    goals: &BTreeMap<String, Goal>,
    activities: &mut Vec<Activity>,
) {
    let mut simple_activities = vec![];
    for goal in goals.values() {
        let activity_duration = goal.min_duration;
        if activity_duration.is_none() {
            continue;
        };

        let mut duration_of_children: usize = 0;
        match &goal.children {
            None => {}
            Some(children) => {
                //figure out how many to subtract
                //todo make recursive as children might also have children - messing up the total to subtract - if even any
                for child in children {
                    if goals.get(child).unwrap().children.is_some() {
                        todo!()
                    };
                    if let Some(child_duration) = goals.get(child).unwrap().min_duration {
                        duration_of_children += child_duration;
                    }
                }
            }
        }
        simple_activities.extend(Activity::get_simple_activities(
            goal,
            calendar,
            duration_of_children,
        ));
    }
    activities.extend(simple_activities);
}

pub(crate) fn add_budget_min_day_activities(
    calendar: &Calendar,
    goals: &BTreeMap<String, Goal>,
    activities: &mut Vec<Activity>,
) {
    let mut min_day_activities = vec![];
    for goal in goals.values() {
        min_day_activities.extend(Activity::get_budget_min_day_activities(goal, calendar));
    }
    activities.extend(min_day_activities);
}
