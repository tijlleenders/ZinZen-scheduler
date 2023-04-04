use crate::goal::Day;
use crate::goal::TimeFilter;
use crate::repetition::Repetition;
use crate::slot::Slot;
use chrono::prelude::*;
use chrono::Days;
use chrono::Duration;
use chrono::NaiveDateTime;

// derive Debug for TimeSlotsIterator
#[derive(Debug)]
/// An iterator that returns slots that conform to a repetition,
/// with optional filters (after/before/Mondays/Weekdays/Weekends/Bank holidays...)
/// e.g. iterate over all MONDAYS between 1st September 2022 and 30th September 2022.
/// e.g. iterate over all DAYS between 1st September 2022 and 30th September 2022.
/// e.g. iterate over all HOURS between 1st September 2022 and 30th September 2022.
/// e.g. iterate over all 'every two hours' slots between times 10-21 for time period X-Y
pub(crate) struct TimeSlotsIterator {
    timeline: Vec<Slot>,
    repetition: Option<Repetition>,
    filters: Option<TimeFilter>,
    current_start_position: NaiveDateTime,
}

impl TimeSlotsIterator {
    pub fn new(
        start: NaiveDateTime,
        end: NaiveDateTime,
        repetition: Option<Repetition>,
        filters: Option<TimeFilter>,
    ) -> TimeSlotsIterator {
        let mut result = TimeSlotsIterator {
            timeline: vec![Slot { start, end }],
            repetition,
            filters,
            current_start_position: start, //override after applying filters
        };
        result.apply_filters();
        result.current_start_position = result.timeline[0].start;
        result
    }

