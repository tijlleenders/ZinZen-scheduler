pub mod iterator;
pub mod utils;

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
        dbg!(&result.timeline);
        result.timeline.apply_filter(&result.filters);
        dbg!(&result.timeline);
        result.current_start_position = result.timeline.slots.first().unwrap().start;
        result
    }
}

/// Enum representing Timing Scenario for the provided
/// timing range (after_time and before_time)
#[derive(PartialEq, Debug)]
pub enum TimingScenario {
    /// Unbounded timing scenario where neither `after_time` nor `before_time` is defined
    Unbounded,
    /// Bounded timing scenario where both `after_time` and `before_time` are defined,
    /// and `after_time` is less than or equal to `before_time`
    Bounded,
    /// Timing scenario where only `after_time` is defined and `before_time` is `None`
    AfterOnly,
    /// Timing scenario where only `before_time` is defined and `after_time` is `None`
    BeforeOnly,
    /// Timing scenario where `after_time` is greater than `before_time`, indicating a time range that wraps around midnight
    Overflow,
}
