pub struct TimeFilter {
    filter_type: FilterType,
}

pub enum FilterType {
    After,
    Before,
    Weekdays,
    Weekends,
    Mondays,
    Tuesdays,
    Wednesdays,
    Thursdays,
    Fridays,
    Saturdays,
    Sundays,
}

impl TimeFilter {
    pub(crate) fn new_after(_clone: usize) -> TimeFilter {
        TimeFilter {
            filter_type: FilterType::After,
        }
    }
}
