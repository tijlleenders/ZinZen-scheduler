use super::TimeSlotsIterator;

impl TimeSlotsIterator {
   
    pub(crate) fn apply_filters(&mut self) {
        match &self.filters {
            Some(time_filter) => {
                let time_filter_str: String = time_filter.to_string();

                if time_filter.after_time.is_some() || time_filter.before_time.is_some() {
                    let mut result: Vec<Slot> = vec![];
                    for slot in self.timeline.iter_mut() {
                        let slot_str = slot.to_string();
                        let mut daily_slots = slot.divide_in_days();
                        if time_filter.after_time.is_some() && time_filter.before_time.is_some() {
                            //after and before time
                            for daily_slot in daily_slots.iter_mut() {
                                let daily_slot_str = daily_slot.to_string();

                                let before_datetime = daily_slot
                                    .start
                                    .with_hour(time_filter.before_time.unwrap() as u32)
                                    .unwrap();
                                let after_datetime = daily_slot
                                    .start
                                    .with_hour(time_filter.after_time.unwrap() as u32)
                                    .unwrap();

                                let after_datetime_str = after_datetime.to_string();
                                let before_datetime_str = before_datetime.to_string();

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