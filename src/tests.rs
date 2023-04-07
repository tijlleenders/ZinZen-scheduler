// use crate::{
//     goal::*, input::*, repetition::Repetition, slot::*, task::TaskStatus::*, task::*,
//     task_generator::*, task_placer::*,
// };
use crate::{
    models::repetition::Repetition,
    models::{goal::Day, slot::*},
};
use chrono::*;

#[cfg(test)]
#[test]
fn get_next_monday() {
    use crate::models::slot_iterator::get_start_of_repeat_step;

    let monday = NaiveDate::from_ymd_opt(2022, 09, 26)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    let saturday = NaiveDate::from_ymd_opt(2022, 10, 1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    let monday_with_time = NaiveDate::from_ymd_opt(2022, 09, 26)
        .unwrap()
        .and_hms_opt(1, 33, 7)
        .unwrap();
    let saturday_with_time = NaiveDate::from_ymd_opt(2022, 10, 1)
        .unwrap()
        .and_hms_opt(1, 33, 7)
        .unwrap();
    let next_monday = NaiveDate::from_ymd_opt(2022, 10, 3)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    let next_monday_from_monday = get_start_of_repeat_step(&monday, Repetition::Weekly(1));
    let next_monday_from_saturday = get_start_of_repeat_step(&saturday, Repetition::Weekly(1));
    let next_monday_from_monday_with_time =
        get_start_of_repeat_step(&monday_with_time, Repetition::Weekly(1));
    let next_monday_from_saturday_with_time =
        get_start_of_repeat_step(&saturday_with_time, Repetition::Weekly(1));
    assert_eq!(next_monday_from_monday, next_monday);
    assert_eq!(next_monday_from_saturday, next_monday);
    assert_eq!(next_monday_from_monday_with_time, next_monday);
    assert_eq!(next_monday_from_saturday_with_time, next_monday);
}

#[test]
fn get_next_weekend() {
    use crate::models::slot_iterator::get_start_of_repeat_step;

    let monday = NaiveDate::from_ymd_opt(2022, 09, 26)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    let saturday = NaiveDate::from_ymd_opt(2022, 10, 1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    let monday_with_time = NaiveDate::from_ymd_opt(2022, 09, 26)
        .unwrap()
        .and_hms_opt(1, 33, 7)
        .unwrap();
    let saturday_with_time = NaiveDate::from_ymd_opt(2022, 10, 1)
        .unwrap()
        .and_hms_opt(1, 33, 7)
        .unwrap();
    let _next_weekend_from_monday = NaiveDate::from_ymd_opt(2022, 10, 8)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    let _next_weekend_from_weekend = NaiveDate::from_ymd_opt(2022, 10, 15)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    let next_weekend_from_monday = get_start_of_repeat_step(&monday, Repetition::WEEKENDS);
    let next_weekend_from_saturday = get_start_of_repeat_step(&saturday, Repetition::WEEKENDS);
    let next_weekend_from_monday_with_time =
        get_start_of_repeat_step(&monday_with_time, Repetition::WEEKENDS);
    let next_weekend_from_saturday_with_time =
        get_start_of_repeat_step(&saturday_with_time, Repetition::WEEKENDS);
    assert_eq!(next_weekend_from_monday, next_weekend_from_monday);
    assert_eq!(next_weekend_from_saturday, next_weekend_from_saturday);
    assert_eq!(next_weekend_from_monday_with_time, next_weekend_from_monday);
    assert_eq!(
        next_weekend_from_saturday_with_time,
        next_weekend_from_saturday
    );
}

#[test]
fn divide_a_day_in_days() {
    let slot_of_exactly_a_day = Slot {
        start: NaiveDate::from_ymd_opt(2022, 09, 26)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap(),
        end: NaiveDate::from_ymd_opt(2022, 09, 27)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap(),
    };
    let exact_day_split_in_days: Vec<Slot> = vec![Slot {
        start: NaiveDate::from_ymd_opt(2022, 09, 26)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap(),
        end: NaiveDate::from_ymd_opt(2022, 09, 27)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap(),
    }];
    let result = slot_of_exactly_a_day.divide_in_days();
    assert_eq!(exact_day_split_in_days, result);
}
#[test]
fn divide_two_days_in_days() {
    let slot_of_exactly_two_day = Slot {
        start: NaiveDate::from_ymd_opt(2022, 09, 26)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap(),
        end: NaiveDate::from_ymd_opt(2022, 09, 28)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap(),
    };
    let exactly_two_days_split_in_days: Vec<Slot> = vec![
        Slot {
            start: NaiveDate::from_ymd_opt(2022, 09, 26)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
            end: NaiveDate::from_ymd_opt(2022, 09, 27)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        },
        Slot {
            start: NaiveDate::from_ymd_opt(2022, 09, 27)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
            end: NaiveDate::from_ymd_opt(2022, 09, 28)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        },
    ];
    let result = slot_of_exactly_two_day.divide_in_days();
    assert_eq!(exactly_two_days_split_in_days, result);
}

#[test]
fn divide_half_a_day_in_days() {
    let slot_of_half_a_day = Slot {
        start: NaiveDate::from_ymd_opt(2022, 10, 01)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap(),
        end: NaiveDate::from_ymd_opt(2022, 10, 01)
            .unwrap()
            .and_hms_opt(6, 0, 0)
            .unwrap(),
    };
    let half_a_day_split_in_days: Vec<Slot> = vec![Slot {
        start: NaiveDate::from_ymd_opt(2022, 10, 01)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap(),
        end: NaiveDate::from_ymd_opt(2022, 10, 01)
            .unwrap()
            .and_hms_opt(6, 0, 0)
            .unwrap(),
    }];
    let result = slot_of_half_a_day.divide_in_days();
    assert_eq!(half_a_day_split_in_days, result);
}

#[test]
fn test_convert_day_object_from_string() {
    let day: Day = Day::from("Tue".to_string());
    assert_eq!(day, Day::Tue);

    let day: Day = Day::from("tue".to_string());
    assert_eq!(day, Day::Tue);

    let day: Day = Day::from("thu".to_string());
    assert_eq!(day, Day::Thu);
}

#[test]
fn test_convert_day_object_into_string() {
    let fri_converted: String = Day::Fri.into();

    let fri_str: String = "Fri".to_string();
    assert_eq!(fri_str, fri_converted);

    let fri_str: String = "FRI".to_string();
    assert_ne!(fri_str, fri_converted);
}
