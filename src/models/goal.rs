use chrono::NaiveDateTime;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Goal {
    pub id: String,
    pub start: NaiveDateTime,
    pub deadline: NaiveDateTime,
    pub filters: Filters,
    pub min_duration: usize,
    pub title: String,
}

#[derive(Deserialize, Debug)]
pub struct Filters {
    pub after_time: usize,
    pub before_time: usize,
}
