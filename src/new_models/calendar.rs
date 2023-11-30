use std::cell::RefCell;
use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;
use crate::models::input::Input;
use crate::models::output::{DayTasks, FinalTasks, Task};
use crate::new_models::date::{DateTime, DateTimeRange};
use crate::new_models::flexibility::Flexibility;
use crate::new_models::goal::Goal;
use crate::new_models::day::Day;
use crate::schedule;

pub type Goals = Vec<Rc<Goal>>;
pub type Span = usize;
pub type Position = usize;
pub type Flex = usize;
pub type Unprocessed = RefCell<Vec<Position>>;
pub type Scheduled = RefCell<Vec<(Position, DateTimeRange, Rc<Goal>)>>;

pub type Data = RefCell<Vec<Flexibility>>;

pub struct Calendar {
    day: DateTime,
    span_of_day: Span,

    flexibilities: Data,

    unprocessed: Unprocessed,

    scheduled: Scheduled,
    impossible: Scheduled,
}

impl Calendar {
    pub fn new(input: &Input, goals: &Goals) -> Self {

        let date_start = DateTime::from_naive_date_time(&input.calendar_start);
        let date_end = DateTime::from_naive_date_time(&input.calendar_end);
        let day = date_start.start_of_day();
        let span_of_day = day.span_of_day();

        let mut flexibilities = goals.into_iter()
            .map(|goal| get_flexibilities(goal.clone(), &date_start, &date_end))
            .collect::<Vec<_>>()
            ;
        flexibilities.sort_by(|a, b| a.goal.id().cmp(&b.goal.id()));
        let flexibilities = RefCell::new(flexibilities);

        let unprocessed: Unprocessed = RefCell::new((0..flexibilities.borrow().len()).collect());

        let scheduled = RefCell::new(vec![]);
        let impossible = RefCell::new(vec![]);

        Self {
            day,
            span_of_day,

            flexibilities,

            unprocessed,

            scheduled,
            impossible,
        }
    }

    pub fn has_finished_scheduling(&self) -> bool {
        !self.unprocessed.borrow().is_empty()
    }

    pub fn flexibility_at(&self, pos: Position) -> Option<Flexibility> {
        self.flexibilities.borrow().get(pos).cloned()
    }
    pub fn flexibility(&self, pos: Position) -> Option<(Position, Flex, Flexibility)> {
        self.flexibility_at(pos)
            .map(|f| (pos, f.day.flexibility(f.goal.min_span()), f))
    }
    pub fn unprocessed(&self) -> Vec<Position> {
        self.unprocessed.borrow().clone()
    }
    pub fn push_impossible(&self, position: Position, range: DateTimeRange) {
        self.flexibility_at(position).unwrap().day.occupy(&range);
        self.occupy_unprocessed(&range);
        self.impossible.borrow_mut().push((position, range, self.flexibility_at(position).unwrap().goal.clone()));
    }
    pub fn push_scheduled(&self, position: Position, range: DateTimeRange) {
        self.flexibility_at(position).unwrap().day.occupy(&range);
        self.occupy_unprocessed(&range);
        self.scheduled.borrow_mut().push((position, range, self.flexibility_at(position).unwrap().goal.clone()));
    }
    pub fn occupy_unprocessed(&self, range: &DateTimeRange) {
        self.unprocessed.borrow().iter().for_each(|pos|
            self.flexibility_at(*pos).unwrap().day.occupy(range)
        );
    }
    pub fn take(&self, position: Position) -> Option<(Flexibility, Vec<Position>)> {
        let (head, tail): (Vec<_>, Vec<_>) = self.unprocessed.borrow().iter()
            .partition(|&p| *p == position);
        *self.unprocessed.borrow_mut() = tail;
        head.first()
            .map(|position| self.flexibility_at(*position))
            .unwrap_or(None)
            .map(|f| (f, self.unprocessed()))
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
        slots.iter().enumerate().for_each(|(idx, (position, range, goal))| {

            let start = range.start().naive_date_time();
            let deadline = range.end().naive_date_time();

            if let Some(f) = self.flexibility_at(*position) {
                tasks.push(Task {
                    taskid: idx,
                    goalid: f.goal.id(),
                    title: f.goal.title(),
                    duration: f.goal.min_span(),
                    start,
                    deadline,
                    tags: vec![],
                    impossible,
                })
            }
        })
    }
    fn gather_tasks_with_filler(&self, tasks: &mut Vec<Task>, slots: &Scheduled, impossible: bool) {
        let mut current = self.day.start_of_day();
        let mut filler_offset = 0;
        let mut slots = slots.borrow().to_vec();
        slots.sort_by(|a, b| a.1.cmp(&b.1));
        slots.iter().enumerate().for_each(|(idx, (position, range, goal))| {

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

            if let Some(f) = self.flexibility_at(*position) {
                tasks.push(Task {
                    taskid: idx + filler_offset,
                    goalid: f.goal.id(),
                    title: f.goal.title(),
                    duration: f.goal.min_span(),
                    start,
                    deadline,
                    tags: vec![],
                    impossible,
                })
            }
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

fn get_flexibilities(goal: Rc<Goal>, start: &DateTime, end: &DateTime) -> Flexibility {
    let goals = vec![goal];
    goals.into_iter()
        .map(|g| (g.clone(), Flexibility {
            goal: g,
            day: Rc::new(Day::new(start.clone())),
        }))
        .map(|(g, f)| {
            let day = f.day;
            day.occupy_inverse_range(&DateTimeRange::new(start.clone(), end.clone()));
            (g, Flexibility {
                goal: f.goal,
                day,
            })
        })
        .map(|(g, f)| {
            let day = f.day;
            day.occupy_inverse_range(&g.day_filter(start));
            Flexibility {
                goal: f.goal,
                day,
            }
        })
        .collect::<Vec<_>>()
        .pop()
        .unwrap()
}

impl Debug for Calendar {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self.flexibilities.borrow().iter()
                .map(|f| f.day.to_string())
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}
