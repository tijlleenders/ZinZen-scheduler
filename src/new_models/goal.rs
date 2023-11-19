use std::hash::{Hash, Hasher};
use std::rc::Rc;
use crate::new_models::calendar::Span;
use crate::new_models::date::{DateTime, DateTimeRange};
use crate::new_models::day_filter::DayFilter;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Goal {
    id: String,
    title: String,

    min_span: Option<usize>,

    day_filter: Option<DayFilter>,
}
impl Goal {
    pub(crate) fn new(id: &str, title: &str) -> Self {
        Self {
            id: id.to_string(),
            title: title.to_string(),
            min_span: None,
            day_filter: None,
        }
    }
    pub fn id(&self) -> String {
        self.id.clone()
    }
    pub fn title(&self) -> String {
        self.title.clone()
    }
    pub fn min_span(&self) -> Span {
        self.min_span.unwrap_or(1)
    }
    pub fn day_filter(&self, date: &DateTime) -> DateTimeRange {
        let out = DateTimeRange::new(
            self.day_filter.as_ref().map(|f| f.after(date)).unwrap_or(date.start_of_day()),
            self.day_filter.as_ref().map(|f| f.before(date)).unwrap_or(date.end_of_day()),
        );
        out
    }
}
impl From<&crate::models::goal::Goal> for Goal {
    fn from(goal: &crate::models::goal::Goal) -> Self {
        #[inline]
        fn to_string(hour: usize) -> String {
            format!("{hour:0>2}:00")
        }
        let filter = goal.filters.clone()
            .map(|f| DayFilter::from_str(
                f.after_time.map(to_string).as_deref(),
                f.before_time.map(to_string).as_deref(),
            ));

        Self {
            id: goal.id.clone(),
            title: goal.title.clone(),

            min_span: goal.min_duration,

            day_filter: filter,
        }
    }
}

impl Hash for Goal {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.title.hash(state);
    }
}
