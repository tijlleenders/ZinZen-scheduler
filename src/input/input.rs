use crate::models::goal::Goal;
use chrono::NaiveDateTime;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Input {
    start_date: NaiveDateTime,
    end_date: NaiveDateTime,
    goals: Vec<Goal>,
}
