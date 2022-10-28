use self::serde::*;
use crate::{
    goal::*, input::*, output_formatter::*, slot::*, task::TaskStatus::*, task::*,
    task_generator::*, task_placer::*, time_slot_iterator::*,
};
use chrono::*;

fn get_test_tasks() -> Vec<Task> {
    vec![
        Task {
            id: 20,
            goal_id: 2,
            title: "dentist".to_string(),
            duration: 1,
            status: TaskStatus::UNSCHEDULED,
            flexibility: 0,
            start: NaiveDate::from_ymd(2022, 1, 1).and_hms(0, 0, 0),
            deadline: NaiveDate::from_ymd(2022, 1, 2).and_hms(0, 0, 0),
            after_time: 10,
            before_time: 11,
            slots: Vec::new(),
            confirmed_start: None,
            confirmed_deadline: None,
            internal_marker: NaiveDate::from_ymd(2022, 1, 1).and_hms(10, 0, 0),
        },
        Task {
            id: 10,
            goal_id: 1,
            title: "shopping".to_string(),
            duration: 1,
            status: TaskStatus::UNSCHEDULED,
            flexibility: 0,
            start: NaiveDate::from_ymd(2022, 1, 1).and_hms(0, 0, 0),
            deadline: NaiveDate::from_ymd(2022, 1, 2).and_hms(0, 0, 0),
            after_time: 10,
            before_time: 13,
            slots: Vec::new(),
            confirmed_start: None,
            confirmed_deadline: None,
            internal_marker: NaiveDate::from_ymd(2022, 1, 1).and_hms(10, 0, 0),
        },
        Task {
            id: 30,
            goal_id: 3,
            title: "exercise".to_string(),
            duration: 1,
            status: TaskStatus::UNSCHEDULED,
            flexibility: 0,
            start: NaiveDate::from_ymd(2022, 1, 1).and_hms(0, 0, 0),
            deadline: NaiveDate::from_ymd(2022, 1, 2).and_hms(0, 0, 0),
            after_time: 10,
            before_time: 18,
            slots: Vec::new(),
            confirmed_start: None,
            confirmed_deadline: None,
            internal_marker: NaiveDate::from_ymd(2022, 1, 1).and_hms(10, 0, 0),
        },
    ]
}

fn get_calendar_bounds() -> (NaiveDateTime, NaiveDateTime) {
    (
        (NaiveDate::from_ymd(2022, 1, 1).and_hms(0, 0, 0)),
        NaiveDate::from_ymd(2022, 1, 2).and_hms(0, 0, 0),
    )
}

#[test]
fn time_slot_iterator_splits_into_single_days() {
    let r = TimeSlotIterator {
        start: NaiveDate::from_ymd(2022, 1, 1).and_hms(0, 0, 0),
        end: NaiveDate::from_ymd(2022, 1, 7).and_hms(23, 59, 59),
        repetition: Repetition::DAILY,
    };

    assert_eq!(
        r.into_iter().collect::<Vec<_>>(),
        vec![
            (Slot {
                start: NaiveDate::from_ymd(2022, 1, 1).and_hms(0, 0, 0),
                end: NaiveDate::from_ymd(2022, 1, 2).and_hms(0, 0, 0),
            }),
            (Slot {
                start: NaiveDate::from_ymd(2022, 1, 2).and_hms(0, 0, 0),
                end: NaiveDate::from_ymd(2022, 1, 3).and_hms(0, 0, 0),
            }),
            (Slot {
                start: NaiveDate::from_ymd(2022, 1, 3).and_hms(0, 0, 0),
                end: NaiveDate::from_ymd(2022, 1, 4).and_hms(0, 0, 0),
            }),
            (Slot {
                start: NaiveDate::from_ymd(2022, 1, 4).and_hms(0, 0, 0),
                end: NaiveDate::from_ymd(2022, 1, 5).and_hms(0, 0, 0),
            }),
            (Slot {
                start: NaiveDate::from_ymd(2022, 1, 5).and_hms(0, 0, 0),
                end: NaiveDate::from_ymd(2022, 1, 6).and_hms(0, 0, 0),
            }),
            (Slot {
                start: NaiveDate::from_ymd(2022, 1, 6).and_hms(0, 0, 0),
                end: NaiveDate::from_ymd(2022, 1, 7).and_hms(0, 0, 0),
            }),
            (Slot {
                start: NaiveDate::from_ymd(2022, 1, 7).and_hms(0, 0, 0),
                end: NaiveDate::from_ymd(2022, 1, 7).and_hms(23, 59, 59),
            }),
        ]
    )
}

