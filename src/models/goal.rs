use chrono::{NaiveDateTime, Weekday};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Goal {
    pub id: String,
    #[serde(default)]
    pub start: NaiveDateTime,
    #[serde(default)]
    pub deadline: NaiveDateTime,
    #[serde(rename = "budget")]
    pub budget_config: Option<BudgetConfig>,
    pub filters: Option<Filters>,
    pub min_duration: Option<usize>,
    pub title: String,
    pub children: Option<Vec<String>>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Filters {
    pub after_time: usize,
    pub before_time: usize,
    pub on_days: Vec<Weekday>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BudgetConfig {
    pub min_per_day: usize,
    pub max_per_day: usize,
    pub min_per_week: usize,
    pub max_per_week: usize,
}
