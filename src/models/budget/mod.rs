use std::collections::HashMap;

use chrono::NaiveDateTime;
use serde::Deserialize;

use super::slot::Slot;

pub mod impls;

/// Keeps track of the min and max time allowed and scheduled per time period for a collection of Increments/Tasks.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Budget {
    pub budget_type: BudgetType,
    pub min: Option<usize>,
    pub max: Option<usize>,
}

/// weekly or daily
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum BudgetType {
    Weekly,
    Daily,
}

#[derive(Debug, Deserialize)] //Todo deserialize not needed as this is not in input, only TaskBudget is
pub struct TaskBudgets {
    pub calendar_start: NaiveDateTime,
    pub calendar_end: NaiveDateTime,
    pub goal_id_to_budget_ids: HashMap<String, Vec<String>>,
    pub budget_id_to_budget: HashMap<String, TaskBudget>,
}

#[derive(Debug, Deserialize)]
pub struct TaskBudget {
    task_budget_type: BudgetType,
    pub slot_budgets: Vec<SlotBudget>,
    pub min: Option<usize>, //only needed once, can't remove as used for subsequent SlotBudget initialization?
    max: Option<usize>, //only needed once, can't remove as used for subsequent SlotBudget initialization?
}

#[derive(Debug, Deserialize)]
pub struct SlotBudget {
    pub slot: Slot,
    pub min: Option<usize>,
    pub max: Option<usize>,
    pub used: usize,
}
