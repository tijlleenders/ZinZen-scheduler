use std::collections::BTreeSet;

use chrono::{Datelike, Duration};

use crate::models::{
    goal::Day,
    slot::Slot,
    timeline::{iterator::TimelineIterator, Timeline},
};

/// Filtering timeline based on on_days field in TimeFilter
fn filter_on_days(timeline: Timeline, days_to_filter: &[Day]) -> Timeline {
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
    let timeline_iterator = TimelineIterator::new(timeline.clone(), Duration::days(1));
    dbg!(&timeline, &timeline_iterator, &days_to_filter);
    let mut slots_from_iter: Vec<Slot> = vec![];

    for walking_slots in timeline_iterator {
        dbg!(&walking_slots);
        slots_from_iter.extend(walking_slots);
    }
    dbg!(&slots_from_iter);

    let mut expected_timeline = Timeline {
        slots: slots_from_iter.clone().into_iter().collect(),
    };

    for slot in slots_from_iter {
        let day = slot.start.weekday().to_string();
        dbg!(&day);

        if days_to_filter.contains(&Day::from(day)) {
            days_to_remove.push(slot);
            dbg!(&days_to_remove);
        }
    }
    dbg!(&days_to_remove);

    expected_timeline.remove_slots(days_to_remove);
    dbg!(&expected_timeline);
    expected_timeline
}

#[cfg(test)]
mod tests {
    use chrono::Duration;

    use crate::{
        models::{goal::Day, slot::Slot, timeline::Timeline},
        services::new_filter::filter_on_days::filter_on_days,
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
    /// - Expected list of 10 days
    #[test]
    fn test_when_timeline_have_many_slots() {
        let days_to_filter: Vec<Day> = vec![Day::Mon, Day::Fri];

        let timeline = Timeline::mock_as_days(15, 2023, 05, 1);
        dbg!(&timeline);

        let expected_result: Timeline = Timeline {
            slots: vec![
                Slot::mock(Duration::days(1), 2023, 05, 2, 0, 0),
                Slot::mock(Duration::days(1), 2023, 05, 3, 0, 0),
                Slot::mock(Duration::days(1), 2023, 05, 4, 0, 0),
                Slot::mock(Duration::days(1), 2023, 05, 6, 0, 0),
                Slot::mock(Duration::days(1), 2023, 05, 7, 0, 0),
                Slot::mock(Duration::days(1), 2023, 05, 9, 0, 0),
                Slot::mock(Duration::days(1), 2023, 05, 10, 0, 0),
                Slot::mock(Duration::days(1), 2023, 05, 11, 0, 0),
                Slot::mock(Duration::days(1), 2023, 05, 13, 0, 0),
                Slot::mock(Duration::days(1), 2023, 05, 14, 0, 0),
            ]
            .into_iter()
            .collect(),
        };
        dbg!(&expected_result);

        let result = filter_on_days(timeline, &days_to_filter);

        assert_eq!(expected_result, result);
    }

    /// Test filter_on_days function when timeline single slot
    /// - timeline: single slot for 15 days (Starting Mon 2023-05-01 to Mon 2023-05-15)
    /// - days_to_filter: Mon Fri
    /// - Expected list of 10 days
    #[test]
    fn test_unsplitted_timeline() {
        let days_to_filter: Vec<Day> = vec![Day::Mon, Day::Fri];

        let timeline = Timeline::mock(Duration::days(15), 2023, 05, 1);
        dbg!(&timeline);

        let expected_result: Timeline = Timeline {
            slots: vec![
                Slot::mock(Duration::days(1), 2023, 05, 2, 0, 0),
                Slot::mock(Duration::days(1), 2023, 05, 3, 0, 0),
                Slot::mock(Duration::days(1), 2023, 05, 4, 0, 0),
                Slot::mock(Duration::days(1), 2023, 05, 6, 0, 0),
                Slot::mock(Duration::days(1), 2023, 05, 7, 0, 0),
                Slot::mock(Duration::days(1), 2023, 05, 9, 0, 0),
                Slot::mock(Duration::days(1), 2023, 05, 10, 0, 0),
                Slot::mock(Duration::days(1), 2023, 05, 11, 0, 0),
                Slot::mock(Duration::days(1), 2023, 05, 13, 0, 0),
                Slot::mock(Duration::days(1), 2023, 05, 14, 0, 0),
            ]
            .into_iter()
            .collect(),
        };
        dbg!(&expected_result);

        let result = filter_on_days(timeline, &days_to_filter);

        assert_eq!(expected_result, result);
    }
}