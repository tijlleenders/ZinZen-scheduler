use std::collections::BTreeSet;

use chrono::Datelike;

use crate::models::{
    goal::{Day, TimeFilter},
    slot::Slot,
    timeline::{Timeline, TimelineOperations},
};

// TODO 2023-04-25 | Develop Error handling here

/// Applies time filter on the given timeline, then return filtered timeline
pub fn apply_filter(timeline: &Timeline, filter: &TimeFilter) -> Timeline {
    // Algorithm
    // - apply ontime filter (before_time and after_time fields)
    // - apply on_days filter
    // - apply not_on filter

    // - loop over timeline
    // -

    // filter_timing(timeline, before_time, after_time);

    todo!()
}

/// Filtering timeline based on before_time and after_time fields in TimeFilter
pub fn filter_timing(
    timeline: &Timeline,
    before_time: Option<usize>,
    after_time: Option<usize>,
) -> Timeline {
    todo!("filter_timing")
}

/// Filtering timeline based on on_days field in TimeFilter
pub fn filter_on_days(timeline: &Timeline, days_to_filter: &Vec<Day>) -> Timeline {
    // TODO 2023-04-25 | Need heavy test cases to confirm functionality.
    // Examples are scenarios like below:
    // - if timeline is not splitted
    // - if timeline splitted and slots are not complete day (only some hours of the day)

    let days_to_filter: BTreeSet<Day> = days_to_filter.iter().cloned().collect();
    dbg!(&days_to_filter);

    let mut days_to_remove: Vec<Slot> = vec![];
    let mut timeline = timeline.clone();

    for slot in timeline.slots.iter() {
        dbg!(&slot);

        let slot_days = slot.divide_into_days();
        dbg!(&slot_days);

        for day_slot in slot_days.iter() {
            dbg!(&day_slot);

            let day = day_slot.start.weekday().to_string();
            dbg!(&day);

            if !days_to_filter.contains(&Day::from(day)) {
                days_to_remove.push(day_slot.clone());
                dbg!(&days_to_remove);
            }
        }
    }

    dbg!(&days_to_remove);

    if let Some(()) = timeline.remove_slots(days_to_remove) {
        dbg!(&timeline);
        timeline
    } else {
        timeline
    }
}

/// Filtering timeline based on not_on field in TimeFilter
pub fn filter_not_on(timeline: &Timeline, slots_to_filter: &Vec<Slot>) -> Timeline {
    dbg!(&slots_to_filter);
    let mut timeline = timeline.clone();

    if let Some(()) = timeline.remove_slots(slots_to_filter.clone()) {
        dbg!(&timeline);
        timeline
    } else {
        timeline
    }
}
