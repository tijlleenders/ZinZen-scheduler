use super::super::slot::{iterator::SlotIterator, Slot};
use super::Timeline;
use chrono::{Duration, NaiveDateTime, Timelike};

// TODO 2023-05-20  | create edge cases to test behavior when first slot start time in the timeline is not 00 (midnight)
// - Test idea to froce make the start of the timeline from 00 (midnight) of the first whatever even if it is other time in the same day

/*
TimelineIterator goals:
- ability to cros over timeline slots like SlotIterator through interval duration
    - cross over timeline by 1 day duration
- Get count of days, hours, minutes, etc for a timeline

Ideas:
- Ability to move to a specific time in the timeline.
- Ability to perform some action in a timeline without
forcing to split slots into hours, or similar.
-
*/

/// Iterator for a `Timeline` and provide functionalities to walk through
/// slots in a `Timeline` based on custom interval duration
#[derive(Debug, Clone)]
pub struct TimelineIterator {
    timeline: Timeline,
    /// Duration interval for pointer to corss over timeline timelines
    interval: Duration,
}

impl TimelineIterator {
    /// Initialize new TimelineIterator with default interval duration to 1 day
    #[allow(dead_code)]
    pub fn initialize(timeline: Timeline) -> TimelineIterator {
        // if let Some(_) = timeline.slots.first() {
        if timeline.slots.first().is_some() {
            TimelineIterator {
                timeline,
                interval: Duration::days(1),
            }
        } else {
            panic!("Timeline slots are empty")
        }
    }

    /// Create new TimelineIterator with custom interval duration
    #[allow(dead_code)]
    pub fn new(timeline: Timeline, interval_duration: Duration) -> TimelineIterator {
        if timeline.slots.first().is_some() {
            TimelineIterator {
                timeline,
                interval: interval_duration,
            }
        } else {
            panic!("Timeline slots are empty")
        }
    }

    /// Create new TimelineIterator which iterate for a daily calendar
    /// day regardless time of slots in the timeline
    #[allow(dead_code)]
    pub fn new_calendar_day(timeline: Timeline) -> TimelineIterator {
        // TODO 2023-07-11: based on debugging in https://github.com/tijlleenders/ZinZen-scheduler/pull/363
        // for case bug_215, agreed to create a custom TimelineIterator to iterate on daily basis from
        // midnight to midnight.
        if let Some(first_slot) = timeline.slots.first() {
            let start_date = first_slot
                .start
                .with_hour(0)
                .unwrap()
                .with_minute(0)
                .unwrap()
                .with_second(0)
                .unwrap();
            let end_date: NaiveDateTime;
            if timeline.slots.len() == 1 {
                end_date = first_slot.end;
            } else if let Some(last_slot) = timeline.slots.last() {
                end_date = last_slot.end;
            } else {
                panic!("Can't get last timeline slot")
            }

            let custom_timeline = Timeline::initialize(start_date, end_date).unwrap();
            TimelineIterator::initialize(custom_timeline)
        } else {
            panic!("Timeline slots are empty")
        }
    }
}

/// Walk through list of slots in timeline based on custom interval duration
impl Iterator for TimelineIterator {
    type Item = Vec<Slot>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.timeline.slots.is_empty() {
            return None;
        }

        if let Some(first_slot) = self.timeline.slots.first() {
            match self.timeline.slots.take(&first_slot.clone()) {
                Some(slot) => {
                    let slot_duration = slot.end.signed_duration_since(slot.start);

                    // A condition to avoid iteration over slots when inerval > slot duration
                    let slot_iterator: SlotIterator = if self.interval > slot_duration {
                        SlotIterator::new(slot, slot_duration)
                    } else {
                        SlotIterator::new(slot, self.interval)
                    };

                    let mut walking_slots: Vec<Slot> = vec![];
                    for slot in slot_iterator {
                        walking_slots.push(slot);
                    }

                    Some(walking_slots)
                }
                None => None,
            }
        } else {
            None
        }
    }
}
