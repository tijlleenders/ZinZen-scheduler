use crate::{
    models::{goal::TimeFilter, slot::Slot, timeline::Timeline},
    services::filter::apply_filter,
    tests::utils::get_timeline,
};
use chrono::{Duration, NaiveDate};

#[test]
fn test_after_5am() {
    let init_year = 2022;
    let init_month = 1;
    let init_day = 1;
    let init_duration = Duration::days(1);

    // intiate a sample timeline
    let timeline = get_timeline(init_duration, init_year, init_month, init_day);
    dbg!(&timeline);

    let expected_result = Timeline {
        slots: vec![Slot {
            start: NaiveDate::from_ymd_opt(init_year, init_month, init_day)
                .unwrap()
                .and_hms_opt(5, 0, 0)
                .unwrap(),
            end: NaiveDate::from_ymd_opt(init_year, init_month, init_day + 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        }]
        .into_iter()
        .collect(),
    };
    dbg!(&expected_result);

    let filtered_timeline = apply_filter(
        &timeline,
        &(TimeFilter {
            before_time: None,
            after_time: Some(5),
            on_days: None,
            not_on: None,
        }),
    );
    dbg!(&filtered_timeline);

    assert_eq!(filtered_timeline, expected_result);
}