#[test]
fn time_slot_iterator_returns_all_mondays() {
    let r = TimeSlotIterator {
        start: NaiveDate::from_ymd(2022, 9, 1).and_hms(0, 0, 0),
        end: NaiveDate::from_ymd(2022, 9, 30).and_hms(0, 0, 0),
        repetition: Repetition::MONDAYS,
    };

    assert_eq!(
        r.into_iter().collect::<Vec<_>>(),
        vec![
            (Slot {
                start: NaiveDate::from_ymd(2022, 9, 5).and_hms(0, 0, 0),
                end: NaiveDate::from_ymd(2022, 9, 6).and_hms(0, 0, 0),
            }),
            (Slot {
                start: NaiveDate::from_ymd(2022, 9, 12).and_hms(0, 0, 0),
                end: NaiveDate::from_ymd(2022, 9, 13).and_hms(0, 0, 0),
            }),
            (Slot {
                start: NaiveDate::from_ymd(2022, 9, 19).and_hms(0, 0, 0),
                end: NaiveDate::from_ymd(2022, 9, 20).and_hms(0, 0, 0),
            }),
            (Slot {
                start: NaiveDate::from_ymd(2022, 9, 26).and_hms(0, 0, 0),
                end: NaiveDate::from_ymd(2022, 9, 27).and_hms(0, 0, 0),
            }),
        ]
    )
}

#[test]
fn goal_generates_single_nonrepetitive_task() {
    let goal = Goal::new(1)
        .duration(1)
        .start(NaiveDate::from_ymd(2022, 1, 1).and_hms(0, 0, 0))
        .deadline(NaiveDate::from_ymd(2022, 1, 4).and_hms(0, 0, 0));
    let mut counter = 0;
    assert_eq!(
        goal.generate_tasks(
            NaiveDate::from_ymd(2022, 1, 1).and_hms(0, 0, 0),
            NaiveDate::from_ymd(2022, 1, 4).and_hms(0, 0, 0),
            &mut counter
        ),
        vec![Task {
            id: 0,
            goal_id: 1,
            title: "Test".to_string(),
            duration: 1,
            status: TaskStatus::UNSCHEDULED,
            flexibility: 0,
            start: NaiveDate::from_ymd(2022, 1, 1).and_hms(0, 0, 0),
            deadline: NaiveDate::from_ymd(2022, 1, 4).and_hms(0, 0, 0),
            after_time: 0,
            before_time: 24,
            slots: Vec::new(),
            confirmed_start: None,
            confirmed_deadline: None,
            internal_marker: NaiveDate::from_ymd(2022, 1, 1).and_hms(0, 0, 0),
        },]
    )
}

#[test]
fn output_formatter_works() {
    let desired_output = r#"[{"taskid":0,"goalid":2,"title":"dentist","duration":1,"start":"2022-01-01T10:00:00","deadline":"2022-01-01T11:00:00"},{"taskid":1,"goalid":1,"title":"shopping","duration":1,"start":"2022-01-01T11:00:00","deadline":"2022-01-01T12:00:00"},{"taskid":2,"goalid":3,"title":"exercise","duration":1,"start":"2022-01-01T13:00:00","deadline":"2022-01-01T14:00:00"}]"#;

    let (calendar_start, calendar_end) = get_calendar_bounds();
    let scheduled_tasks = task_placer(get_test_tasks(), calendar_start, calendar_end);
    let output = output_formatter(scheduled_tasks).unwrap();
    assert_eq!(desired_output, serde_json::to_string(&output).unwrap());
}

