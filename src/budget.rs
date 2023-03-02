use std::collections::HashMap;

use chrono::Weekday;
use serde::Deserialize;

use crate::{slot, task::Task, Slot};

#[derive(Deserialize, Debug, Clone)]
pub struct Budget {
    budgets_per_week: Vec<BudgetPerWeek>,
}

// we don't need the min hours since placer will check if task exceeds the max hours
#[derive(Deserialize, Debug, Clone)]
pub struct BudgetPerWeek {
    max_hours: usize,
    week: Slot,
    schduled: usize,
}

pub fn intersection() {}
impl BudgetPerWeek {
    fn new_from_week() {}
    fn reduce_available_hours(&mut self, filled_hours: usize) {
        self.max_hours -= filled_hours;
    }
}

// impl Budget {
//    fn get_allowed_slot(desired:Slot)->Slot{
//     divive slot/Week ->vec(slot)=week
//     foreach week find the budget/week :
//     - cannot find : continue
//         allowed.push(budgetPer_week_found.get_intersection(desired))
//     -
// }
//}
