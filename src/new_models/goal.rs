use crate::new_models::day_filter::DayFilter;

#[derive(Debug, Clone)]
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
    pub fn min_span(&self) -> usize {
        self.min_span.unwrap_or(1)
    }
}
impl From<&crate::models::goal::Goal> for Goal {
    fn from(goal: &crate::models::goal::Goal) -> Self {
        #[inline]
        fn to_string(hour: usize) -> String {
            format!("{hour:0>2}")
        }
        Self {
            id: goal.id.clone(),
            title: goal.title.clone(),

            min_span: goal.min_duration.clone(),

            day_filter: goal.filters.clone()
                .map(|f| DayFilter::from_str(
                    f.after_time.map(|hour| to_string(hour)).as_deref(),
                    f.before_time.map(|hour| to_string(hour)).as_deref(),
                ))
        }
    }
}
