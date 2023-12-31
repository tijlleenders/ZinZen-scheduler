use crate::legacy::input::Input;
use crate::legacy::output::{DayTasks, FinalTasks, Task};
use crate::models::goal::Goal;
use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

pub type Goals = Vec<Rc<Goal>>;
pub type Span = usize;
pub type Position = usize;
pub type FlexValue = usize;
pub type Unprocessed = RefCell<Vec<Position>>;
pub type Scheduled = RefCell<Vec<(Position, Rc<Goal>)>>;

pub struct Calendar {
    unprocessed: Unprocessed,

    scheduled: Scheduled,
    impossible: Scheduled,
}

impl Calendar {
    pub fn new(input: &Input, goals: &Goals) -> Self {
        let date_start = &input.calendar_start;
        let date_end = &input.calendar_end;

        let unprocessed: Unprocessed = RefCell::new(vec![]);
        let scheduled = RefCell::new(vec![]);
        let impossible = RefCell::new(vec![]);

        Self {
            unprocessed,
            scheduled,
            impossible,
        }
    }

    pub fn has_finished_scheduling(&self) -> bool {
        self.unprocessed.borrow().is_empty()
    }

    pub fn unprocessed(&self) -> Vec<Position> {
        self.unprocessed.borrow().clone()
    }

    pub fn result(&self) -> () {}
}

impl Debug for Calendar {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Calendar debug output\n")
    }
}
