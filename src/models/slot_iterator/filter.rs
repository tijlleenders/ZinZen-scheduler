use std::ops::Add;

use super::{Slot, TimeSlotsIterator};
use crate::models::{goal::Day, slot::utils::convert_into_1h_slots, timeline::Timeline};
use chrono::{Datelike, Timelike};

impl TimeSlotsIterator {
    pub(crate) fn apply_filters(&mut self) {
        match &self.filters {
            Some(time_filter) => {
                dbg!(&self.timeline);
                dbg!(&time_filter);

                if let Some(slots) = apply_timing_filter(self) {
                    self.timeline = Timeline {
                        slots: slots.into_iter().collect(),
                    }
                }
                dbg!(&self.timeline);

                if let Some(slots) = apply_on_days_filter(self) {
                    self.timeline = Timeline {
                        slots: slots.into_iter().collect(),
                    }
                }
                dbg!(&self.timeline);

                if let Some(slots) = apply_not_on_filter(self) {
                    self.timeline = Timeline {
                        slots: slots.into_iter().collect(),
                    }
                }
                dbg!(&self.timeline);
            }
            None => (),
        }
    }
}

/// Applies postpone feature for the given slot iterator.
fn apply_not_on_filter(slot_iterator: &TimeSlotsIterator) -> Option<Vec<Slot>> {
    if let Some(time_filter) = &slot_iterator.filters {
        dbg!(&time_filter);
        match &time_filter.not_on {
            Some(not_on_slots) => {
                dbg!(&not_on_slots);

                // Filter out time slots that are on the "not_on" list

                let not_on_slots_as_hours: Vec<Slot> = convert_into_1h_slots(not_on_slots.clone());
                dbg!(&not_on_slots_as_hours);

                let mut timeline_as_hours: Vec<Slot> = convert_into_1h_slots(
                    slot_iterator.timeline.slots.clone().into_iter().collect(),
                );
                dbg!(&timeline_as_hours);

                timeline_as_hours.retain(|slot| !not_on_slots_as_hours.contains(slot));
                dbg!(&timeline_as_hours);

                Some(timeline_as_hours)
            }
            None => None,
        }
    } else {
        None
    }
}

fn apply_on_days_filter(slot_iterator: &TimeSlotsIterator) -> Option<Vec<Slot>> {
    if let Some(time_filter) = &slot_iterator.filters {
        dbg!(&time_filter);
        match &time_filter.on_days {
            Some(on_days) => {
                let mut result: Vec<Slot> = vec![];
                for slot in slot_iterator.timeline.slots.iter() {
                    let daily_slots = slot.divide_into_days();

                    for daily_slot in daily_slots.iter() {
                        // Check if the weekday matches with the given on days filter value
                        //  and push it to result vector if true.
                        let start_day: String = slot.start.weekday().to_string();
                        if on_days.contains(&Day::from(start_day)) {
                            result.push(*daily_slot);
                        }
                    }
                }

                Some(result)
            }
            None => None,
        }
    } else {
        None
    }
}

fn apply_timing_filter(slot_iterator: &TimeSlotsIterator) -> Option<Vec<Slot>> {
    if let Some(time_filter) = &slot_iterator.filters {
        dbg!(time_filter);
        if time_filter.after_time.is_some() || time_filter.before_time.is_some() {
            let mut result: Vec<Slot> = vec![];
            for slot in slot_iterator.timeline.slots.clone().into_iter() {
                dbg!(&slot);
                let mut daily_slots = slot.divide_into_days();
                if time_filter.after_time.is_some() && time_filter.before_time.is_some() {
                    //after and before time
                    for daily_slot in daily_slots.iter() {
                        dbg!(&daily_slot);

                        let before_datetime = daily_slot
                            .start
                            .with_hour(time_filter.before_time.unwrap() as u32)
                            .unwrap();
                        let after_datetime = daily_slot
                            .start
                            .with_hour(time_filter.after_time.unwrap() as u32)
                            .unwrap();

                        dbg!(&after_datetime);
                        dbg!(&before_datetime);

                        if after_datetime > before_datetime {
                            if daily_slot.start < before_datetime {
                                result.push({
                                    Slot {
                                        start: daily_slot.start,
                                        end: before_datetime,
                                    }
                                });
                            }
                            if daily_slot.end > after_datetime {
                                result.push({
                                    Slot {
                                        start: after_datetime,
                                        end: daily_slot.end,
                                    }
                                });
                            }
                        } else {
                            result.push({
                                Slot {
                                    start: after_datetime,
                                    end: before_datetime,
                                }
                            })
                        }
                    }
                }
                if time_filter.after_time.is_some() && time_filter.before_time.is_none() {
                    //only after time
                    for daily_slot in daily_slots.iter_mut() {
                        let after_datetime = daily_slot
                            .start
                            .with_hour(time_filter.after_time.unwrap() as u32)
                            .unwrap();
                        if daily_slot.start < after_datetime {
                            daily_slot.start = after_datetime;
                        }
                        result.push(*daily_slot);
                    }
                }
                if time_filter.after_time.is_none() && time_filter.before_time.is_some() {
                    //only before time
                    for daily_slot in daily_slots.iter_mut() {
                        let before_datetime = daily_slot
                            .start
                            .with_hour(time_filter.before_time.unwrap() as u32)
                            .unwrap();
                        if daily_slot.end > before_datetime {
                            daily_slot.end = before_datetime;
                        }
                        result.push(*daily_slot);
                    }
                }
            }

            // TODO 2023-04-21
            //  As agreed in meeting on 2023-04-21 to avoid merge_consequent_slots
            // and implementing new feature Timeline
            // if let Some(merged) = merge_consequent_slots(&result) {
            //     dbg!(&merged);
            //     dbg!(&merged.len());
            //     dbg!(&result.len());
            //     return Some(merged);
            // }

            return Some(result);
        }
        None
    } else {
        None
    }
}

// function merge_consequent_slots
fn _merge_consequent_slots(slots: &Vec<Slot>) -> Option<Vec<Slot>> {
    dbg!(&slots);
    dbg!(&slots.len());

    if slots.is_empty() {
        return None;
    }

    let mut result: Vec<Slot> = vec![];

    for (index, slot) in slots.iter().enumerate() {
        // check if there is next slot?
        if index < slots.len() - 1 {
            let next_slot = slots[index + 1];
            dbg!(&next_slot);

            // add next_slot to slot
            if let Some(merged_slot) = slot.add(next_slot) {
                dbg!(merged_slot);
                result.push(merged_slot);
            }
        }
    }
    if result.is_empty() {
        return None;
    }
    Some(result)
}
