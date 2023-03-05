use self::time_filter::TimeFilter;
use crate::repetition::Repetition;
use crate::slot::Slot;
use chrono::prelude::*;
use chrono::Duration;
use chrono::NaiveDateTime;

pub mod time_filter;

/// An iterator that returns slots that conform to a repetition,
/// with optional filters (after/before/Mondays/Weekdays/Weekends/Bank holidays...)
/// e.g. iterate over all MONDAYS between 1st September 2022 and 30th September 2022.
/// e.g. iterate over all DAYS between 1st September 2022 and 30th September 2022.
/// e.g. iterate over all HOURS between 1st September 2022 and 30th September 2022.
/// e.g. iterate over all 'every two hours' slots between times 10-21 for time period X-Y
pub(crate) struct TimeSlotIterator {
    timeline: Vec<Slot>,
    repetition: Option<Repetition>,
    filters: Vec<TimeFilter>,
}

impl TimeSlotIterator {
    pub fn new(
        start: NaiveDateTime,
        end: NaiveDateTime,
        repetition: Option<Repetition>,
        filters: Vec<TimeFilter>,
    ) -> TimeSlotIterator {
        let mut result = TimeSlotIterator {
            timeline: vec![Slot {
                start: start,
                end: end,
            }],
            repetition,
            filters,
        };
        result.apply_filters();
        result
    }

    fn apply_filters(&mut self) {
        for filter in self.filters.iter() {
            match filter.filter_type {
                time_filter::FilterType::After => {
                    let mut result: Vec<Slot> = vec![];
                    for slot in self.timeline.iter_mut() {
                        let mut daily_slots = slot.divide_in_days();
                        for daily_slot in daily_slots.iter_mut() {
                            let after_datetime = daily_slot
                                .start
                                .with_hour(filter.after_time as u32)
                                .unwrap();
                            if daily_slot.start < after_datetime {
                                daily_slot.start = after_datetime;
                            }
                            result.push(*daily_slot);
                        }
                    }
                    self.timeline = result;
                }
                time_filter::FilterType::Before => todo!(),
                time_filter::FilterType::Weekdays => todo!(),
                time_filter::FilterType::Weekends => todo!(),
                time_filter::FilterType::Mondays => todo!(),
                time_filter::FilterType::Tuesdays => todo!(),
                time_filter::FilterType::Wednesdays => todo!(),
                time_filter::FilterType::Thursdays => todo!(),
                time_filter::FilterType::Fridays => todo!(),
                time_filter::FilterType::Saturdays => todo!(),
                time_filter::FilterType::Sundays => todo!(),
            }
        }
    }
}

impl Iterator for TimeSlotIterator {
    type Item = Slot;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

fn hour_is_within_bounds(after_time: usize, before_time: usize, hour: usize) -> bool {
    if before_time < after_time {
        hour < before_time || hour >= after_time
    } else {
        hour >= after_time && hour < before_time
    }
}

fn skip_to_after_time(mut original_time: NaiveDateTime, after_time: usize) -> NaiveDateTime {
    while original_time.hour() < after_time as u32 {
        original_time += Duration::hours(1);
    }
    original_time
}