#[test]
fn task_placer_slots_tasks_correctly() {
    let tasks = get_test_tasks();
    let (calendar_start, calendar_end) = get_calendar_bounds();
    let scheduled_tasks = task_placer(tasks, calendar_start, calendar_end);
    assert_eq!(scheduled_tasks[0].status, SCHEDULED);
    assert_eq!(scheduled_tasks[1].status, SCHEDULED);
    assert_eq!(scheduled_tasks[2].status, SCHEDULED);

    assert_eq!(
        scheduled_tasks[0].confirmed_start.unwrap(),
        NaiveDate::from_ymd(2022, 1, 1).and_hms(13, 0, 0)
    );
    assert_eq!(
        scheduled_tasks[0].confirmed_deadline.unwrap(),
        NaiveDate::from_ymd(2022, 1, 1).and_hms(14, 0, 0)
    );
    assert_eq!(
        scheduled_tasks[1].confirmed_start.unwrap(),
        NaiveDate::from_ymd(2022, 1, 1).and_hms(11, 0, 0)
    );
    assert_eq!(
        scheduled_tasks[1].confirmed_deadline.unwrap(),
        NaiveDate::from_ymd(2022, 1, 1).and_hms(12, 0, 0)
    );
    assert_eq!(
        scheduled_tasks[2].confirmed_start.unwrap(),
        NaiveDate::from_ymd(2022, 1, 1).and_hms(10, 0, 0)
    );
    assert_eq!(
        scheduled_tasks[2].confirmed_deadline.unwrap(),
        NaiveDate::from_ymd(2022, 1, 1).and_hms(11, 0, 0)
    );
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
        start: NaiveDate::from_ymd(2022, 1, 1).and_hms(12, 0, 0),
        end: NaiveDate::from_ymd(2022, 1, 1).and_hms(18, 0, 0),
    };

    let slot_b = Slot {
        start: NaiveDate::from_ymd(2022, 1, 1).and_hms(15, 0, 0),
        end: NaiveDate::from_ymd(2022, 1, 1).and_hms(16, 0, 0),
    };

    assert_eq!(
        vec![
            Slot {
                start: NaiveDate::from_ymd(2022, 1, 1).and_hms(12, 0, 0),
                end: NaiveDate::from_ymd(2022, 1, 1).and_hms(15, 0, 0)
            },
            Slot {
                start: NaiveDate::from_ymd(2022, 1, 1).and_hms(16, 0, 0),
                end: NaiveDate::from_ymd(2022, 1, 1).and_hms(18, 0, 0)
            }
        ],
        slot_a - slot_b
    );

    let slot_c = Slot {
        start: NaiveDate::from_ymd(2022, 1, 1).and_hms(12, 0, 0),
        end: NaiveDate::from_ymd(2022, 1, 1).and_hms(13, 0, 0),
    };

    assert_eq!(
        vec![Slot {
            start: NaiveDate::from_ymd(2022, 1, 1).and_hms(13, 0, 0),
            end: NaiveDate::from_ymd(2022, 1, 1).and_hms(18, 0, 0)
        }],
        slot_a - slot_c
    );

    let slot_d = Slot {
        start: NaiveDate::from_ymd(2022, 1, 1).and_hms(17, 0, 0),
        end: NaiveDate::from_ymd(2022, 1, 1).and_hms(18, 0, 0),
    };

    assert_eq!(
        vec![Slot {
            start: NaiveDate::from_ymd(2022, 1, 1).and_hms(12, 0, 0),
            end: NaiveDate::from_ymd(2022, 1, 1).and_hms(17, 0, 0)
        }],
        slot_a - slot_d
    );

    let slot_e = Slot {
        start: NaiveDate::from_ymd(2022, 1, 1).and_hms(12, 0, 0),
        end: NaiveDate::from_ymd(2022, 1, 1).and_hms(18, 0, 0),
    };

    assert_eq!(Vec::<Slot>::new(), slot_a - slot_e);

    let slot_f = Slot {
        start: NaiveDate::from_ymd(2022, 1, 1).and_hms(10, 0, 0),
        end: NaiveDate::from_ymd(2022, 1, 1).and_hms(13, 0, 0),
    };

    assert_eq!(
        vec![Slot {
            start: NaiveDate::from_ymd(2022, 1, 1).and_hms(13, 0, 0),
            end: NaiveDate::from_ymd(2022, 1, 1).and_hms(18, 0, 0)
        }],
        slot_a - slot_f
    );

    let slot_g = Slot {
        start: NaiveDate::from_ymd(2022, 1, 1).and_hms(10, 0, 0),
        end: NaiveDate::from_ymd(2022, 1, 1).and_hms(18, 0, 0),
    };

    assert_eq!(Vec::<Slot>::new(), slot_a - slot_g);

    let slot_h = Slot {
        start: NaiveDate::from_ymd(2022, 1, 1).and_hms(15, 0, 0),
        end: NaiveDate::from_ymd(2022, 1, 1).and_hms(19, 0, 0),
    };

    assert_eq!(
        vec![Slot {
            start: NaiveDate::from_ymd(2022, 1, 1).and_hms(12, 0, 0),
            end: NaiveDate::from_ymd(2022, 1, 1).and_hms(15, 0, 0)
        }],
        slot_a - slot_h
    );
}

