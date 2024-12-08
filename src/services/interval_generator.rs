use crate::models::interval::Interval;

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
