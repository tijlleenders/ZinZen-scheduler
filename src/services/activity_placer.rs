use std::rc::Rc;

use crate::models::{
    activity::{Activity, Status},
    calendar::{Calendar, Hour},
    task::FinalTasks,
};

pub fn place(mut calendar: Calendar, mut activities: Vec<Activity>) -> FinalTasks {
    for _ in 0..activities.len() {
        let act_index_to_schedule = find_act_index_to_schedule(&activities);
        if act_index_to_schedule.is_none() {
            println!("Tried to schedule activity index None");
            continue;
        }
        println!(
            "Next to schedule: {:?}",
            &activities[act_index_to_schedule.unwrap()].title
        );
        activities[act_index_to_schedule.unwrap()].status = Status::Scheduled;
        let best_hour_index: Option<usize> =
            activities[act_index_to_schedule.unwrap()].get_best_scheduling_index();
        println!("Best index:{:?}", &best_hour_index);
        println!(
            "reserving {:?} hours...",
            &activities[act_index_to_schedule.unwrap()].total_duration
        );
        for duration_offset in 0..activities[act_index_to_schedule.unwrap()].total_duration {
            //print statements
            {
                println!(
                    "weak counters:{:?}",
                    Rc::weak_count(&calendar.hours[best_hour_index.unwrap() + duration_offset])
                );
                println!(
                    "stong counters:{:?}\n",
                    Rc::strong_count(&calendar.hours[best_hour_index.unwrap() + duration_offset])
                );
            }
            Rc::make_mut(&mut calendar.hours[best_hour_index.unwrap() + duration_offset]);
            calendar.hours[best_hour_index.unwrap() + duration_offset] = Rc::new(Hour::Occupied {
                activity_index: act_index_to_schedule.unwrap(),
            });
            (activities[act_index_to_schedule.unwrap()]).release_claims();
            //TODO: call activity.release_claims() so it doesn't count for conflicts anymore

            //print statements
            {
                println!(
                    "weak counters:{:?}",
                    Rc::weak_count(&calendar.hours[best_hour_index.unwrap() + duration_offset])
                );
                println!(
                    "stong counters:{:?}\n",
                    Rc::strong_count(&calendar.hours[best_hour_index.unwrap() + duration_offset])
                );
            }
            dbg!(&calendar);
        }
    }
    calendar.get_tasks(activities)
}

fn find_act_index_to_schedule(activities: &Vec<Activity>) -> Option<usize> {
    let mut act_index_to_schedule = None;
    for index in 0..activities.len() {
        if activities[index].status == Status::Scheduled
            || activities[index].status == Status::Impossible
        {
            continue;
        }
        match act_index_to_schedule {
            None => act_index_to_schedule = Some(index),
            Some(_) => match activities[index].flex() {
                0 => {
                    println!("Found activity index {:?} with flex 0...", &index);
                    continue;
                }
                1 => {
                    if activities[act_index_to_schedule.unwrap()].flex() == 1 {
                        break;
                    } else {
                        act_index_to_schedule = Some(index);
                        break;
                    }
                }
                _ => {
                    if activities[act_index_to_schedule.unwrap()].flex() < activities[index].flex()
                    {
                        act_index_to_schedule = Some(index);
                    }
                }
            },
        }
    }
    act_index_to_schedule
}