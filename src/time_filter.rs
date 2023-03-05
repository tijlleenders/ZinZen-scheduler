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

impl TimeFilter {}
