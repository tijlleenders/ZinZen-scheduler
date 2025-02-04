use crate::models::calendar::Calendar;
use crate::models::calendar_interval::CalIntStatus;
use crate::models::goal::{Filter, Slot};
use crate::models::interval::Interval;
use chrono::{Datelike, Duration, NaiveDateTime};
use std::ops::Sub;

pub fn reduce(compatible_hours: &[bool]) -> Vec<Interval> {
    let mut result: Vec<Interval> = vec![];
    let mut start: Option<usize> = None;
    for (index, hour) in compatible_hours.iter().enumerate() {
        if *hour {
            if start.is_none() {
                start = Some(index);
            }
        } else if start.is_some() {
            result.push(Interval {
                start: start.unwrap(),
                end: index,
            });
            start = None;
        }
    }
    // Handle the case where the last element is true
    if let Some(start) = start {
        result.push(Interval {
            start,
            end: compatible_hours.len(),
        });
    }
    result
}

fn normalize(intervals: &mut Vec<Interval>) {
    intervals.sort_by_key(|i| i.start);
    let mut result = Vec::new();
    if let Some(first) = intervals.first().cloned() {
        let mut current = first;
        for interval in intervals.iter().skip(1) {
            if interval.start <= current.end {
                current.end = current.end.max(interval.end);
            } else {
                result.push(current);
                current = interval.clone();
            }
        }
        result.push(current);
    }
    *intervals = result;
}

fn add_intervals(mut a: Vec<Interval>, b: Vec<Interval>) -> Vec<Interval> {
    a.extend(b);
    normalize(&mut a);
    a
}

fn subtract_intervals(a: Vec<Interval>, b: &Vec<Interval>) -> Vec<Interval> {
    let mut result = Vec::new();
    for interval in a {
        let mut current = interval;
        for other_interval in b {
            if other_interval.start < current.end && other_interval.end > current.start {
                if other_interval.start > current.start {
                    result.push(Interval {
                        start: current.start,
                        end: other_interval.start,
                    });
                }
                if other_interval.end < current.end {
                    current.start = other_interval.end;
                } else {
                    current.start = current.end;
                    break;
                }
            }
        }
        if current.start < current.end {
            result.push(current);
        }
    }
    result
}

pub(crate) fn get_compatible_intervals(
    calendar: &Calendar,
    filter: &Option<Filter>,
    start: NaiveDateTime,
    end: NaiveDateTime,
    not_on: &Option<Vec<Slot>>,
) -> Vec<Interval> {
    let mut result = vec![Interval {
        start: calendar.get_index_of(start),
        end: calendar.get_index_of(end),
    }];

    if let Some(not_on) = not_on {
        for slot in not_on {
            result = subtract_intervals(
                result,
                &vec![Interval {
                    start: calendar.get_index_of(slot.start),
                    end: calendar.get_index_of(slot.end),
                }],
            );
        }
    }

    //remove anything that is not in the filter
    let mut intervals_to_remove: Vec<Interval> = vec![];
    if let Some(filter) = filter {
        let end = calendar.end_date_time;
        let mut current = calendar.start_date_time.sub(Duration::days(1)).clone();
        let mut current_index_offset: usize = 0;

        while current <= end {
            if filter.on_days.contains(&current.weekday()) {
                if filter.after_time < filter.before_time {
                    intervals_to_remove.push(Interval {
                        start: current_index_offset,
                        end: current_index_offset + filter.after_time,
                    });
                    intervals_to_remove.push(Interval {
                        start: current_index_offset + filter.before_time,
                        end: current_index_offset + 24,
                    })
                } else {
                    intervals_to_remove.push(Interval {
                        start: current_index_offset + filter.before_time,
                        end: current_index_offset + filter.after_time,
                    });
                }
            } else {
                intervals_to_remove.push(Interval {
                    start: current_index_offset,
                    end: current_index_offset + 24,
                })
            }
            current += Duration::days(1);
            current_index_offset += 24;
        }
    }
    result = subtract_intervals(result, &intervals_to_remove);

    //remove anything that is alreeady occupied
    let mut intervals_to_remove2: Vec<Interval> = vec![];
    for cal_interval in &calendar.intervals {
        match cal_interval.status {
            CalIntStatus::Claimable(_) => {}
            CalIntStatus::Occupied(_, _) => intervals_to_remove2.push(Interval {
                start: cal_interval.interval.start,
                end: cal_interval.interval.end,
            }),
        }
    }
    result = subtract_intervals(result, &intervals_to_remove2);

    result
}
