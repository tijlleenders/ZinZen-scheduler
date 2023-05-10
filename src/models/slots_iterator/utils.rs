use chrono::{Datelike, Days, Duration, NaiveDateTime, Timelike, Weekday};

use crate::models::repetition::Repetition;

pub fn get_start_of_repeat_step(
    current_date_time: &NaiveDateTime,
    repeat: Repetition,
) -> NaiveDateTime {
    //TODO 2023-04-19 This function doesn't consider merged slots. It should be fixed to consider merged slots.

    let mut result = *current_date_time;
    // dbg!(&result);
    match repeat {
        Repetition::DAILY(_) => result
            .checked_add_days(Days::new(1))
            .unwrap()
            .with_hour(0)
            .unwrap()
            .with_minute(0)
            .unwrap()
            .with_second(0)
            .unwrap(),

        Repetition::HOURLY => result.checked_add_signed(Duration::hours(1)).unwrap(),
        Repetition::Weekly(_) => next_week(&mut result),
        Repetition::WEEKDAYS => {
            match result.weekday() {
                Weekday::Sat => result
                    .checked_add_days(Days::new(2))
                    .unwrap()
                    .with_hour(0)
                    .unwrap()
                    .with_minute(0)
                    .unwrap()
                    .with_second(0)
                    .unwrap(),
                Weekday::Sun => result
                    .checked_add_days(Days::new(1))
                    .unwrap()
                    .with_hour(0)
                    .unwrap()
                    .with_minute(0)
                    .unwrap()
                    .with_second(0)
                    .unwrap(),
                _ => result
                    .checked_add_days(Days::new(1))
                    .unwrap()
                    .with_hour(0)
                    .unwrap()
                    .with_minute(0)
                    .unwrap()
                    .with_second(0)
                    .unwrap(),
            }

            // if result.weekday() == Weekday::Sat {
            //     return result
            //         .checked_add_days(Days::new(2))
            //         .unwrap()
            //         .with_hour(0)
            //         .unwrap()
            //         .with_minute(0)
            //         .unwrap()
            //         .with_second(0)
            //         .unwrap();
            // } else if result.weekday() == Weekday::Sun {
            //     return result
            //         .checked_add_days(Days::new(1))
            //         .unwrap()
            //         .with_hour(0)
            //         .unwrap()
            //         .with_minute(0)
            //         .unwrap()
            //         .with_second(0)
            //         .unwrap();
            // } else {
            //     return result
            //         .checked_add_days(Days::new(1))
            //         .unwrap()
            //         .with_hour(0)
            //         .unwrap()
            //         .with_minute(0)
            //         .unwrap()
            //         .with_second(0)
            //         .unwrap();
            // }
        }
        Repetition::WEEKENDS => {
            if result.weekday() == Weekday::Sat {
                return result
                    .checked_add_days(Days::new(7))
                    .unwrap()
                    .with_hour(0)
                    .unwrap()
                    .with_minute(0)
                    .unwrap()
                    .with_second(0)
                    .unwrap();
            }
            if result.weekday() == Weekday::Sun {
                return result
                    .checked_add_days(Days::new(6))
                    .unwrap()
                    .with_hour(0)
                    .unwrap()
                    .with_minute(0)
                    .unwrap()
                    .with_second(0)
                    .unwrap();
            }
            for _days_to_add in 1..=6 {
                result = result.checked_add_days(Days::new(1)).unwrap();
                if result.weekday() == Weekday::Sat {
                    return result
                        .with_hour(0)
                        .unwrap()
                        .with_minute(0)
                        .unwrap()
                        .with_second(0)
                        .unwrap();
                }
            }
            panic!("Shouldn't reach this");
        }
        Repetition::EveryXdays(day_interval) => result
            .checked_add_days(Days::new(day_interval.try_into().unwrap()))
            .unwrap()
            .with_hour(0)
            .unwrap()
            .with_minute(0)
            .unwrap()
            .with_second(0)
            .unwrap(),
        Repetition::EveryXhours(hour_interval) => result
            .checked_add_signed(Duration::hours(hour_interval.try_into().unwrap()))
            .unwrap(),
        Repetition::MONDAYS => {
            if result.weekday() == Weekday::Mon {
                return result
                    .checked_add_days(Days::new(7))
                    .unwrap()
                    .with_hour(0)
                    .unwrap()
                    .with_minute(0)
                    .unwrap()
                    .with_second(0)
                    .unwrap();
            }
            for _days_to_add in 1..=6 {
                result = result.checked_add_days(Days::new(1)).unwrap();
                if result.weekday() == Weekday::Mon {
                    return result
                        .with_hour(0)
                        .unwrap()
                        .with_minute(0)
                        .unwrap()
                        .with_second(0)
                        .unwrap();
                }
            }
            panic!("Shouldn't reach this");
        }
        Repetition::TUESDAYS => {
            if result.weekday() == Weekday::Tue {
                return result
                    .checked_add_days(Days::new(7))
                    .unwrap()
                    .with_hour(0)
                    .unwrap()
                    .with_minute(0)
                    .unwrap()
                    .with_second(0)
                    .unwrap();
            }
            for _days_to_add in 1..=6 {
                result = result.checked_add_days(Days::new(1)).unwrap();
                if result.weekday() == Weekday::Tue {
                    return result
                        .with_hour(0)
                        .unwrap()
                        .with_minute(0)
                        .unwrap()
                        .with_second(0)
                        .unwrap();
                }
            }
            panic!("Shouldn't reach this");
        }
        Repetition::WEDNESDAYS => {
            if result.weekday() == Weekday::Wed {
                return result
                    .checked_add_days(Days::new(7))
                    .unwrap()
                    .with_hour(0)
                    .unwrap()
                    .with_minute(0)
                    .unwrap()
                    .with_second(0)
                    .unwrap();
            }
            for _days_to_add in 1..=6 {
                result = result.checked_add_days(Days::new(1)).unwrap();
                if result.weekday() == Weekday::Wed {
                    return result
                        .with_hour(0)
                        .unwrap()
                        .with_minute(0)
                        .unwrap()
                        .with_second(0)
                        .unwrap();
                }
            }
            panic!("Shouldn't reach this");
        }
        Repetition::THURSDAYS => {
            if result.weekday() == Weekday::Thu {
                return result
                    .checked_add_days(Days::new(7))
                    .unwrap()
                    .with_hour(0)
                    .unwrap()
                    .with_minute(0)
                    .unwrap()
                    .with_second(0)
                    .unwrap();
            }
            for _days_to_add in 1..=6 {
                result = result.checked_add_days(Days::new(1)).unwrap();
                if result.weekday() == Weekday::Thu {
                    return result
                        .with_hour(0)
                        .unwrap()
                        .with_minute(0)
                        .unwrap()
                        .with_second(0)
                        .unwrap();
                }
            }
            panic!("Shouldn't reach this");
        }
        Repetition::FRIDAYS => {
            if result.weekday() == Weekday::Fri {
                return result
                    .checked_add_days(Days::new(7))
                    .unwrap()
                    .with_hour(0)
                    .unwrap()
                    .with_minute(0)
                    .unwrap()
                    .with_second(0)
                    .unwrap();
            }
            for _days_to_add in 1..=6 {
                result = result.checked_add_days(Days::new(1)).unwrap();
                if result.weekday() == Weekday::Fri {
                    return result
                        .with_hour(0)
                        .unwrap()
                        .with_minute(0)
                        .unwrap()
                        .with_second(0)
                        .unwrap();
                }
            }
            panic!("Shouldn't reach this");
        }
        Repetition::SATURDAYS => {
            if result.weekday() == Weekday::Sat {
                return result
                    .checked_add_days(Days::new(7))
                    .unwrap()
                    .with_hour(0)
                    .unwrap()
                    .with_minute(0)
                    .unwrap()
                    .with_second(0)
                    .unwrap();
            }
            for _days_to_add in 1..=6 {
                result = result.checked_add_days(Days::new(1)).unwrap();
                if result.weekday() == Weekday::Sat {
                    return result
                        .with_hour(0)
                        .unwrap()
                        .with_minute(0)
                        .unwrap()
                        .with_second(0)
                        .unwrap();
                }
            }
            panic!("Shouldn't reach this");
        }
        Repetition::SUNDAYS => {
            if result.weekday() == Weekday::Sun {
                return result
                    .checked_add_days(Days::new(7))
                    .unwrap()
                    .with_hour(0)
                    .unwrap()
                    .with_minute(0)
                    .unwrap()
                    .with_second(0)
                    .unwrap();
            }
            for _days_to_add in 1..=6 {
                result = result.checked_add_days(Days::new(1)).unwrap();
                if result.weekday() == Weekday::Sun {
                    return result
                        .with_hour(0)
                        .unwrap()
                        .with_minute(0)
                        .unwrap()
                        .with_second(0)
                        .unwrap();
                }
            }
            panic!("Shouldn't reach this");
        }
        Repetition::FlexDaily(_, _) => todo!(),
        Repetition::FlexWeekly(_, _) => todo!(),
    }
    // dbg!(&result);
}

/// Get next week for a current datetime
pub(crate) fn next_week(result: &mut NaiveDateTime) -> NaiveDateTime {
    return result
        .checked_add_days(Days::new(7))
        .unwrap()
        .with_hour(0)
        .unwrap()
        .with_minute(0)
        .unwrap()
        .with_second(0)
        .unwrap();
}
