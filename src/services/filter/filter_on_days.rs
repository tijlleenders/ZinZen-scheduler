use std::collections::BTreeSet;

use chrono::{Datelike, Duration};

use crate::models::{
    goal::Day,
    slot::Slot,
    timeline::{iterator::TimelineIterator, Timeline},
};

/// Filtering timeline based on on_days field in TimeFilter
pub(crate) fn filter_on_days(timeline: Timeline, days_to_filter: &[Day]) -> Timeline {
    // TODO 2023-04-25 | Need heavy test cases to confirm functionality.
    // Examples are scenarios like below:
    // - if timeline is not splitted
    // - if timeline splitted and slots are not complete day (only some hours of the day)

    // return the same timeline if timeline is empty or days_to_filter is empty
    if timeline.slots.is_empty() || days_to_filter.is_empty() {
        return timeline;
    }

    let days_to_filter: BTreeSet<Day> = days_to_filter.iter().cloned().collect();
    let mut days_to_remove: Vec<Slot> = vec![];
    let timeline_iterator = TimelineIterator::new(timeline, Duration::days(1));

    let mut slots_from_iter: Vec<Slot> = vec![];

    for walking_slots in timeline_iterator {
        slots_from_iter.extend(walking_slots);
    }

    let mut expected_timeline = Timeline {
        slots: slots_from_iter.clone().into_iter().collect(),
    };

    for slot in slots_from_iter {
        let day = slot.start.weekday().to_string();

        if !days_to_filter.contains(&Day::from(day)) {
            days_to_remove.push(slot);
        }
    }

    expected_timeline.remove_slots(days_to_remove);

    expected_timeline
}

#[cfg(test)]
mod tests {
    use chrono::Duration;

    use crate::{
        models::{goal::Day, slot::Slot, timeline::Timeline},
        services::filter::filter_on_days::filter_on_days,
    };

    #[test]
    fn test_when_empty_days() {
        let timeline = Timeline::mock(Duration::days(5), 2023, 05, 1);

        let days_to_filter: Vec<Day> = vec![];
        let result = filter_on_days(timeline.clone(), &days_to_filter);
        assert_eq!(timeline, result);
    }

    #[test]
    fn test_when_timeline_is_empty() {
        let timeline = Timeline::new();

        let days_to_filter: Vec<Day> = vec![Day::Mon];
        let result = filter_on_days(timeline.clone(), &days_to_filter);
        assert_eq!(timeline, result);
    }

    /// Test filter_on_days function when timeline have many slots
    /// - timeline: 15 days (Starting Mon 2023-05-01 to Mon 2023-05-15)
    /// - days_to_filter: Mon Fri
    /// - Expected list of 5 days (Mon, Fri) between Mon 2023-05-01 to Mon 2023-05-15
    #[test]
    fn test_when_timeline_have_many_slots() {
        let days_to_filter: Vec<Day> = vec![Day::Mon, Day::Fri];

        let timeline = Timeline::mock_as_days(15, 2023, 05, 1);

        let expected_result: Timeline = Timeline {
            slots: vec![
                Slot::mock(Duration::days(1), 2023, 05, 1, 0, 0),
                Slot::mock(Duration::days(1), 2023, 05, 5, 0, 0),
                Slot::mock(Duration::days(1), 2023, 05, 8, 0, 0),
                Slot::mock(Duration::days(1), 2023, 05, 12, 0, 0),
                Slot::mock(Duration::days(1), 2023, 05, 15, 0, 0),
            ]
            .into_iter()
            .collect(),
        };

        let result = filter_on_days(timeline, &days_to_filter);

        assert_eq!(expected_result, result);
    }

