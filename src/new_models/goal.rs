use crate::new_models::day_filter::DayFilter;

#[derive(Clone)]
pub struct Goal {
    id: String,
    title: String,

    min_span: Option<usize>,

    day_filter: Option<DayFilter>,
}
impl Goal {
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

            day_filter: goal.filters
                .map(|f| DayFilter::from_str(
                    f.after_time.map(|hour| to_string(hour).as_ref()),
                    f.before_time.map(|hour| to_string(hour).as_ref()),
                ))
        }
    }
}
