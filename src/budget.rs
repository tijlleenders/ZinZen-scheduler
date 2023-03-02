use std::collections::HashMap;

use chrono::{Datelike, Days, NaiveDateTime, NaiveTime, Weekday};
use serde::Deserialize;

use crate::{output_formatter::get_calender_days, slot, task::Task, Slot};

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

impl BudgetPerWeek {
    fn new_from_week() {}
    fn reduce_available_hours(&mut self, filled_hours: usize) {
        self.max_hours -= filled_hours;
    }
}

impl Budget {
    fn get_allowed_slot(desired: Slot) -> Slot {
        //     divive slot/Week ->vec(slot)=week
        //     foreach week find the budget/week :
        //     - cannot find : continue
        //         allowed.push(budgetPer_week_found.get_intersection(desired))
        //     -
    }
}

// pub fn slot_per_day(calendar_start: NaiveDateTime, calender_end: NaiveDateTime) -> Vec<Slot> {

// }
pub fn slot_per_week(calendar_start: NaiveDateTime, calender_end: NaiveDateTime) -> Vec<Slot> {
    let calendar_days = get_calender_days(
        calendar_start.checked_add_days(Days::new(1)).unwrap(),
        calender_end,
    );
    let mut weeks = vec![];
    let t = NaiveTime::from_hms_milli_opt(00, 00, 00, 00).unwrap();
    let mut start = calendar_start;
    for day in calendar_days.iter() {
        if day.weekday() == Weekday::Mon {
            weeks.push(Slot {
                start,
                end: day.and_time(t),
            });
            start = day.and_time(t);
        }
    }
    if calender_end.weekday() != Weekday::Mon {
        let start = weeks
            .last()
            .unwrap_or(&Slot {
                start,
                end: calender_end,
            })
            .end;
        let end = calender_end;
        weeks.push(Slot { start, end });
    }
    weeks
}
