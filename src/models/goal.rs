use crate::models::calendar::Span;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Goal {
    id: String,
    title: String,

    min_span: Option<usize>,
    // day_filter: Option<DayFilter>,
}
impl Goal {
    #[allow(dead_code)]
    pub(crate) fn new(id: &str, title: &str) -> Self {
        Self {
            id: id.to_string(),
            title: title.to_string(),
            min_span: None,
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
}

impl Hash for Goal {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.title.hash(state);
    }
}
