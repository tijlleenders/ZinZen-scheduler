use crate::models::{activity::Activity, calendar::Calendar, goal::Goal};

pub fn generate_activities(calendar: &Calendar, goals: Vec<Goal>) -> Vec<Activity> {
    dbg!(&goals);
    let mut activities: Vec<Activity> = Vec::with_capacity(goals.capacity());
    for goal in goals {
        let activity = Activity::new_from(goal, calendar);
        activities.push(activity);
    }
    activities
}
