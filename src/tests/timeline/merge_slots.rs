use crate::models::{slot::Slot, timeline::Timeline};
use chrono::Duration;

#[test]
fn test_merge_all_consequent_slots() {
    /*
    - Timeline: 2023-05-01 from 00:00 to 05:00
    - Define timeline splitted into hours
    - confirm that merge will return one slot which merge all slots into
     one slot because they all are consequent
    */
    let year: i32 = 2023;
    let month: u32 = 5;
    let day: u32 = 1;

    let start_hour: u32 = 0;
    let end_hour: u32 = 5;
    let duration = Duration::hours((end_hour - start_hour) as i64);

    let expected_timeline: Timeline = Timeline::mock(duration, year, month, day);

    let mut input_slots: Vec<Slot> = vec![];
    input_slots.append(
        &mut Slot::mock(Duration::hours(1), year, month, day, start_hour, 0).split_into_1h_slots(),
    );
    input_slots.append(
        &mut Slot::mock(Duration::hours(1), year, month, day, start_hour + 1, 0)
            .split_into_1h_slots(),
    );
    input_slots.append(
        &mut Slot::mock(Duration::hours(1), year, month, day, start_hour + 2, 0)
            .split_into_1h_slots(),
    );
    input_slots.append(
        &mut Slot::mock(Duration::hours(1), year, month, day, start_hour + 3, 0)
            .split_into_1h_slots(),
    );
    input_slots.append(
        &mut Slot::mock(Duration::hours(1), year, month, day, start_hour + 4, 0)
            .split_into_1h_slots(),
    );

    let input_timeline: Timeline = Timeline {
        slots: input_slots.into_iter().collect(),
    };

    let result_timeline = input_timeline.get_merged_slots();

    assert_eq!(expected_timeline, result_timeline);
}

#[test]
fn test_merge_some_consequent_slots() {
    /*
    - Timeline: 2023-05-01 from 00:00 to 05:00 and from 09:00 to 10:00
    - Define timeline splitted into hours
    - confirm that merge will return 2 slots one for consequent and
    one for non-consequent
    */
    let year: i32 = 2023;
    let month: u32 = 5;
    let day: u32 = 1;

    let start_hour: u32 = 0;
    let end_hour: u32 = 5;
    let duration = Duration::hours((end_hour - start_hour) as i64);

    let mut expected_timeline: Timeline = Timeline::mock(duration, year, month, day);
    expected_timeline
        .slots
        .insert(Slot::mock(Duration::hours(1), year, month, day, 9, 0));

    let mut input_slots: Vec<Slot> = vec![];
    input_slots.append(
        &mut Slot::mock(Duration::hours(1), year, month, day, start_hour, 0).split_into_1h_slots(),
    );
    input_slots.append(
        &mut Slot::mock(Duration::hours(1), year, month, day, start_hour + 1, 0)
            .split_into_1h_slots(),
    );
    input_slots.append(
        &mut Slot::mock(Duration::hours(1), year, month, day, start_hour + 2, 0)
            .split_into_1h_slots(),
    );
    input_slots.append(
        &mut Slot::mock(Duration::hours(1), year, month, day, start_hour + 3, 0)
            .split_into_1h_slots(),
    );
    input_slots.append(
        &mut Slot::mock(Duration::hours(1), year, month, day, start_hour + 4, 0)
            .split_into_1h_slots(),
    );
    input_slots
        .append(&mut Slot::mock(Duration::hours(1), year, month, day, 9, 0).split_into_1h_slots());

    let input_timeline: Timeline = Timeline {
        slots: input_slots.into_iter().collect(),
    };

    let result_timeline = input_timeline.get_merged_slots();

    assert_eq!(expected_timeline, result_timeline);
}
