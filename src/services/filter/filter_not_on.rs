use crate::models::{slot::Slot, timeline::Timeline};

// TODO 2023-05-15  | Need more test cases to cover all possible unexpected situations
// Examples: partial overlap, no overlap, etc

/// Filtering timeline based on not_on field in TimeFilter
pub(crate) fn filter_not_on(mut timeline: Timeline, slots_to_filter: &[Slot]) -> Timeline {
    if timeline.slots.is_empty() || slots_to_filter.is_empty() {
        return timeline;
    }

    timeline.remove_slots(slots_to_filter.to_vec());

    timeline
}

#[cfg(test)]
mod tests {
    use chrono::Duration;

    use crate::{
        models::{slot::Slot, timeline::Timeline},
        services::filter::filter_not_on::filter_not_on,
    };

    #[test]
    fn test_when_timeline_is_empty() {
        let timeline = Timeline::new();

        let slots_to_filter: Vec<Slot> = vec![Slot::mock(Duration::days(1), 2023, 5, 1, 0, 0)];
        let result = filter_not_on(timeline.clone(), &slots_to_filter);
        assert_eq!(timeline, result);
    }

    #[test]
    fn test_when_empty_days() {
        let timeline = Timeline::mock(Duration::days(5), 2023, 5, 1);

        let slots_to_filter: Vec<Slot> = vec![];
        let result = filter_not_on(timeline.clone(), &slots_to_filter);
        assert_eq!(timeline, result);
    }

    /// Test filter_not_on function when timeline have many slots
    /// - timeline: 5 days (Starting Mon 2023-05-01 to Fri 2023-05-05)
    /// - slots_to_filter: 2023-05-02 00 to 05
    /// - Expected list of all 5 days except first 5 hours of 2023-05-02
    #[test]
    fn test_when_timeline_have_many_slots() {
        let slots_to_filter: Vec<Slot> = vec![Slot::mock(Duration::hours(5), 2023, 5, 2, 0, 0)];

        let timeline = Timeline::mock_as_days(5, 2023, 5, 1);

        let expected_result: Timeline = Timeline {
            slots: vec![
                Slot::mock(Duration::days(1), 2023, 5, 1, 0, 0),
                Slot::mock(Duration::hours(19), 2023, 5, 2, 5, 0),
                Slot::mock(Duration::days(1), 2023, 5, 3, 0, 0),
                Slot::mock(Duration::days(1), 2023, 5, 4, 0, 0),
                Slot::mock(Duration::days(1), 2023, 5, 5, 0, 0),
            ]
            .into_iter()
            .collect(),
        };

        let result = filter_not_on(timeline, &slots_to_filter);

        assert_eq!(expected_result, result);
    }

    /// Test filter_not_on function when timeline have many slots and
    /// many slots to filter
    /// - timeline: 5 days (Starting Mon 2023-05-01 to Fri 2023-05-05)
    /// - slots_to_filter: 2023-05-02 00 to 05 and 2023-05-04 13 to 17
    /// - Expected list of all 5 days except first 5 hours of 2023-05-02 and
    /// except hours from 13 to 17 in 2023-05-04
    #[test]
    fn test_timeline_have_many_slots_many_to_filter() {
        let slots_to_filter: Vec<Slot> = vec![
            Slot::mock(Duration::hours(5), 2023, 5, 2, 0, 0),
            Slot::mock(Duration::hours(4), 2023, 5, 4, 13, 0),
        ];

        let timeline = Timeline::mock_as_days(5, 2023, 5, 1);

        let expected_result: Timeline = Timeline {
            slots: vec![
                Slot::mock(Duration::days(1), 2023, 5, 1, 0, 0),
                Slot::mock(Duration::hours(19), 2023, 5, 2, 5, 0),
                Slot::mock(Duration::days(1), 2023, 5, 3, 0, 0),
                Slot::mock(Duration::hours(13), 2023, 5, 4, 0, 0),
                Slot::mock(Duration::hours(7), 2023, 5, 4, 17, 0),
                Slot::mock(Duration::days(1), 2023, 5, 5, 0, 0),
            ]
            .into_iter()
            .collect(),
        };

        let result = filter_not_on(timeline, &slots_to_filter);

        assert_eq!(expected_result, result);
    }

    /// Test filter_not_on function for failed test case i293_postpone_2
    /// - timeline: 6 days (Starting Sat 2023-04-01 to Fri 2023-04-07)
    /// - slots_to_filter: 2023-04-01 00 to 03 and 05 to 06
    /// - Expected list of all 6 days except:
    ///     - 2023-04-01 from 00 to 03 and from 05 to 06
    #[test]
    fn test_i293_postpone_2() {
        let slots_to_filter: Vec<Slot> = vec![
            Slot::mock(Duration::hours(3), 2023, 4, 1, 0, 0),
            Slot::mock(Duration::hours(1), 2023, 4, 1, 5, 0),
        ];

        let timeline = Timeline::mock_as_days(6, 2023, 4, 1);

        let expected_result: Timeline = Timeline {
            slots: vec![
                Slot::mock(Duration::hours(2), 2023, 4, 1, 3, 0),
                Slot::mock(Duration::hours(18), 2023, 4, 1, 6, 0),
                Slot::mock(Duration::days(1), 2023, 4, 2, 0, 0),
                Slot::mock(Duration::days(1), 2023, 4, 3, 0, 0),
                Slot::mock(Duration::days(1), 2023, 4, 4, 0, 0),
                Slot::mock(Duration::days(1), 2023, 4, 5, 0, 0),
                Slot::mock(Duration::days(1), 2023, 4, 6, 0, 0),
            ]
            .into_iter()
            .collect(),
        };

        let result = filter_not_on(timeline, &slots_to_filter);

        assert_eq!(expected_result, result);
    }
}
