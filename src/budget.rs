use std::collections::HashMap;

use chrono::Weekday;
use serde::Deserialize;

use crate::{slot, task::Task, Slot};
/// An event type.
#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Event {
    Schedule,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Budget {
    budgets_per_week: Vec<BudgetPerWeek>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct BudgetPerWeek {
    min_hours: usize,
    max_hours: Option<usize>,
    week: Slot,
    schduled: Vec<Slot>,
}

pub fn intersection() {}
impl BudgetPerWeek {
    fn new_from_week() {}
    fn reduce_available_hours(&mut self, filled_hours: usize) {
        self.min_hours -= filled_hours;
        if self.max_hours.is_some() {
            self.max_hours = Some(self.max_hours.unwrap() - filled_hours);
        }
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
