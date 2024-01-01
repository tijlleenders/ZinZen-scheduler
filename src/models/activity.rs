use crate::models::calendar::Hour;
use std::rc::Rc;

#[derive(Debug)]
pub struct Activity {
    id: String,
    title: String,
    min_block_size: usize,
    max_block_size: usize,
    calendar_overlay: Vec<Rc<Hour>>,
    budget: Vec<Option<Budget>>,
    total_duration: usize,
    duration_left: usize,
}

#[derive(Debug)]
enum CalendarFilter {
    StartDateTime,
    Deadline,
    DaysOfTheWeek,
    HoursOfTheDay,
}
#[derive(Debug)]
struct Budget {
    calendar_start_index: usize,
    calendar_end_index: usize,
    scheduled: usize,
    minimum: usize,
    maximum: usize,
}

// impl Budget {
//     fn new(node) -> Self {
//         Budget {
//             vec![],
//         }
//     }
// }
#[derive(Debug)]
enum BudgetInput {
    HoursPerDay,
    HoursPerWeek,
}

struct HoursPerDay {
    min_per_day: usize,
    max_per_day: usize,
}

struct HoursPerWeek {
    min_per_week: usize,
    max_per_week: usize,
}
