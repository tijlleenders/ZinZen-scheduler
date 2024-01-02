use crate::models::{activity::Activity, calendar::Calendar, goal::Goal};

pub fn generate_activities(calendar: &Calendar, goals: Vec<Goal>) -> Vec<Activity> {
    dbg!(&goals);
    vec![]
}
