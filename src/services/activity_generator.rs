use crate::models::{activity::Activity, calendar::Calendar, goal::Goal};

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
