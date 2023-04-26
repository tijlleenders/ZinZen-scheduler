use std::collections::BTreeSet;

use chrono::{Datelike, Timelike};

use crate::models::{
    goal::{Day, TimeFilter},
    slot::Slot,
    timeline::Timeline,
};

// TODO 2023-04-25 | Develop Error handling here

/// Applies time filter on the given timeline, then return filtered timeline
pub fn apply_filter(timeline: &Timeline, filter: &TimeFilter) -> Timeline {
    dbg!(&timeline);
    dbg!(&filter);

    /*
    Algorithm
    - if not None TimeFilter.after_time OR TimeFilter.after_time
        - apply ontime filter
    - if not None TimeFilter.on_days
        - apply on_days filter
    - if not None TimeFilter.not_on
        - apply not_on filter
    */
    let mut filtered_timeline = timeline.clone();
    dbg!(&filtered_timeline);

    if filter.after_time.is_some() || filter.before_time.is_some() {
        filtered_timeline =
            filter_timing(&filtered_timeline, filter.before_time, filter.after_time);
    }
    dbg!(&filtered_timeline);

    if let Some(days) = &filter.on_days {
        filtered_timeline = filter_on_days(&filtered_timeline, days);
    }
    dbg!(&filtered_timeline);

    if let Some(not_on) = &filter.not_on {
        dbg!(&not_on);
        filtered_timeline = filter_not_on(&filtered_timeline, not_on);
    }
    dbg!(&filtered_timeline);

    filtered_timeline
}

/// Filtering timeline based on before_time and after_time fields in TimeFilter
pub fn filter_timing(
    timeline: &Timeline,
    before_time: Option<usize>,
    after_time: Option<usize>,
) -> Timeline {
    dbg!(&timeline);
    dbg!(before_time, after_time);

    // TODO 2023-04-25 | Remove panic and use error handling for below validations
    // TODO 2023-04-26 | Create test scenarios: when slot is not a complete day

    // Some validations
    validate_time(before_time, "before_time");
    validate_time(after_time, "after_time");

    let daily_timeline = timeline.split_into_days();
    dbg!(&daily_timeline);

    let mut filtered_slots: Vec<Slot> = vec![];

    for daily_slot in daily_timeline.slots.into_iter() {
        dbg!(&daily_slot);

        let mut slot_filtered = Slot {
            start: daily_slot.start,
            end: daily_slot.end,
        };
        dbg!(&slot_filtered);

        if let Some(after_time) = after_time {
            let after_datetime = daily_slot.start.with_hour(after_time as u32).unwrap();
            dbg!(&after_datetime);
            if daily_slot.start < after_datetime {
                slot_filtered.start = after_datetime;
                dbg!(&slot_filtered);
            }
        }
        if let Some(before_time) = before_time {
            let before_datetime = daily_slot.start.with_hour(before_time as u32).unwrap();
            dbg!(&before_datetime);

            if daily_slot.end > before_datetime {
                // if before_time before after_time
                if after_time.is_some() && before_time < after_time.unwrap() {
                    let new_slot = Slot {
                        start: daily_slot.start,
                        end: before_datetime,
                    };
                    dbg!(&new_slot);
                    filtered_slots.push(new_slot);
                    dbg!(&filtered_slots);
                } else {
                    slot_filtered.end = before_datetime;
                    dbg!(&slot_filtered);
                }
            }
        }

        dbg!(&slot_filtered);

        filtered_slots.push(slot_filtered);
        dbg!(&filtered_slots);
    }

    dbg!(&filtered_slots);

    Timeline {
        slots: filtered_slots.into_iter().collect(),
    }
}

/// Filtering timeline based on on_days field in TimeFilter
pub fn filter_on_days(timeline: &Timeline, days_to_filter: &[Day]) -> Timeline {
    // TODO 2023-04-25 | Need heavy test cases to confirm functionality.
    // Examples are scenarios like below:
    // - if timeline is not splitted
    // - if timeline splitted and slots are not complete day (only some hours of the day)

    let days_to_filter: BTreeSet<Day> = days_to_filter.iter().cloned().collect();
    dbg!(&days_to_filter);

    let mut days_to_remove: Vec<Slot> = vec![];

    dbg!(&timeline);
    // Get new timeline based ont splitting into days
    let mut timeline = timeline.split_into_days();
    dbg!(&timeline);

    for slot in timeline.slots.iter() {
        dbg!(&slot);

        let day = slot.start.weekday().to_string();
        dbg!(&day);

        if !days_to_filter.contains(&Day::from(day)) {
            days_to_remove.push(*slot);
            dbg!(&days_to_remove);
        }
    }
    dbg!(&days_to_remove);

    timeline.remove_slots(days_to_remove);
    dbg!(&timeline);
    timeline
}

/// Filtering timeline based on not_on field in TimeFilter
pub fn filter_not_on(timeline: &Timeline, slots_to_filter: &Vec<Slot>) -> Timeline {
    dbg!(&slots_to_filter);
    let mut timeline = timeline.clone();

    timeline.remove_slots(slots_to_filter.clone());
    dbg!(&timeline);
    timeline
}

fn validate_time(time: Option<usize>, time_name: &str) {
    if let Some(time) = time {
        if time > 24 {
            panic!("{} must be between 0 and 24", time_name);
        }
    }
}
