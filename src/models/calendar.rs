use crate::legacy::input::Input;
use crate::legacy::output::{DayTasks, FinalTasks, Task};
use crate::models::date::{DateTime, DateTimeRange};
use crate::models::day::Day;
use crate::models::goal::Goal;
use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

pub type Goals = Vec<Rc<Goal>>;
pub type Span = usize;
pub type Position = usize;
pub type FlexValue = usize;
pub type Unprocessed = RefCell<Vec<Position>>;
pub type Scheduled = RefCell<Vec<(Position, DateTimeRange, Rc<Goal>)>>;

pub struct Calendar {
    day: DateTime,

    unprocessed: Unprocessed,

    scheduled: Scheduled,
    impossible: Scheduled,
}

impl Calendar {
    pub fn new(input: &Input, goals: &Goals) -> Self {
        let date_start = DateTime::from_naive_date_time(&input.calendar_start);
        let date_end = DateTime::from_naive_date_time(&input.calendar_end);
        let day = date_start.start_of_day();

        let unprocessed: Unprocessed = RefCell::new(vec![]);
        let scheduled = RefCell::new(vec![]);
        let impossible = RefCell::new(vec![]);

        Self {
            day,

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

    pub fn result(&self) -> FinalTasks {
        let mut tasks = vec![];
        self.gather_tasks_with_filler(&mut tasks, &self.scheduled, false);
        let mut impossible_tasks = vec![];
        self.gather_tasks(&mut impossible_tasks, &self.impossible, true);

        FinalTasks {
            scheduled: vec![DayTasks {
                day: self.day.naive_date(),
                tasks,
            }],
            impossible: vec![DayTasks {
                day: self.day.naive_date(),
                tasks: impossible_tasks,
            }],
        }
    }

    fn gather_tasks(&self, tasks: &mut Vec<Task>, slots: &Scheduled, impossible: bool) {
        let mut slots = slots.borrow().to_vec();
        slots.sort_by(|a, b| a.1.cmp(&b.1));
        slots
            .iter()
            .enumerate()
            .for_each(|(idx, (position, range, _goal))| {
                let start = range.start().naive_date_time();
                let deadline = range.end().naive_date_time();

                // if let Some(f) = self.flexibility_at(*position) {
                //     tasks.push(Task {
                //         taskid: idx,
                //         goalid: f.goal.id(),
                //         title: f.goal.title(),
                //         duration: f.goal.min_span(),
                //         start,
                //         deadline,
                //         tags: vec![],
                //         impossible,
                //     })
                // }
            })
    }
    fn gather_tasks_with_filler(&self, tasks: &mut Vec<Task>, slots: &Scheduled, impossible: bool) {
        let mut current = self.day.start_of_day();
        let mut filler_offset = 0;
        let mut slots = slots.borrow().to_vec();
        slots.sort_by(|a, b| a.1.cmp(&b.1));
        slots
            .iter()
            .enumerate()
            .for_each(|(idx, (position, range, _goal))| {
                if current.lt(range.start()) {
                    let span = current.span_by(range.start());
                    tasks.push(Task {
                        taskid: idx + filler_offset,
                        goalid: "free".to_string(),
                        title: "free".to_string(),
                        duration: span,
                        start: current.naive_date_time(),
                        deadline: current.inc_by(span).naive_date_time(),
                        tags: vec![],
                        impossible: false,
                    });
                    filler_offset += 1;
                }
                current = range.end().clone();

                let start = range.start().naive_date_time();
                let deadline = range.end().naive_date_time();

                // if let Some(f) = self.flexibility_at(*position) {
                //     tasks.push(Task {
                //         taskid: idx + filler_offset,
                //         goalid: f.goal.id(),
                //         title: f.goal.title(),
                //         duration: f.goal.min_span(),
                //         start,
                //         deadline,
                //         tags: vec![],
                //         impossible,
                //     })
                // }
            });
        if current.lt(&self.day.end_of_day()) {
            let span = current.span_by(&self.day.end_of_day());
            tasks.push(Task {
                taskid: slots.len() + filler_offset,
                goalid: "free".to_string(),
                title: "free".to_string(),
                duration: span,
                start: current.naive_date_time(),
                deadline: current.inc_by(span).naive_date_time(),
                tags: vec![],
                impossible: false,
            });
        }
    }
}

impl Debug for Calendar {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Calendar debug output\n")
    }
}