#[test]
fn adding_slots_to_each_other_works() {
    let slot_a = Slot {
        start: NaiveDate::from_ymd(2022, 1, 1).and_hms(12, 0, 0),
        end: NaiveDate::from_ymd(2022, 1, 1).and_hms(18, 0, 0),
    };

    let slot_b = Slot {
        start: NaiveDate::from_ymd(2022, 1, 1).and_hms(18, 0, 0),
        end: NaiveDate::from_ymd(2022, 1, 1).and_hms(19, 0, 0),
    };

    assert_eq!(
        Slot {
            start: NaiveDate::from_ymd(2022, 1, 1).and_hms(12, 0, 0),
            end: NaiveDate::from_ymd(2022, 1, 1).and_hms(19, 0, 0)
        },
        slot_a + slot_b
    );

    let slot_c = Slot {
        start: NaiveDate::from_ymd(2022, 1, 1).and_hms(11, 0, 0),
        end: NaiveDate::from_ymd(2022, 1, 1).and_hms(12, 0, 0),
    };

    assert_eq!(
        Slot {
            start: NaiveDate::from_ymd(2022, 1, 1).and_hms(11, 0, 0),
            end: NaiveDate::from_ymd(2022, 1, 1).and_hms(18, 0, 0)
        },
        slot_a + slot_c
    );

    let slot_d = Slot {
        start: NaiveDate::from_ymd(2022, 1, 1).and_hms(18, 0, 0),
        end: NaiveDate::from_ymd(2022, 1, 1).and_hms(19, 0, 0),
    };

    let slot_e = Slot {
        start: NaiveDate::from_ymd(2022, 1, 1).and_hms(19, 0, 0),
        end: NaiveDate::from_ymd(2022, 1, 1).and_hms(20, 0, 0),
    };

    let slot_f = Slot {
        start: NaiveDate::from_ymd(2022, 1, 1).and_hms(20, 0, 0),
        end: NaiveDate::from_ymd(2022, 1, 1).and_hms(21, 0, 0),
    };

    assert_eq!(
        Slot {
            start: NaiveDate::from_ymd(2022, 1, 1).and_hms(12, 0, 0),
            end: NaiveDate::from_ymd(2022, 1, 1).and_hms(21, 0, 0)
        },
        slot_a + slot_d + slot_e + slot_f
    );

    let slot_g = Slot {
        start: NaiveDate::from_ymd(2022, 1, 1).and_hms(10, 0, 0),
        end: NaiveDate::from_ymd(2022, 1, 1).and_hms(11, 0, 0),
    };

    assert_eq!(
        Slot {
            start: NaiveDate::from_ymd(2022, 1, 1).and_hms(12, 0, 0),
            end: NaiveDate::from_ymd(2022, 1, 1).and_hms(18, 0, 0)
        },
        slot_a + slot_g
    );

    let slot_h = Slot {
        start: NaiveDate::from_ymd(2022, 1, 1).and_hms(19, 0, 0),
        end: NaiveDate::from_ymd(2022, 1, 1).and_hms(20, 0, 0),
    };

    assert_eq!(
        Slot {
            start: NaiveDate::from_ymd(2022, 1, 1).and_hms(12, 0, 0),
            end: NaiveDate::from_ymd(2022, 1, 1).and_hms(18, 0, 0)
        },
        slot_a + slot_h
    );
}