    fn apply_filters(&mut self) {
        match &self.filters {
            Some(time_filter) => {
                if time_filter.after_time.is_some() || time_filter.before_time.is_some() {
                    let mut result: Vec<Slot> = vec![];
                    for slot in self.timeline.iter_mut() {
                        let mut daily_slots = slot.divide_in_days();
                        if time_filter.after_time.is_some() && time_filter.before_time.is_some() {
                            //after and before time
                            for daily_slot in daily_slots.iter_mut() {
                                let before_datetime = daily_slot
                                    .start
                                    .with_hour(time_filter.before_time.unwrap() as u32)
                                    .unwrap();
                                let after_datetime = daily_slot
                                    .start
                                    .with_hour(time_filter.after_time.unwrap() as u32)
                                    .unwrap();

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
                    self.timeline = result;
                }

                match &time_filter.on_days {
                    Some(on_days) => {
                        // Todo 2023-04-04  | make sure that on_days is not duplicated items
                        let mut result: Vec<Slot> = vec![];
                        for slot in self.timeline.iter_mut() {
                            let mut daily_slots = slot.divide_in_days();

                            for daily_slot in daily_slots.iter_mut() {
                                // Check if the weekday matches with the given on days filter value
                                //  and push it to result vector if true.
                                let start_day: String = slot.start.weekday().to_string();
                                if on_days.contains(&Day::from(start_day)) {
                                    result.push(*daily_slot);
                                }
                            }
                        }

                        self.timeline = result;
                    }
                    None => (),
                }
            }
            None => (),
        }
    }
}

impl Iterator for TimeSlotsIterator {
    type Item = Vec<Slot>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.timeline.is_empty() {
            return None;
        }
        match self.repetition {
            Some(repetition) => {
                let mut result = vec![];

                let next_start_position =
                    get_start_of_repeat_step(&self.current_start_position, repetition);
                let mut indexes_to_delete_count: usize = 0;
                for slot in self.timeline.iter_mut() {
                    if next_start_position.le(&slot.end) && next_start_position.gt(&slot.start) {
                        //next_start_position is 'on' the current slot
                        result.push(Slot {
                            start: slot.start,
                            end: next_start_position,
                        });
                        if next_start_position.eq(&slot.end) {
                            indexes_to_delete_count += 1;
                        } else {
                            slot.start = next_start_position;
                        }
                        self.current_start_position = next_start_position;
                        continue;
                    } else if next_start_position.gt(&slot.end) {
                        //next_start_position is 'past' the current slot
                        indexes_to_delete_count += 1;
                        result.push(*slot);
                    } else {
                        //next_start_position is 'before' the current slot
                        self.current_start_position = next_start_position;
                        break;
                    }
                }
                for _i in 1..=indexes_to_delete_count {
                    self.timeline.remove(0);
                }
                Some(result)
            }
            None => {
                let result = self.timeline.clone();
                self.timeline.clear();
                Some(result)
            }
        }
    }
}

pub fn get_start_of_repeat_step(
    current_date_time: &NaiveDateTime,
    repeat: Repetition,
) -> NaiveDateTime {
    let mut result = *current_date_time;
    match repeat {
        Repetition::DAILY(_) => result
            .checked_add_days(Days::new(1))
            .unwrap()
            .with_hour(0)
            .unwrap()
            .with_minute(0)
            .unwrap()
            .with_second(0)
            .unwrap(),
        Repetition::HOURLY => result.checked_add_signed(Duration::hours(1)).unwrap(),
        Repetition::Weekly(_) => {
            if result.weekday() == Weekday::Mon {
                return result
                    .checked_add_days(Days::new(7))
                    .unwrap()
                    .with_hour(0)
                    .unwrap()
                    .with_minute(0)
                    .unwrap()
                    .with_second(0)
                    .unwrap();
            }
            for _days_to_add in 1..=6 {
                result = result.checked_add_days(Days::new(1)).unwrap();
                if result.weekday() == Weekday::Mon {
                    return result
                        .with_hour(0)
                        .unwrap()
                        .with_minute(0)
                        .unwrap()
                        .with_second(0)
                        .unwrap();
                }
            }
            panic!("shouldn't reach")
        }
        Repetition::WEEKDAYS => {
            match result.weekday() {
                Weekday::Sat => result
                    .checked_add_days(Days::new(2))
                    .unwrap()
                    .with_hour(0)
                    .unwrap()
                    .with_minute(0)
                    .unwrap()
                    .with_second(0)
                    .unwrap(),
                Weekday::Sun => result
                    .checked_add_days(Days::new(1))
                    .unwrap()
                    .with_hour(0)
                    .unwrap()
                    .with_minute(0)
                    .unwrap()
                    .with_second(0)
                    .unwrap(),
                _ => result
                    .checked_add_days(Days::new(1))
                    .unwrap()
                    .with_hour(0)
                    .unwrap()
                    .with_minute(0)
                    .unwrap()
                    .with_second(0)
                    .unwrap(),
            }

            // if result.weekday() == Weekday::Sat {
            //     return result
            //         .checked_add_days(Days::new(2))
            //         .unwrap()
            //         .with_hour(0)
            //         .unwrap()
            //         .with_minute(0)
            //         .unwrap()
            //         .with_second(0)
            //         .unwrap();
            // } else if result.weekday() == Weekday::Sun {
            //     return result
            //         .checked_add_days(Days::new(1))
            //         .unwrap()
            //         .with_hour(0)
            //         .unwrap()
            //         .with_minute(0)
            //         .unwrap()
            //         .with_second(0)
            //         .unwrap();
            // } else {
            //     return result
            //         .checked_add_days(Days::new(1))
            //         .unwrap()
            //         .with_hour(0)
            //         .unwrap()
            //         .with_minute(0)
            //         .unwrap()
            //         .with_second(0)
            //         .unwrap();
            // }
        }
        Repetition::WEEKENDS => {
            if result.weekday() == Weekday::Sat {
                return result
                    .checked_add_days(Days::new(7))
                    .unwrap()
                    .with_hour(0)
                    .unwrap()
                    .with_minute(0)
                    .unwrap()
                    .with_second(0)
                    .unwrap();
            }
            if result.weekday() == Weekday::Sun {
                return result
                    .checked_add_days(Days::new(6))
                    .unwrap()
                    .with_hour(0)
                    .unwrap()
                    .with_minute(0)
                    .unwrap()
                    .with_second(0)
                    .unwrap();
            }
            for _days_to_add in 1..=6 {
                result = result.checked_add_days(Days::new(1)).unwrap();
                if result.weekday() == Weekday::Sat {
                    return result
                        .with_hour(0)
                        .unwrap()
                        .with_minute(0)
                        .unwrap()
                        .with_second(0)
                        .unwrap();
                }
            }
            panic!("Shouldn't reach this");
        }
        Repetition::EveryXdays(day_interval) => result
            .checked_add_days(Days::new(day_interval.try_into().unwrap()))
            .unwrap()
            .with_hour(0)
            .unwrap()
            .with_minute(0)
            .unwrap()
            .with_second(0)
            .unwrap(),
        Repetition::EveryXhours(hour_interval) => result
            .checked_add_signed(Duration::hours(hour_interval.try_into().unwrap()))
            .unwrap(),
        Repetition::MONDAYS => {
            if result.weekday() == Weekday::Mon {
                return result
                    .checked_add_days(Days::new(7))
                    .unwrap()
                    .with_hour(0)
                    .unwrap()
                    .with_minute(0)
                    .unwrap()
                    .with_second(0)
                    .unwrap();
            }
            for _days_to_add in 1..=6 {
                result = result.checked_add_days(Days::new(1)).unwrap();
                if result.weekday() == Weekday::Mon {
                    return result
                        .with_hour(0)
                        .unwrap()
                        .with_minute(0)
                        .unwrap()
                        .with_second(0)
                        .unwrap();
                }
            }
            panic!("Shouldn't reach this");
        }
        Repetition::TUESDAYS => {
            if result.weekday() == Weekday::Tue {
                return result
                    .checked_add_days(Days::new(7))
                    .unwrap()
                    .with_hour(0)
                    .unwrap()
                    .with_minute(0)
                    .unwrap()
                    .with_second(0)
                    .unwrap();
            }
            for _days_to_add in 1..=6 {
                result = result.checked_add_days(Days::new(1)).unwrap();
                if result.weekday() == Weekday::Tue {
                    return result
                        .with_hour(0)
                        .unwrap()
                        .with_minute(0)
                        .unwrap()
                        .with_second(0)
                        .unwrap();
                }
            }
            panic!("Shouldn't reach this");
        }
        Repetition::WEDNESDAYS => {
            if result.weekday() == Weekday::Wed {
                return result
                    .checked_add_days(Days::new(7))
                    .unwrap()
                    .with_hour(0)
                    .unwrap()
                    .with_minute(0)
                    .unwrap()
                    .with_second(0)
                    .unwrap();
            }
            for _days_to_add in 1..=6 {
                result = result.checked_add_days(Days::new(1)).unwrap();
                if result.weekday() == Weekday::Wed {
                    return result
                        .with_hour(0)
                        .unwrap()
                        .with_minute(0)
                        .unwrap()
                        .with_second(0)
                        .unwrap();
                }
            }
            panic!("Shouldn't reach this");
        }
        Repetition::THURSDAYS => {
            if result.weekday() == Weekday::Thu {
                return result
                    .checked_add_days(Days::new(7))
                    .unwrap()
                    .with_hour(0)
                    .unwrap()
                    .with_minute(0)
                    .unwrap()
                    .with_second(0)
                    .unwrap();
            }
            for _days_to_add in 1..=6 {
                result = result.checked_add_days(Days::new(1)).unwrap();
                if result.weekday() == Weekday::Thu {
                    return result
                        .with_hour(0)
                        .unwrap()
                        .with_minute(0)
                        .unwrap()
                        .with_second(0)
                        .unwrap();
                }
            }
            panic!("Shouldn't reach this");
        }
        Repetition::FRIDAYS => {
            if result.weekday() == Weekday::Fri {
                return result
                    .checked_add_days(Days::new(7))
                    .unwrap()
                    .with_hour(0)
                    .unwrap()
                    .with_minute(0)
                    .unwrap()
                    .with_second(0)
                    .unwrap();
            }
            for _days_to_add in 1..=6 {
                result = result.checked_add_days(Days::new(1)).unwrap();
                if result.weekday() == Weekday::Fri {
                    return result
                        .with_hour(0)
                        .unwrap()
                        .with_minute(0)
                        .unwrap()
                        .with_second(0)
                        .unwrap();
                }
            }
            panic!("Shouldn't reach this");
        }
        Repetition::SATURDAYS => {
            if result.weekday() == Weekday::Sat {
                return result
                    .checked_add_days(Days::new(7))
                    .unwrap()
                    .with_hour(0)
                    .unwrap()
                    .with_minute(0)
                    .unwrap()
                    .with_second(0)
                    .unwrap();
            }
            for _days_to_add in 1..=6 {
                result = result.checked_add_days(Days::new(1)).unwrap();
                if result.weekday() == Weekday::Sat {
                    return result
                        .with_hour(0)
                        .unwrap()
                        .with_minute(0)
                        .unwrap()
                        .with_second(0)
                        .unwrap();
                }
            }
            panic!("Shouldn't reach this");
        }
        Repetition::SUNDAYS => {
            if result.weekday() == Weekday::Sun {
                return result
                    .checked_add_days(Days::new(7))
                    .unwrap()
                    .with_hour(0)
                    .unwrap()
                    .with_minute(0)
                    .unwrap()
                    .with_second(0)
                    .unwrap();
            }
            for _days_to_add in 1..=6 {
                result = result.checked_add_days(Days::new(1)).unwrap();
                if result.weekday() == Weekday::Sun {
                    return result
                        .with_hour(0)
                        .unwrap()
                        .with_minute(0)
                        .unwrap()
                        .with_second(0)
                        .unwrap();
                }
            }
            panic!("Shouldn't reach this");
        }
        Repetition::FlexDaily(_, _) => todo!(),
        Repetition::FlexWeekly(_, _) => todo!(),
    }
}
