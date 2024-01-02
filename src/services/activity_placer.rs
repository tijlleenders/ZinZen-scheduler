use crate::models::{activity::Activity, calendar::Calendar, goal::Goal};

pub fn place(calendar: Calendar, activities: Vec<Activity>) -> () {
    for activity in activities {
        dbg!(&activity);
    }
    ()
}
