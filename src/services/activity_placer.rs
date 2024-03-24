use crate::models::{
    activity::{Activity, ActivityType, Status},
    calendar::{Calendar, Hour, ImpossibleActivity},
};
use std::{cmp, rc::Rc};

pub fn place(calendar: &mut Calendar, mut activities: Vec<Activity>) -> Vec<Activity> {
    loop {
        for activity in activities.iter_mut() {
            activity.update_overlay_with(&calendar.budgets);
        }
        if let Some(act_index_to_schedule) = find_act_index_to_schedule(&activities) {
            println!(
                "Next to schedule: {:?}, id {:?}\n",
                &activities[act_index_to_schedule].title,
                &activities[act_index_to_schedule].goal_id
                    [0..cmp::min(activities[act_index_to_schedule].goal_id.len(), 5)]
            );
            if let Some((best_hour_index, best_size, conflicts)) =
                activities[act_index_to_schedule].get_best_scheduling_index_length_conflicts()
            {
                println!(
                    "Best index:{:?}, size {:?}, best conflicts {:?}",
                    &best_hour_index, &best_size, &conflicts
                );
                println!("reserving {:?} hours...", best_size);
                for duration_offset in 0..best_size {
                    Rc::make_mut(&mut calendar.hours[best_hour_index + duration_offset]);
                    calendar.hours[best_hour_index + duration_offset] = Rc::new(Hour::Occupied {
                        activity_index: act_index_to_schedule,
                        activity_title: activities[act_index_to_schedule].title.clone(),
                        activity_goalid: activities[act_index_to_schedule].goal_id.clone(),
                    });
                    calendar.update_budgets_for(
                        &activities[act_index_to_schedule].goal_id.clone(),
                        best_hour_index + duration_offset,
                    );
                    activities[act_index_to_schedule].duration_left -= 1;
                }
                if activities[act_index_to_schedule].duration_left == 0 {
                    activities[act_index_to_schedule].status = Status::Scheduled;
                    (activities[act_index_to_schedule]).release_claims();
                }

                dbg!(&calendar);
            } else {
                activities[act_index_to_schedule].release_claims();
                if activities[act_index_to_schedule].activity_type == ActivityType::BudgetMinDay {
                    activities[act_index_to_schedule].status = Status::Processed;
                    continue;
                } else {
                    activities[act_index_to_schedule].status = Status::Impossible;
                }
                let impossible_activity = ImpossibleActivity {
                    id: activities[act_index_to_schedule].goal_id.clone(),
                    hours_missing: activities[act_index_to_schedule].duration_left,
                    period_start_date_time: activities[act_index_to_schedule].start,
                    period_end_date_time: activities[act_index_to_schedule].deadline,
                };
                calendar.impossible_activities.push(impossible_activity);
                continue;
            }
        } else {
            println!("Tried to schedule activity index None");
            break;
        }
    }
    dbg!(&calendar);
    activities
}

fn find_act_index_to_schedule(activities: &[Activity]) -> Option<usize> {
    let mut act_index_to_schedule = None;
    for index in 0..activities.len() {
        if activities[index].status == Status::Scheduled
            || activities[index].status == Status::Impossible
            || activities[index].status == Status::Processed
        {
            continue;
        }
        let current_act_flex = activities[index].flex();
        println!(
            "Flex {:?} for {:?}",
            current_act_flex, activities[index].title
        );
        match act_index_to_schedule {
            None => act_index_to_schedule = Some(index),
            Some(_) => match current_act_flex {
                0 => {
                    println!("Found activity index {:?} with flex 0...", &index);
                    continue;
                }
                1 => {
                    if activities[act_index_to_schedule?].flex() == 1 {
                        break;
                    } else {
                        act_index_to_schedule = Some(index);
                        break;
                    }
                }
                _ => {
                    if activities[act_index_to_schedule?].flex() < current_act_flex {
                        act_index_to_schedule = Some(index);
                    }
                }
            },
        }
    }
    act_index_to_schedule
}