    /// Test filter_on_days function when timeline single slot
    /// - timeline: single slot for 15 days (Starting Mon 2023-05-01 to Mon 2023-05-15)
    /// - days_to_filter: Sun THu
    /// - Expected list of 4 days (Sun, Thu) between Mon 2023-05-01 to Mon 2023-05-15
    #[test]
    fn test_when_timeline_have_single_slot() {
        let days_to_filter: Vec<Day> = vec![Day::Sun, Day::Thu];

        let timeline = Timeline::mock(Duration::days(15), 2023, 05, 1);

        let expected_result: Timeline = Timeline {
            slots: vec![
                Slot::mock(Duration::days(1), 2023, 05, 4, 0, 0),
                Slot::mock(Duration::days(1), 2023, 05, 7, 0, 0),
                Slot::mock(Duration::days(1), 2023, 05, 11, 0, 0),
                Slot::mock(Duration::days(1), 2023, 05, 14, 0, 0),
            ]
            .into_iter()
            .collect(),
        };

        let result = filter_on_days(timeline, &days_to_filter);

        assert_eq!(expected_result, result);
    }

    /// Test filter_on_days function when normal workday which appeared when
    /// testing apply_filter
    /// - timeline: 15 days from 5am to 3pm
    /// - days_to_filter: Sun, Mon, Tue, Wed, Thu
    /// - Expected list of 11 days (Sun, Mon, Tue, Wed, Thu) between
    ///  Mon 2023-05-01 to Mon 2023-05-15
    #[test]
    fn test_normal_workday_issue() {
        let year = 2023;
        let month = 5;

        let start_time: u32 = 5;

        let days_to_filter: Vec<Day> = vec![Day::Sun, Day::Mon, Day::Tue, Day::Wed, Day::Thu];

        let timeline = Timeline {
            slots: vec![
                Slot::mock(Duration::hours(10), year, 05, 1, start_time, 0),
                Slot::mock(Duration::hours(10), year, 05, 2, start_time, 0),
                Slot::mock(Duration::hours(10), year, 05, 3, start_time, 0),
                Slot::mock(Duration::hours(10), year, 05, 4, start_time, 0),
                Slot::mock(Duration::hours(10), year, 05, 5, start_time, 0),
                Slot::mock(Duration::hours(10), year, 05, 6, start_time, 0),
                Slot::mock(Duration::hours(10), year, 05, 7, start_time, 0),
                Slot::mock(Duration::hours(10), year, 05, 8, start_time, 0),
                Slot::mock(Duration::hours(10), year, 05, 9, start_time, 0),
                Slot::mock(Duration::hours(10), year, 05, 10, start_time, 0),
                Slot::mock(Duration::hours(10), year, 05, 11, start_time, 0),
                Slot::mock(Duration::hours(10), year, 05, 12, start_time, 0),
                Slot::mock(Duration::hours(10), year, 05, 13, start_time, 0),
                Slot::mock(Duration::hours(10), year, 05, 14, start_time, 0),
                Slot::mock(Duration::hours(10), year, 05, 15, start_time, 0),
            ]
            .into_iter()
            .collect(),
        };

        let expected_result: Timeline = Timeline {
            slots: vec![
                Slot::mock(Duration::hours(10), year, month, 1, start_time, 0),
                Slot::mock(Duration::hours(10), year, month, 2, start_time, 0),
                Slot::mock(Duration::hours(10), year, month, 3, start_time, 0),
                Slot::mock(Duration::hours(10), year, month, 4, start_time, 0),
                Slot::mock(Duration::hours(10), year, month, 7, start_time, 0),
                Slot::mock(Duration::hours(10), year, month, 8, start_time, 0),
                Slot::mock(Duration::hours(10), year, month, 9, start_time, 0),
                Slot::mock(Duration::hours(10), year, month, 10, start_time, 0),
                Slot::mock(Duration::hours(10), year, month, 11, start_time, 0),
                Slot::mock(Duration::hours(10), year, month, 14, start_time, 0),
                Slot::mock(Duration::hours(10), year, month, 15, start_time, 0),
            ]
            .into_iter()
            .collect(),
        };

        let result = filter_on_days(timeline, &days_to_filter);

        assert_eq!(expected_result, result);
    }
}
