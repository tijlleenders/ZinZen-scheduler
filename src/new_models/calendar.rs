use crate::models::output::{DayTasks, FinalTasks, Task};
use crate::new_models::calendar::CalendarItem::{Expanded, Filler, Occupied};
use crate::new_models::date::{DateTime, DateTimeRange, DateTimeRangeContainerResult};
use crate::new_models::goal::Goal;

#[derive(Clone)]
pub enum CalendarItem<'a> {
    Filler(DateTimeRange),
    Expanded(DateTimeRangeContainerResult, &'a Goal),
    Occupied(DateTimeRange, &'a Goal),
}

pub struct Calendar<'a> {
    calendar: Vec<CalendarItem<'a>>,
    no_fit: Vec<&'a Goal>,
}

impl<'a> Calendar<'a> {
    pub fn new(date_start: &DateTime, date_end: &DateTime) -> Self {
        Self {
            calendar: vec![Filler(DateTimeRange::new(date_start.clone(), date_end.clone()))],
            no_fit: vec![],
        }
    }
    pub fn result(&self) -> FinalTasks {

        let mut scheduled = vec![];
        for (idx, item) in self.calendar.iter().enumerate() {
            match item {
                Filler(ref range) => scheduled.push(create_task_from_filler(idx, range)),
                Occupied(ref range, goal) => scheduled.push(create_task_from_goal(idx, range, goal)),
                Expanded(_, _) => unreachable!(),
            }
        }

        let date = scheduled[0].start;
        let no_range = DateTimeRange::new(DateTime::from_naive_date_time(&date), DateTime::from_naive_date_time(&date));
        let impossible= self.no_fit.iter()
            .enumerate()
            .map(|(idx, &goal)| create_task_from_goal(idx, &no_range, goal))
            .collect::<Vec<_>>()
            ;

        FinalTasks {
            scheduled: vec![DayTasks {
                day: date.date(),
                tasks: scheduled,
            }],
            impossible: vec![DayTasks {
                day: date.date(),
                tasks: impossible,
            }],
        }
    }
    pub fn fit<'b: 'a>(&'a mut self, range: &DateTimeRange, goal: &'b Goal) {
        let result = self.try_to_fit(range, goal);
        if let Some(goal) = result {
            self.no_fit.push(goal);
        }
        else {
            self.optimize();
        }
    }
    fn try_to_fit<'b: 'a>(&'a mut self, range: &DateTimeRange, goal: &'b Goal) -> Option<&'b Goal> {
        for item in &mut self.calendar {
            if let CalendarItem::Filler(item_range) = item {
                match item_range.is_fitting(range) {
                    DateTimeRangeContainerResult::NoFit => {}
                    DateTimeRangeContainerResult::PerfectFit => {
                        *item = Occupied(item_range.clone(), goal);
                        return None;
                    }
                    result => {
                        *item = Expanded(result, goal);
                        return None;
                    }
                }
            }
        }
        Some(goal)
    }
    fn optimize<'b: 'a>(&'a mut self) {
        let mut new_vec = vec![];

        for item in &self.calendar {
            if let Expanded(result, goal) = item {
                match result {
                    DateTimeRangeContainerResult::FitAtStart(start, end) => {
                        new_vec.push(Occupied(start.clone(), goal));
                        new_vec.push(Filler(end.clone()))
                    }
                    DateTimeRangeContainerResult::FitAtEnd(start, end) => {
                        new_vec.push(Filler(start.clone()));
                        new_vec.push(Occupied(end.clone(), goal));
                    }
                    DateTimeRangeContainerResult::FitInBetween(start, mid, end) => {
                        new_vec.push(Filler(start.clone()));
                        new_vec.push(Occupied(mid.clone(), goal));
                        new_vec.push(Filler(end.clone()))
                    }
                    _ => unreachable!(),
                }
            }
            else {
                new_vec.push(item.clone());
            }
        }

        self.calendar = new_vec;
    }
}

fn create_task_from_goal(idx: usize, range: &DateTimeRange, goal: &Goal) -> Task {
    Task {
        taskid: idx,
        goalid: goal.id(),
        title: goal.title(),
        duration: range.span(),
        start: range.start().into(),
        deadline: range.end().into(),
        tags: vec![],
        impossible: range.span() == 0,
    }
}
fn create_task_from_filler(idx: usize, range: &DateTimeRange) -> Task {
    create_task_from_goal(idx, range, &Goal::new("filler", "filler"))
}