fn get_test_tasks_2() -> Vec<Task> {
    vec![
        Task {
            id: 1,
            goal_id: 2,
            title: "clean car".to_string(),
            duration: 2,
            status: TaskStatus::UNSCHEDULED,
            flexibility: 0,
            start: NaiveDate::from_ymd(2022, 10, 24).and_hms(0, 0, 0),
            deadline: NaiveDate::from_ymd(2022, 10, 29).and_hms(0, 0, 0),
            after_time: 10,
            before_time: 14,
            slots: Vec::new(),
            confirmed_start: None,
            confirmed_deadline: None,
            internal_marker: NaiveDate::from_ymd(2022, 10, 24).and_hms(10, 0, 0),
        },
        Task {
            id: 2,
            goal_id: 1,
            title: "shopping".to_string(),
            duration: 1,
            status: TaskStatus::UNSCHEDULED,
            flexibility: 0,
            start: NaiveDate::from_ymd(2022, 11, 1).and_hms(0, 0, 0),
            deadline: NaiveDate::from_ymd(2022, 11, 2).and_hms(0, 0, 0),
            after_time: 10,
            before_time: 14,
            slots: Vec::new(),
            confirmed_start: None,
            confirmed_deadline: None,
            internal_marker: NaiveDate::from_ymd(2022, 11, 1).and_hms(10, 0, 0),
        },
        Task {
            id: 3,
            goal_id: 3,
            title: "dentist".to_string(),
            duration: 1,
            status: TaskStatus::UNSCHEDULED,
            flexibility: 0,
            start: NaiveDate::from_ymd(2022, 11, 2).and_hms(0, 0, 0),
            deadline: NaiveDate::from_ymd(2022, 11, 3).and_hms(0, 0, 0),
            after_time: 10,
            before_time: 11,
            slots: Vec::new(),
            confirmed_start: None,
            confirmed_deadline: None,
            internal_marker: NaiveDate::from_ymd(2022, 11, 2).and_hms(10, 0, 0),
        },
    ]
}

fn get_calendar_bounds_2() -> (NaiveDateTime, NaiveDateTime) {
    (
        (NaiveDate::from_ymd(2022, 10, 23).and_hms(0, 0, 0)),
        NaiveDate::from_ymd(2022, 11, 3).and_hms(0, 0, 0),
    )
}

#[test]
fn task_placer_assigns_contiguous_slots() {
    let tasks = get_test_tasks_2();
    let (calendar_start, calendar_end) = get_calendar_bounds_2();
    let scheduled_tasks = task_placer(tasks, calendar_start, calendar_end);
    assert_eq!(scheduled_tasks[0].status, SCHEDULED);
    assert_eq!(scheduled_tasks[1].status, SCHEDULED);
    assert_eq!(scheduled_tasks[2].status, SCHEDULED);
    println!("In test: {:?}", scheduled_tasks[0].slots);
    assert_eq!(scheduled_tasks[0].slots.len(), 5);
    assert_eq!(scheduled_tasks[1].slots.len(), 1);
    assert_eq!(scheduled_tasks[2].slots.len(), 0); //0 because the entire slot was removed when this
                                                   //task was scheduled.

    assert_eq!(
        scheduled_tasks[0].slots[0].start,
        NaiveDate::from_ymd(2022, 10, 24).and_hms(12, 0, 0)
    );
    assert_eq!(
        scheduled_tasks[0].slots[0].end,
        NaiveDate::from_ymd(2022, 10, 24).and_hms(14, 0, 0)
    );

    assert_eq!(
        scheduled_tasks[0].slots[1].start,
        NaiveDate::from_ymd(2022, 10, 25).and_hms(10, 0, 0)
    );
    assert_eq!(
        scheduled_tasks[0].slots[1].end,
        NaiveDate::from_ymd(2022, 10, 25).and_hms(14, 0, 0)
    );

    assert_eq!(
        scheduled_tasks[0].slots[2].start,
        NaiveDate::from_ymd(2022, 10, 26).and_hms(10, 0, 0)
    );
    assert_eq!(
        scheduled_tasks[0].slots[2].end,
        NaiveDate::from_ymd(2022, 10, 26).and_hms(14, 0, 0)
    );

    assert_eq!(
        scheduled_tasks[0].slots[3].start,
        NaiveDate::from_ymd(2022, 10, 27).and_hms(10, 0, 0)
    );
    assert_eq!(
        scheduled_tasks[0].slots[3].end,
        NaiveDate::from_ymd(2022, 10, 27).and_hms(14, 0, 0)
    );

    assert_eq!(
        scheduled_tasks[0].slots[4].start,
        NaiveDate::from_ymd(2022, 10, 28).and_hms(10, 0, 0)
    );
    assert_eq!(
        scheduled_tasks[0].slots[4].end,
        NaiveDate::from_ymd(2022, 10, 28).and_hms(14, 0, 0)
    );

    assert_eq!(
        scheduled_tasks[1].slots[0].start,
        NaiveDate::from_ymd(2022, 11, 1).and_hms(11, 0, 0)
    );
    assert_eq!(
        scheduled_tasks[1].slots[0].end,
        NaiveDate::from_ymd(2022, 11, 1).and_hms(14, 0, 0)
    );
}
