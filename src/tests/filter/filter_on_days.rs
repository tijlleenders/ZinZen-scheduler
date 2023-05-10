use crate::{
    models::{goal::Day, slot::Slot, timeline::Timeline},
    services::filter::filter_on_days,
};
use chrono::Duration;

#[test]
fn test_splitted_timeline() {
    // 2022-01-01 is a Saturday

    let init_year = 2022;
    let init_month = 1;
    let init_day = 1;
    let days_count: i64 = 15;
    let duration = Duration::days(1);
    let hour: u32 = 0;
    let minute: u32 = 0;

    let timeline = Timeline::mock_as_days(days_count, init_year, init_month, init_day);
    dbg!(&timeline);

    let days_to_filter: Vec<Day> = vec![Day::Sun, Day::Tue, Day::Fri];

    let expected_result = Timeline {
        slots: vec![
            Slot::mock(duration, init_year, init_month, 2, hour, minute),
            Slot::mock(duration, init_year, init_month, 4, hour, minute),
            Slot::mock(duration, init_year, init_month, 7, hour, minute),
            Slot::mock(duration, init_year, init_month, 9, hour, minute),
            Slot::mock(duration, init_year, init_month, 11, hour, minute),
            Slot::mock(duration, init_year, init_month, 14, hour, minute),
        ]
        .into_iter()
        .collect(),
    };

    let filtered_timeline = filter_on_days(&timeline, &days_to_filter);
    dbg!(&expected_result);
    dbg!(&filtered_timeline);

    assert_eq!(filtered_timeline, expected_result);
}

#[test]
fn test_unsplitted_timeline() {
    // 2022-01-01 is a Saturday
    // TODO this test is not working
    let init_year = 2022;
    let init_month = 1;
    let init_day = 1;
    let days_count: i64 = 15;
    let duration = Duration::days(days_count);
    let hour: u32 = 0;
    let minute: u32 = 0;

    let timeline = Timeline::mock(duration, init_year, init_month, init_day);
    dbg!(&timeline);

    let days_to_filter: Vec<Day> = vec![Day::Sun, Day::Tue, Day::Fri];

    let expected_result = Timeline {
        slots: vec![
            Slot::mock(Duration::days(1), init_year, init_month, 2, hour, minute),
            Slot::mock(Duration::days(1), init_year, init_month, 4, hour, minute),
            Slot::mock(Duration::days(1), init_year, init_month, 7, hour, minute),
            Slot::mock(Duration::days(1), init_year, init_month, 9, hour, minute),
            Slot::mock(Duration::days(1), init_year, init_month, 11, hour, minute),
            Slot::mock(Duration::days(1), init_year, init_month, 14, hour, minute),
        ]
        .into_iter()
        .collect(),
    };

    let filtered_timeline = filter_on_days(&timeline, &days_to_filter);
    dbg!(&expected_result);
    dbg!(&filtered_timeline);

    assert_eq!(filtered_timeline, expected_result);
}