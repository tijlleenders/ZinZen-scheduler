pub struct TimeFilter {
    pub filter_type: FilterType,
    pub after_time: usize,
    pub before_time: usize,
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
    pub(crate) fn new_after(after_time: usize) -> TimeFilter {
        TimeFilter {
            filter_type: FilterType::After,
            after_time,
            before_time: 0,
        }
    }

    pub(crate) fn new_before(before_time: usize) -> TimeFilter {
        TimeFilter {
            filter_type: FilterType::Before,
            after_time: 0,
            before_time,
        }
    }
}
