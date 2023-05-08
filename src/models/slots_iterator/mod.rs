pub mod filter;
pub mod iterator;
pub mod utils;

use crate::services;

use super::{goal::TimeFilter, repetition::Repetition, slot::Slot, timeline::Timeline};
use chrono::NaiveDateTime;

// derive Debug for TimeSlotsIterator
#[derive(Debug)]
/// An iterator that returns slots that conform to a repetition,
/// with optional filters (after/before/Mondays/Weekdays/Weekends/Bank holidays...)
/// e.g. iterate over all MONDAYS between 1st September 2022 and 30th September 2022.
/// e.g. iterate over all DAYS between 1st September 2022 and 30th September 2022.
/// e.g. iterate over all HOURS between 1st September 2022 and 30th September 2022.
/// e.g. iterate over all 'every two hours' slots between times 10-21 for time period X-Y
/// e.g. iterate in 24h steps over a time period that has certain filters applied, starting at after filter time value on day of the first available slot.
pub(crate) struct TimeSlotsIterator {
    pub timeline: Timeline,
    pub repetition: Option<Repetition>,
    pub filters: Option<TimeFilter>,
    pub current_start_position: NaiveDateTime,
}

impl TimeSlotsIterator {
    pub fn new(
        start: NaiveDateTime,
        end: NaiveDateTime,
        repetition: Option<Repetition>,
        filters: Option<TimeFilter>,
    ) -> TimeSlotsIterator {
        let mut result = TimeSlotsIterator {
            timeline: Timeline::initialize(start, end).unwrap(),
            repetition,
            filters,
            current_start_position: start, //override after applying filters
        };
        result.timeline = services::filter::apply_filter(&result.timeline, &result.filters);
        result.current_start_position = result.timeline.slots.first().unwrap().start;
        result
    }
}
