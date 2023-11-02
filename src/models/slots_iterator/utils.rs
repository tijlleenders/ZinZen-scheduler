use crate::models::repetition::Repetition;
use chrono::{Datelike, Days, Duration, NaiveDateTime, Timelike, Weekday};

pub fn get_start_of_repeat_step(
    current_date_time: &NaiveDateTime,
    repeat: Repetition,
) -> NaiveDateTime {
    //TODO 2023-04-19 This function doesn't consider merged slots. It should be fixed to consider merged slots.

    let mut result = *current_date_time;
    match repeat {
        Repetition::DAILY(_) => result.checked_add_days(Days::new(1)).unwrap(),
        Repetition::HOURLY => result.checked_add_signed(Duration::hours(1)).unwrap(),
        Repetition::WEEKLY(_) => next_week(&result),
        Repetition::WEEKDAYS => match result.weekday() {
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
        },
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
        Repetition::EVERY_X_DAYS(day_interval) => result
            .checked_add_days(Days::new(day_interval.try_into().unwrap()))
            .unwrap()
            .with_hour(0)
            .unwrap()
            .with_minute(0)
            .unwrap()
            .with_second(0)
            .unwrap(),
        Repetition::EVERY_X_HOURS(hour_interval) => result
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
        Repetition::FLEX_DAILY(_, _) => todo!(),
        Repetition::FLEX_WEEKLY(_, _) => todo!(),
    }
}

/// Get next week for a current datetime
pub(crate) fn next_week(result: &NaiveDateTime) -> NaiveDateTime {
    result
        .checked_add_days(Days::new(7))
        .unwrap()
        .with_hour(0)
        .unwrap()
        .with_minute(0)
        .unwrap()
        .with_second(0)
        .unwrap()
}
