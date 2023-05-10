use crate::models::{slot::Slot, timeline::Timeline};
use chrono::Duration;

#[test]
fn test_remove_from() {
    let sample_slot = Slot::mock(Duration::hours(15), 2022, 10, 1, 05, 0);

    if let Some(mut timeline) = Timeline::initialize(sample_slot.start, sample_slot.end) {
        let slot_to_remove = Slot::mock(Duration::hours(5), 2022, 10, 1, 05, 0);
        dbg!(&slot_to_remove);

        timeline.remove_slots(vec![slot_to_remove]);
        let result: Vec<Slot> = timeline.slots.clone().into_iter().collect();
        let expected_result = vec![Slot::mock(Duration::hours(10), 2022, 10, 1, 10, 0)];
        dbg!(&expected_result);
        dbg!(&result);

        assert_eq!(expected_result, result);
    } else {
        assert!(false);
    }
}

#[test]
fn test_remove_halfday_from_fullday() {
    let slot_fullday = Slot::mock(Duration::hours(24), 2022, 10, 1, 00, 0);
    dbg!(&slot_fullday);
    let mut timeline = Timeline {
        slots: vec![slot_fullday].into_iter().collect(),
    };
    dbg!(&timeline);

    let slot_halfday_night = Slot::mock(Duration::hours(12), 2022, 10, 1, 12, 0);
    let slot_halfday_morning = Slot::mock(Duration::hours(12), 2022, 10, 1, 0, 0);
    let expected_result = vec![slot_halfday_morning];

    dbg!(&slot_halfday_night);
    dbg!(&expected_result);

    timeline.remove_slots(vec![slot_halfday_night]);
    let result: Vec<Slot> = timeline.slots.clone().into_iter().collect();
    dbg!(&result);

    assert_eq!(expected_result, result);
}

#[test]
fn test_remove_afternoon_hours_from_fullday() {
    let slot_fullday = Slot::mock(Duration::hours(24), 2022, 10, 1, 00, 0);
    let mut timeline_fullday = Timeline {
        slots: vec![slot_fullday].into_iter().collect(),
    };

    let slot_afternoon = Slot::mock(Duration::hours(3), 2022, 10, 1, 12, 0);

    let expected_result = vec![
        Slot::mock(Duration::hours(12), 2022, 10, 1, 0, 0),
        Slot::mock(Duration::hours(9), 2022, 10, 1, 15, 0),
    ];

    dbg!(&timeline_fullday);
    dbg!(&slot_afternoon);
    dbg!(&expected_result);

    timeline_fullday.remove_slots(vec![slot_afternoon]);
    let result: Vec<Slot> = timeline_fullday.slots.clone().into_iter().collect();
    dbg!(&result);

    assert_eq!(expected_result, result);
}

#[test]
fn test_based_on_i284_7days() {
    // Test based on failed test: issue-284-filter-days-of-week-7days

    let year: i32 = 2023;
    let month: u32 = 3;
    let day: u32 = 9;
    let start_hour = 8;
    let end_hour = 12;
    let duration = Duration::hours(end_hour - start_hour as i64);

    let slots: Vec<Slot> = vec![
        Slot::mock(duration, year, month, day, start_hour, 0),
        Slot::mock(duration, year, month, day + 1, start_hour, 0),
        Slot::mock(duration, year, month, day + 2, start_hour, 0),
        Slot::mock(duration, year, month, day + 3, start_hour, 0),
        Slot::mock(duration, year, month, day + 4, start_hour, 0),
        Slot::mock(duration, year, month, day + 5, start_hour, 0),
        Slot::mock(duration, year, month, day + 6, start_hour, 0),
        Slot::mock(duration, year, month, day + 7, start_hour, 0),
        Slot::mock(duration, year, month, day + 8, start_hour, 0),
        Slot::mock(duration, year, month, day + 9, start_hour, 0),
        Slot::mock(duration, year, month, day + 10, start_hour, 0),
    ];

    let mut timeline = Timeline {
        slots: slots.clone().into_iter().collect(),
    };
    dbg!(&timeline);

    let expected_result: Vec<Slot> = slots;

    timeline.remove_slots(vec![]);
    let result: Vec<Slot> = timeline.slots.clone().into_iter().collect();
    dbg!(&result);

    assert_eq!(expected_result, result);
}
