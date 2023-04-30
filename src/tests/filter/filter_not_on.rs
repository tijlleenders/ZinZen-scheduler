use crate::{
    models::{slot::Slot, timeline::Timeline},
    services::filter::filter_not_on,
    tests::utils::{get_slot, get_timeline_days},
};
use chrono::Duration;

// #[test]
fn test_simple() {
    let init_year = 2022;
    let init_month = 1;
    let init_day = 1;
    let days_count: i64 = 5;
    let duration = Duration::days(1);
    let hour: u32 = 0;
    let minute: u32 = 0;

    let timeline = get_timeline_days(days_count, init_year, init_month, init_day);
    dbg!(&timeline);

    let slots_to_filter: Vec<Slot> =
        vec![get_slot(duration, init_year, init_month, 2, hour, minute)];

    let expected_result = Timeline {
        slots: vec![
            get_slot(duration, init_year, init_month, init_day, hour, minute),
            get_slot(duration, init_year, init_month, 3, hour, minute),
            get_slot(duration, init_year, init_month, 4, hour, minute),
            get_slot(duration, init_year, init_month, 5, hour, minute),
        ]
        .into_iter()
        .collect(),
    };
    dbg!(&expected_result);

    let filtered_timeline = filter_not_on(&timeline, &slots_to_filter);
    dbg!(&filtered_timeline);

    assert_eq!(filtered_timeline, expected_result);
}
