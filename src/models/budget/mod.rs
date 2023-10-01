use std::collections::HashMap;

use chrono::NaiveDateTime;
use serde::Deserialize;

use super::slot::Slot;

pub mod impls;

/// Keeps track of the min and max time allowed and scheduled per time period for a collection of Steps/Tasks.
#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
pub struct Budget {
    pub budget_type: BudgetType,
    pub min: Option<usize>,
    pub max: Option<usize>,
}

/// weekly or daily
#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
pub enum BudgetType {
    Weekly,
    Daily,
}

#[derive(Debug, Deserialize)] //Todo deserialize not needed as this is not in input, only TaskBudget is
pub struct StepBudgets {
    pub calendar_start: NaiveDateTime,
    pub calendar_end: NaiveDateTime,
    /// A map from goal IDs to a vector of budget IDs associated with that goal
    pub budget_ids_map: HashMap<String, Vec<String>>,
    /// A map from goal IDs to the `StepBudget` objects associated with that goal.
    pub budget_map: HashMap<String, Vec<StepBudget>>,
}

#[derive(Debug, Deserialize)]
pub struct StepBudget {
    pub(crate) step_budget_type: BudgetType,
    pub slot_budgets: Vec<SlotBudget>,
    pub min: Option<usize>, //only needed once, can't remove as used for subsequent SlotBudget initialization?
    pub(crate) max: Option<usize>, //only needed once, can't remove as used for subsequent SlotBudget initialization?
}

#[derive(Debug, Deserialize)]
pub struct SlotBudget {
    pub slot: Slot,
    pub min: Option<usize>,
    pub max: Option<usize>,
    pub used: usize,
}
