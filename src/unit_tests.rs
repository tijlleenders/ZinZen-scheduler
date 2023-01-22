use crate::{
    goal::*, input::*, repetition::Repetition, slot::*, task::TaskStatus::*, task::*,
    task_generator::*, task_placer::*, time_slot_iterator::*,
};
use chrono::*;

#[test]
fn time_slot_iterator_splits_into_single_days() {
    let r = TimeSlotIterator::new(
        NaiveDate::from_ymd_opt(2022, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap(),
        NaiveDate::from_ymd_opt(2022, 1, 7)
            .unwrap()
            .and_hms_opt(23, 59, 59)
            .unwrap(),
        Some(Repetition::DAILY(1)),
        0,
        24,
    );

    assert_eq!(
        r.into_iter().collect::<Vec<_>>(),
        vec![
            (Slot {
                start: NaiveDate::from_ymd_opt(2022, 1, 1)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
                end: NaiveDate::from_ymd_opt(2022, 1, 2)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
            }),
            (Slot {
                start: NaiveDate::from_ymd_opt(2022, 1, 2)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
                end: NaiveDate::from_ymd_opt(2022, 1, 3)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
            }),
            (Slot {
                start: NaiveDate::from_ymd_opt(2022, 1, 3)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
                end: NaiveDate::from_ymd_opt(2022, 1, 4)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
            }),
            (Slot {
                start: NaiveDate::from_ymd_opt(2022, 1, 4)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
                end: NaiveDate::from_ymd_opt(2022, 1, 5)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
            }),
            (Slot {
                start: NaiveDate::from_ymd_opt(2022, 1, 5)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
                end: NaiveDate::from_ymd_opt(2022, 1, 6)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
            }),
            (Slot {
                start: NaiveDate::from_ymd_opt(2022, 1, 6)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
                end: NaiveDate::from_ymd_opt(2022, 1, 7)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
            }),
            (Slot {
                start: NaiveDate::from_ymd_opt(2022, 1, 7)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
                end: NaiveDate::from_ymd_opt(2022, 1, 7)
                    .unwrap()
                    .and_hms_opt(23, 59, 59)
                    .unwrap(),
            }),
        ]
    )
}

#[test]
fn time_slot_iterator_returns_all_mondays() {
    let r = TimeSlotIterator::new(
        NaiveDate::from_ymd_opt(2022, 9, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap(),
        NaiveDate::from_ymd_opt(2022, 9, 30)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap(),
        Some(Repetition::MONDAYS),
        0,
        24,
    );

    assert_eq!(
        r.into_iter().collect::<Vec<_>>(),
        vec![
            (Slot {
                start: NaiveDate::from_ymd_opt(2022, 9, 5)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
                end: NaiveDate::from_ymd_opt(2022, 9, 6)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
            }),
            (Slot {
                start: NaiveDate::from_ymd_opt(2022, 9, 12)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
                end: NaiveDate::from_ymd_opt(2022, 9, 13)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
            }),
            (Slot {
                start: NaiveDate::from_ymd_opt(2022, 9, 19)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
                end: NaiveDate::from_ymd_opt(2022, 9, 20)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
            }),
            (Slot {
                start: NaiveDate::from_ymd_opt(2022, 9, 26)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
                end: NaiveDate::from_ymd_opt(2022, 9, 27)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
            }),
        ]
    )
}

fn get_calendar_bounds() -> (NaiveDateTime, NaiveDateTime) {
    (
        (NaiveDate::from_ymd_opt(2022, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()),
        NaiveDate::from_ymd_opt(2022, 1, 2)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap(),
    )
}

#[test]
fn custom_deserialization_of_every_x_days_works() {
    let correct_deserialization = Repetition::EveryXdays(3);
    let string = "\"every 3 days\"";
    let actual_deserialization: Repetition = serde_json::from_str(&string).unwrap();
    assert_eq!(correct_deserialization, actual_deserialization);
}

#[test]
fn removing_slot_from_other_slot_works() {
    let slot_a = Slot {
        start: NaiveDate::from_ymd_opt(2022, 1, 1)
            .unwrap()
            .and_hms_opt(12, 0, 0)
            .unwrap(),
        end: NaiveDate::from_ymd_opt(2022, 1, 1)
            .unwrap()
            .and_hms_opt(18, 0, 0)
            .unwrap(),
    };

    let slot_b = Slot {
        start: NaiveDate::from_ymd_opt(2022, 1, 1)
            .unwrap()
            .and_hms_opt(15, 0, 0)
            .unwrap(),
        end: NaiveDate::from_ymd_opt(2022, 1, 1)
            .unwrap()
            .and_hms_opt(16, 0, 0)
            .unwrap(),
    };

    assert_eq!(
        vec![
            Slot {
                start: NaiveDate::from_ymd_opt(2022, 1, 1)
                    .unwrap()
                    .and_hms_opt(12, 0, 0)
                    .unwrap(),
                end: NaiveDate::from_ymd_opt(2022, 1, 1)
                    .unwrap()
                    .and_hms_opt(15, 0, 0)
                    .unwrap()
            },
            Slot {
                start: NaiveDate::from_ymd_opt(2022, 1, 1)
                    .unwrap()
                    .and_hms_opt(16, 0, 0)
                    .unwrap(),
                end: NaiveDate::from_ymd_opt(2022, 1, 1)
                    .unwrap()
                    .and_hms_opt(18, 0, 0)
                    .unwrap()
            }
        ],
        slot_a - slot_b
    );

    let slot_c = Slot {
        start: NaiveDate::from_ymd_opt(2022, 1, 1)
            .unwrap()
            .and_hms_opt(12, 0, 0)
            .unwrap(),
        end: NaiveDate::from_ymd_opt(2022, 1, 1)
            .unwrap()
            .and_hms_opt(13, 0, 0)
            .unwrap(),
    };

    assert_eq!(
        vec![Slot {
            start: NaiveDate::from_ymd_opt(2022, 1, 1)
                .unwrap()
                .and_hms_opt(13, 0, 0)
                .unwrap(),
            end: NaiveDate::from_ymd_opt(2022, 1, 1)
                .unwrap()
                .and_hms_opt(18, 0, 0)
                .unwrap()
        }],
        slot_a - slot_c
    );

    let slot_d = Slot {
        start: NaiveDate::from_ymd_opt(2022, 1, 1)
            .unwrap()
            .and_hms_opt(17, 0, 0)
            .unwrap(),
        end: NaiveDate::from_ymd_opt(2022, 1, 1)
            .unwrap()
            .and_hms_opt(18, 0, 0)
            .unwrap(),
    };

    assert_eq!(
        vec![Slot {
            start: NaiveDate::from_ymd_opt(2022, 1, 1)
                .unwrap()
                .and_hms_opt(12, 0, 0)
                .unwrap(),
            end: NaiveDate::from_ymd_opt(2022, 1, 1)
                .unwrap()
                .and_hms_opt(17, 0, 0)
                .unwrap()
        }],
        slot_a - slot_d
    );

    let slot_e = Slot {
        start: NaiveDate::from_ymd_opt(2022, 1, 1)
            .unwrap()
            .and_hms_opt(12, 0, 0)
            .unwrap(),
        end: NaiveDate::from_ymd_opt(2022, 1, 1)
            .unwrap()
            .and_hms_opt(18, 0, 0)
            .unwrap(),
    };

    assert_eq!(Vec::<Slot>::new(), slot_a - slot_e);

    let slot_f = Slot {
        start: NaiveDate::from_ymd_opt(2022, 1, 1)
            .unwrap()
            .and_hms_opt(10, 0, 0)
            .unwrap(),
        end: NaiveDate::from_ymd_opt(2022, 1, 1)
            .unwrap()
            .and_hms_opt(13, 0, 0)
            .unwrap(),
    };

    assert_eq!(
        vec![Slot {
            start: NaiveDate::from_ymd_opt(2022, 1, 1)
                .unwrap()
                .and_hms_opt(13, 0, 0)
                .unwrap(),
            end: NaiveDate::from_ymd_opt(2022, 1, 1)
                .unwrap()
                .and_hms_opt(18, 0, 0)
                .unwrap()
        }],
        slot_a - slot_f
    );

    let slot_g = Slot {
        start: NaiveDate::from_ymd_opt(2022, 1, 1)
            .unwrap()
            .and_hms_opt(10, 0, 0)
            .unwrap(),
        end: NaiveDate::from_ymd_opt(2022, 1, 1)
            .unwrap()
            .and_hms_opt(18, 0, 0)
            .unwrap(),
    };

    assert_eq!(Vec::<Slot>::new(), slot_a - slot_g);

    let slot_h = Slot {
        start: NaiveDate::from_ymd_opt(2022, 1, 1)
            .unwrap()
            .and_hms_opt(15, 0, 0)
            .unwrap(),
        end: NaiveDate::from_ymd_opt(2022, 1, 1)
            .unwrap()
            .and_hms_opt(19, 0, 0)
            .unwrap(),
    };

    assert_eq!(
        vec![Slot {
            start: NaiveDate::from_ymd_opt(2022, 1, 1)
                .unwrap()
                .and_hms_opt(12, 0, 0)
                .unwrap(),
            end: NaiveDate::from_ymd_opt(2022, 1, 1)
                .unwrap()
                .and_hms_opt(15, 0, 0)
                .unwrap()
        }],
        slot_a - slot_h
    );
}

#[test]
fn adding_slots_to_each_other_works() {
    let slot_a = Slot {
        start: NaiveDate::from_ymd_opt(2022, 1, 1)
            .unwrap()
            .and_hms_opt(12, 0, 0)
            .unwrap(),
        end: NaiveDate::from_ymd_opt(2022, 1, 1)
            .unwrap()
            .and_hms_opt(18, 0, 0)
            .unwrap(),
    };

    let slot_b = Slot {
        start: NaiveDate::from_ymd_opt(2022, 1, 1)
            .unwrap()
            .and_hms_opt(18, 0, 0)
            .unwrap(),
        end: NaiveDate::from_ymd_opt(2022, 1, 1)
            .unwrap()
            .and_hms_opt(19, 0, 0)
            .unwrap(),
    };

    assert_eq!(
        Slot {
            start: NaiveDate::from_ymd_opt(2022, 1, 1)
                .unwrap()
                .and_hms_opt(12, 0, 0)
                .unwrap(),
            end: NaiveDate::from_ymd_opt(2022, 1, 1)
                .unwrap()
                .and_hms_opt(19, 0, 0)
                .unwrap()
        },
        slot_a + slot_b
    );

    let slot_c = Slot {
        start: NaiveDate::from_ymd_opt(2022, 1, 1)
            .unwrap()
            .and_hms_opt(11, 0, 0)
            .unwrap(),
        end: NaiveDate::from_ymd_opt(2022, 1, 1)
            .unwrap()
            .and_hms_opt(12, 0, 0)
            .unwrap(),
    };

    assert_eq!(
        Slot {
            start: NaiveDate::from_ymd_opt(2022, 1, 1)
                .unwrap()
                .and_hms_opt(11, 0, 0)
                .unwrap(),
            end: NaiveDate::from_ymd_opt(2022, 1, 1)
                .unwrap()
                .and_hms_opt(18, 0, 0)
                .unwrap()
        },
        slot_a + slot_c
    );

    let slot_d = Slot {
        start: NaiveDate::from_ymd_opt(2022, 1, 1)
            .unwrap()
            .and_hms_opt(18, 0, 0)
            .unwrap(),
        end: NaiveDate::from_ymd_opt(2022, 1, 1)
            .unwrap()
            .and_hms_opt(19, 0, 0)
            .unwrap(),
    };

    let slot_e = Slot {
        start: NaiveDate::from_ymd_opt(2022, 1, 1)
            .unwrap()
            .and_hms_opt(19, 0, 0)
            .unwrap(),
        end: NaiveDate::from_ymd_opt(2022, 1, 1)
            .unwrap()
            .and_hms_opt(20, 0, 0)
            .unwrap(),
    };

    let slot_f = Slot {
        start: NaiveDate::from_ymd_opt(2022, 1, 1)
            .unwrap()
            .and_hms_opt(20, 0, 0)
            .unwrap(),
        end: NaiveDate::from_ymd_opt(2022, 1, 1)
            .unwrap()
            .and_hms_opt(21, 0, 0)
            .unwrap(),
    };

    assert_eq!(
        Slot {
            start: NaiveDate::from_ymd_opt(2022, 1, 1)
                .unwrap()
                .and_hms_opt(12, 0, 0)
                .unwrap(),
            end: NaiveDate::from_ymd_opt(2022, 1, 1)
                .unwrap()
                .and_hms_opt(21, 0, 0)
                .unwrap()
        },
        slot_a + slot_d + slot_e + slot_f
    );

    let slot_g = Slot {
        start: NaiveDate::from_ymd_opt(2022, 1, 1)
            .unwrap()
            .and_hms_opt(10, 0, 0)
            .unwrap(),
        end: NaiveDate::from_ymd_opt(2022, 1, 1)
            .unwrap()
            .and_hms_opt(11, 0, 0)
            .unwrap(),
    };

    assert_eq!(
        Slot {
            start: NaiveDate::from_ymd_opt(2022, 1, 1)
                .unwrap()
                .and_hms_opt(12, 0, 0)
                .unwrap(),
            end: NaiveDate::from_ymd_opt(2022, 1, 1)
                .unwrap()
                .and_hms_opt(18, 0, 0)
                .unwrap()
        },
        slot_a + slot_g
    );

    let slot_h = Slot {
        start: NaiveDate::from_ymd_opt(2022, 1, 1)
            .unwrap()
            .and_hms_opt(19, 0, 0)
            .unwrap(),
        end: NaiveDate::from_ymd_opt(2022, 1, 1)
            .unwrap()
            .and_hms_opt(20, 0, 0)
            .unwrap(),
    };

    assert_eq!(
        Slot {
            start: NaiveDate::from_ymd_opt(2022, 1, 1)
                .unwrap()
                .and_hms_opt(12, 0, 0)
                .unwrap(),
            end: NaiveDate::from_ymd_opt(2022, 1, 1)
                .unwrap()
                .and_hms_opt(18, 0, 0)
                .unwrap()
        },
        slot_a + slot_h
    );
}

fn get_calendar_bounds_2() -> (NaiveDateTime, NaiveDateTime) {
    (
        (NaiveDate::from_ymd_opt(2022, 10, 23)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()),
        NaiveDate::from_ymd_opt(2022, 11, 3)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap(),
    )
}

#[test]
fn slot_generator_assigns_contiguous_slots() {
    let (calendar_start, calendar_end) = get_calendar_bounds_2();
    let goal1 = Goal::new(1)
        .duration(2)
        .after_time(10)
        .before_time(14)
        .start(
            NaiveDate::from_ymd_opt(2022, 10, 24)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        )
        .deadline(
            NaiveDate::from_ymd_opt(2022, 10, 29)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        );
    let goal2 = Goal::new(2)
        .duration(1)
        .after_time(10)
        .before_time(14)
        .start(
            NaiveDate::from_ymd_opt(2022, 11, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        )
        .deadline(
            NaiveDate::from_ymd_opt(2022, 11, 2)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        );
    let goal3 = Goal::new(3)
        .duration(1)
        .after_time(10)
        .before_time(11)
        .start(
            NaiveDate::from_ymd_opt(2022, 11, 2)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        )
        .deadline(
            NaiveDate::from_ymd_opt(2022, 11, 3)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        );
    let goals = vec![goal1, goal2, goal3];
    let tasks = task_generator(Input {
        calendar_start,
        calendar_end,
        goals,
    });
    assert_eq!(tasks[0].slots.len(), 5);
    assert_eq!(tasks[1].slots.len(), 1);
    assert_eq!(tasks[2].slots.len(), 1);
    assert_eq!(
        tasks[0].slots[0].start,
        NaiveDate::from_ymd_opt(2022, 10, 24)
            .unwrap()
            .and_hms_opt(10, 0, 0)
            .unwrap()
    );
    assert_eq!(
        tasks[0].slots[0].end,
        NaiveDate::from_ymd_opt(2022, 10, 24)
            .unwrap()
            .and_hms_opt(14, 0, 0)
            .unwrap()
    );
    assert_eq!(
        tasks[0].slots[1].start,
        NaiveDate::from_ymd_opt(2022, 10, 25)
            .unwrap()
            .and_hms_opt(10, 0, 0)
            .unwrap()
    );
    assert_eq!(
        tasks[0].slots[1].end,
        NaiveDate::from_ymd_opt(2022, 10, 25)
            .unwrap()
            .and_hms_opt(14, 0, 0)
            .unwrap()
    );

    assert_eq!(
        tasks[0].slots[2].start,
        NaiveDate::from_ymd_opt(2022, 10, 26)
            .unwrap()
            .and_hms_opt(10, 0, 0)
            .unwrap()
    );
    assert_eq!(
        tasks[0].slots[2].end,
        NaiveDate::from_ymd_opt(2022, 10, 26)
            .unwrap()
            .and_hms_opt(14, 0, 0)
            .unwrap()
    );

    assert_eq!(
        tasks[0].slots[3].start,
        NaiveDate::from_ymd_opt(2022, 10, 27)
            .unwrap()
            .and_hms_opt(10, 0, 0)
            .unwrap()
    );
    assert_eq!(
        tasks[0].slots[3].end,
        NaiveDate::from_ymd_opt(2022, 10, 27)
            .unwrap()
            .and_hms_opt(14, 0, 0)
            .unwrap()
    );

    assert_eq!(
        tasks[0].slots[4].start,
        NaiveDate::from_ymd_opt(2022, 10, 28)
            .unwrap()
            .and_hms_opt(10, 0, 0)
            .unwrap()
    );
    assert_eq!(
        tasks[0].slots[4].end,
        NaiveDate::from_ymd_opt(2022, 10, 28)
            .unwrap()
            .and_hms_opt(14, 0, 0)
            .unwrap()
    );

    assert_eq!(
        tasks[1].slots[0].start,
        NaiveDate::from_ymd_opt(2022, 11, 1)
            .unwrap()
            .and_hms_opt(10, 0, 0)
            .unwrap()
    );
    assert_eq!(
        tasks[1].slots[0].end,
        NaiveDate::from_ymd_opt(2022, 11, 1)
            .unwrap()
            .and_hms_opt(14, 0, 0)
            .unwrap()
    );
}

#[test]
fn vec_of_tasks_sorts_flex1_then_high_to_low_works() {
    let (calendar_start, calendar_end) = get_calendar_bounds();
    //will generate task of flex 1
    let goal1 = Goal::new(1)
        .duration(1)
        .after_time(10)
        .before_time(11)
        .start(
            NaiveDate::from_ymd_opt(2022, 1, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        )
        .deadline(
            NaiveDate::from_ymd_opt(2022, 1, 2)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        );
    //will generate task of flex 2
    let goal2 = Goal::new(2)
        .duration(1)
        .after_time(10)
        .before_time(12)
        .start(
            NaiveDate::from_ymd_opt(2022, 1, 2)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        )
        .deadline(
            NaiveDate::from_ymd_opt(2022, 1, 3)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        );
    //will generate task of flex 3
    let goal3 = Goal::new(3)
        .duration(1)
        .after_time(10)
        .before_time(13)
        .start(
            NaiveDate::from_ymd_opt(2022, 1, 3)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        )
        .deadline(
            NaiveDate::from_ymd_opt(2022, 1, 4)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        );
    //will generate task of flex 4
    let goal4 = Goal::new(4)
        .duration(1)
        .after_time(10)
        .before_time(14)
        .start(
            NaiveDate::from_ymd_opt(2022, 1, 4)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        )
        .deadline(
            NaiveDate::from_ymd_opt(2022, 1, 5)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        );
    //will generate task of flex 5
    let goal5 = Goal::new(5)
        .duration(1)
        .after_time(10)
        .before_time(15)
        .start(
            NaiveDate::from_ymd_opt(2022, 1, 5)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        )
        .deadline(
            NaiveDate::from_ymd_opt(2022, 1, 6)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        );

    let goals = vec![goal1, goal2, goal3, goal4, goal5];
    let mut tasks: Vec<Task> = task_generator(Input {
        calendar_start,
        calendar_end,
        goals,
    });
    tasks.sort();
    assert_eq!(tasks[0].goal_id, 1.to_string());
    assert_eq!(tasks[1].goal_id, 5.to_string());
    assert_eq!(tasks[2].goal_id, 4.to_string());
    assert_eq!(tasks[3].goal_id, 3.to_string());
    assert_eq!(tasks[4].goal_id, 2.to_string());
}

#[test]
fn vec_of_tasks_sorts_multiple_flex1_then_high_to_low_works() {
    let (calendar_start, calendar_end) = get_calendar_bounds();
    //will generate task of flex 1
    let goal1 = Goal::new(1)
        .duration(1)
        .after_time(10)
        .before_time(11)
        .start(
            NaiveDate::from_ymd_opt(2022, 1, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        )
        .deadline(
            NaiveDate::from_ymd_opt(2022, 1, 2)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        );
    //will generate task of flex 1
    let goal2 = Goal::new(2)
        .duration(1)
        .after_time(10)
        .before_time(11)
        .start(
            NaiveDate::from_ymd_opt(2022, 1, 2)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        )
        .deadline(
            NaiveDate::from_ymd_opt(2022, 1, 3)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        );
    //will generate task of flex 1
    let goal3 = Goal::new(3)
        .duration(1)
        .after_time(10)
        .before_time(11)
        .start(
            NaiveDate::from_ymd_opt(2022, 1, 3)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        )
        .deadline(
            NaiveDate::from_ymd_opt(2022, 1, 4)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        );
    //will generate task of flex 4
    let goal4 = Goal::new(4)
        .duration(1)
        .after_time(10)
        .before_time(14)
        .start(
            NaiveDate::from_ymd_opt(2022, 1, 4)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        )
        .deadline(
            NaiveDate::from_ymd_opt(2022, 1, 5)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        );
    //will generate task of flex 5
    let goal5 = Goal::new(5)
        .duration(1)
        .after_time(10)
        .before_time(15)
        .start(
            NaiveDate::from_ymd_opt(2022, 1, 5)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        )
        .deadline(
            NaiveDate::from_ymd_opt(2022, 1, 6)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        );

    let goals = vec![goal1, goal2, goal3, goal4, goal5];
    let mut tasks: Vec<Task> = task_generator(Input {
        calendar_start,
        calendar_end,
        goals,
    });
    tasks.sort();
    assert_eq!(tasks[0].goal_id, 1.to_string());
    assert_eq!(tasks[1].goal_id, 2.to_string());
    assert_eq!(tasks[2].goal_id, 3.to_string());
    assert_eq!(tasks[3].goal_id, 5.to_string());
    assert_eq!(tasks[4].goal_id, 4.to_string());
}

#[test]
fn vec_of_tasks_sorts_no_flex1_then_high_to_low_works() {
    let (calendar_start, calendar_end) = get_calendar_bounds();
    //will generate task of flex 2
    let goal1 = Goal::new(1)
        .duration(1)
        .after_time(10)
        .before_time(12)
        .start(
            NaiveDate::from_ymd_opt(2022, 1, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        )
        .deadline(
            NaiveDate::from_ymd_opt(2022, 1, 2)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        );
    //will generate task of flex 3
    let goal2 = Goal::new(2)
        .duration(1)
        .after_time(10)
        .before_time(13)
        .start(
            NaiveDate::from_ymd_opt(2022, 1, 2)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        )
        .deadline(
            NaiveDate::from_ymd_opt(2022, 1, 3)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        );
    //will generate task of flex 4
    let goal3 = Goal::new(3)
        .duration(1)
        .after_time(10)
        .before_time(14)
        .start(
            NaiveDate::from_ymd_opt(2022, 1, 3)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        )
        .deadline(
            NaiveDate::from_ymd_opt(2022, 1, 4)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        );
    //will generate task of flex 5
    let goal4 = Goal::new(4)
        .duration(1)
        .after_time(10)
        .before_time(15)
        .start(
            NaiveDate::from_ymd_opt(2022, 1, 4)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        )
        .deadline(
            NaiveDate::from_ymd_opt(2022, 1, 5)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        );
    //will generate task of flex 6
    let goal5 = Goal::new(5)
        .duration(1)
        .after_time(10)
        .before_time(16)
        .start(
            NaiveDate::from_ymd_opt(2022, 1, 5)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        )
        .deadline(
            NaiveDate::from_ymd_opt(2022, 1, 6)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        );

    let goals = vec![goal1, goal2, goal3, goal4, goal5];
    let mut tasks: Vec<Task> = task_generator(Input {
        calendar_start,
        calendar_end,
        goals,
    });
    tasks.sort();
    assert_eq!(tasks[0].goal_id, 5.to_string());
    assert_eq!(tasks[1].goal_id, 4.to_string());
    assert_eq!(tasks[2].goal_id, 3.to_string());
    assert_eq!(tasks[3].goal_id, 2.to_string());
    assert_eq!(tasks[4].goal_id, 1.to_string());
}

#[test]
fn task_placer_returns_impossible_tasks() {
    let (calendar_start, calendar_end) = get_calendar_bounds();
    let goal1 = Goal::new(1)
        .title("dentist")
        .duration(1)
        .after_time(10)
        .before_time(11)
        .start(
            NaiveDate::from_ymd_opt(2022, 1, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        )
        .deadline(
            NaiveDate::from_ymd_opt(2022, 1, 2)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        );
    let goal2 = Goal::new(2)
        .title("shopping")
        .duration(1)
        .after_time(10)
        .before_time(13)
        .start(
            NaiveDate::from_ymd_opt(2022, 1, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        )
        .deadline(
            NaiveDate::from_ymd_opt(2022, 1, 2)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        );
    let goal3 = Goal::new(3)
        .title("exercise")
        .duration(1)
        .after_time(10)
        .before_time(18)
        .start(
            NaiveDate::from_ymd_opt(2022, 1, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        )
        .deadline(
            NaiveDate::from_ymd_opt(2022, 1, 2)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        );
    let goal4 = Goal::new(4)
        .title("go to bank")
        .duration(1)
        .after_time(10)
        .before_time(11)
        .start(
            NaiveDate::from_ymd_opt(2022, 1, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        )
        .deadline(
            NaiveDate::from_ymd_opt(2022, 1, 2)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        );
    let goals = vec![goal1, goal2, goal3, goal4];
    let tasks = task_generator(Input {
        calendar_start,
        calendar_end,
        goals,
    });
    dbg!(&tasks);
    let (scheduled_tasks, impossible_tasks) = task_placer(tasks);
    dbg!(&impossible_tasks);
    assert_eq!(scheduled_tasks[0].status, Scheduled);
    assert_eq!(scheduled_tasks[1].status, Scheduled);
    assert_eq!(scheduled_tasks[2].status, Scheduled);
    assert_eq!(impossible_tasks[0].status, Impossible);

    assert_eq!(
        scheduled_tasks[0].confirmed_start.unwrap(),
        NaiveDate::from_ymd_opt(2022, 1, 1)
            .unwrap()
            .and_hms_opt(10, 0, 0)
            .unwrap()
    );
    assert_eq!(
        scheduled_tasks[0].confirmed_deadline.unwrap(),
        NaiveDate::from_ymd_opt(2022, 1, 1)
            .unwrap()
            .and_hms_opt(11, 0, 0)
            .unwrap()
    );
    assert_eq!(
        scheduled_tasks[1].confirmed_start.unwrap(),
        NaiveDate::from_ymd_opt(2022, 1, 1)
            .unwrap()
            .and_hms_opt(13, 0, 0)
            .unwrap()
    );
    assert_eq!(
        scheduled_tasks[1].confirmed_deadline.unwrap(),
        NaiveDate::from_ymd_opt(2022, 1, 1)
            .unwrap()
            .and_hms_opt(14, 0, 0)
            .unwrap()
    );
    assert_eq!(
        scheduled_tasks[2].confirmed_start.unwrap(),
        NaiveDate::from_ymd_opt(2022, 1, 1)
            .unwrap()
            .and_hms_opt(11, 0, 0)
            .unwrap()
    );
    assert_eq!(
        scheduled_tasks[2].confirmed_deadline.unwrap(),
        NaiveDate::from_ymd_opt(2022, 1, 1)
            .unwrap()
            .and_hms_opt(12, 0, 0)
            .unwrap()
    );
    assert_eq!(
        impossible_tasks[0].conflicts[0],
        (
            Slot {
                start: NaiveDate::from_ymd_opt(2022, 1, 1)
                    .unwrap()
                    .and_hms_opt(10, 0, 0)
                    .unwrap(),
                end: NaiveDate::from_ymd_opt(2022, 1, 1)
                    .unwrap()
                    .and_hms_opt(11, 0, 0)
                    .unwrap()
            },
            "1".to_owned()
        )
    );
}

#[test]
fn custom_deserialization_of_flex_repeat_works() {
    let correct_deserialization = Repetition::FlexWeekly(3, 5);
    let string = "\"3-5/week\"";
    let actual_deserialization: Repetition = serde_json::from_str(&string).unwrap();
    assert_eq!(correct_deserialization, actual_deserialization);

    let correct_deserialization = Repetition::FlexDaily(3, 5);
    let string = "\"3-5/day\"";
    let actual_deserialization: Repetition = serde_json::from_str(&string).unwrap();
    assert_eq!(correct_deserialization, actual_deserialization);
}

#[test]
fn custom_serialization_of_duration_works() {
    let string = "\"35-40h\"";
    let correct_deserialization = GoalDuration(35, Some(40));
    let actual_deserialization: GoalDuration = serde_json::from_str(&string).unwrap();
    assert_eq!(correct_deserialization, actual_deserialization);
}
