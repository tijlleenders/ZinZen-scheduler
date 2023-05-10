use crate::{
    models::{slot::Slot, timeline::Timeline},
    services::filter::filter_not_on,
};
use chrono::Duration;

#[test]
fn test_simple() {
    // Timeline have 5 days
    // slot to filter out for a single day which is the 2nd
    let init_year = 2022;
    let init_month = 1;
    let init_day = 1;
    let days_count: i64 = 5;
    let duration = Duration::days(1);
    let hour: u32 = 0;
    let minute: u32 = 0;

    let timeline = Timeline::mock_as_days(days_count, init_year, init_month, init_day);
    dbg!(&timeline);

    let slots_to_filter: Vec<Slot> =
        vec![Slot::mock(duration, init_year, init_month, 2, hour, minute)];

    let mut expected_slots: Vec<Slot> = vec![];
    expected_slots.append(
        &mut Slot::mock(duration, init_year, init_month, init_day, hour, minute)
            .divide_into_1h_slots(),
    );
    expected_slots.append(
        &mut Slot::mock(duration, init_year, init_month, 3, hour, minute).divide_into_1h_slots(),
    );
    expected_slots.append(
        &mut Slot::mock(duration, init_year, init_month, 4, hour, minute).divide_into_1h_slots(),
    );
    expected_slots.append(
        &mut Slot::mock(duration, init_year, init_month, 5, hour, minute).divide_into_1h_slots(),
    );

    let expected_result = Timeline {
        slots: expected_slots.into_iter().collect(),
    };

    let filtered_timeline = filter_not_on(&timeline, &slots_to_filter);
    dbg!(&filtered_timeline, &expected_result);

    assert_eq!(filtered_timeline, expected_result);
}