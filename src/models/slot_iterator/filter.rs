use super::{Slot, TimeSlotsIterator};
use crate::models::{goal::Day, slot::utils::convert_into_1h_slots};
use chrono::{Datelike, Timelike};
use log::info;

impl TimeSlotsIterator {
    pub(crate) fn apply_filters(&mut self) {
        match &self.filters {
            Some(time_filter) => {
                let _timeline_str: String = format!("{:?}", self.timeline);
                let _time_filter_str: String = time_filter.to_string();

                if let Some(slots) = apply_timing_filter(self) {
                    self.timeline = slots
                }
                let _timeline_str: String = format!("{:?}", self.timeline);

                if let Some(slots) = apply_on_days_filter(self) {
                    self.timeline = slots
                }
                let _timeline_str: String = format!("{:?}", self.timeline);

                if let Some(slots) = apply_not_on_filter(self) {
                    self.timeline = slots
                }
                let _timeline_str: String = format!("{:?}", self.timeline);
                info!("{}", _timeline_str);
            }
            None => (),
        }
    }
}

/// Applies postpone feature for the given slot iterator.
fn apply_not_on_filter(slot_iterator: &TimeSlotsIterator) -> Option<Vec<Slot>> {
    if let Some(time_filter) = &slot_iterator.filters {
        let _time_filter_str: String = time_filter.to_string();
        match &time_filter.not_on {
            Some(not_on_slots) => {
                let _not_on_slots_str: String = format!("{:?}", not_on_slots);

                // Filter out time slots that are on the "not_on" list

                let not_on_slots_as_hours: Vec<Slot> = convert_into_1h_slots(not_on_slots.clone());
                let _not_on_slots_as_hours_str: String = format!("{:?}", not_on_slots_as_hours);

                let mut timeline_as_hours: Vec<Slot> =
                    convert_into_1h_slots(slot_iterator.timeline.clone());
                let _timeline_as_hours_str: String = format!("{:?}", timeline_as_hours);

                timeline_as_hours.retain(|slot| !not_on_slots_as_hours.contains(slot));
                let _timeline_as_hours_str: String = format!("{:?}", timeline_as_hours);

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
        let _time_filter_str: String = time_filter.to_string();
        match &time_filter.on_days {
            Some(on_days) => {
                let mut result: Vec<Slot> = vec![];
                for slot in slot_iterator.timeline.iter() {
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
        let _time_filter_str: String = time_filter.to_string();
        if time_filter.after_time.is_some() || time_filter.before_time.is_some() {
            let mut result: Vec<Slot> = vec![];
            for slot in slot_iterator.timeline.iter() {
                let _slot_str = slot.to_string();
                let mut daily_slots = slot.divide_into_days();
                if time_filter.after_time.is_some() && time_filter.before_time.is_some() {
                    //after and before time
                    for daily_slot in daily_slots.iter() {
                        let _daily_slot_str = daily_slot.to_string();

                        let before_datetime = daily_slot
                            .start
                            .with_hour(time_filter.before_time.unwrap() as u32)
                            .unwrap();
                        let after_datetime = daily_slot
                            .start
                            .with_hour(time_filter.after_time.unwrap() as u32)
                            .unwrap();

                        let _after_datetime_str = after_datetime.to_string();
                        let _before_datetime_str = before_datetime.to_string();

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
            return Some(result);
        }
        None
    } else {
        None
    }
}
