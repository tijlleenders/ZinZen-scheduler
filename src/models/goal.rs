use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Goal {
    id: String,
    deadline: String,
    filters: Filters,
    min_duration: usize,
    title: String,
}

#[derive(Deserialize, Debug)]
struct Filters {
    after_time: usize,
    before_time: usize,
}
