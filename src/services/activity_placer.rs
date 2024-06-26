use crate::models::{
    activity::{Activity, ActivityType, Status},
    calendar::{Calendar, Hour, ImpossibleActivity},
    task::TaskCompletedToday,
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
                if conflicts > 0
                    && activities[act_index_to_schedule].deadline.is_none()
                    && !calendar.is_budget(activities[act_index_to_schedule].goal_id.clone())
                    && activities[act_index_to_schedule].status != Status::BestEffort
                // Place function should try to place BestEffort Activities
                {
                    println!(
                        "Postponing placement of {:?} - since no deadline and conflicts > 0...\n",
                        activities[act_index_to_schedule].title
                    );
                    activities[act_index_to_schedule].status = Status::Postponed;
                    // Don't release the claims of postponed Activities so other tasks try to stay away from their claims.
                    continue;
                }
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
                    let impossible_activity = ImpossibleActivity {
                        id: activities[act_index_to_schedule].goal_id.clone(),
                        hours_missing: activities[act_index_to_schedule].duration_left,
                        period_start_date_time: activities[act_index_to_schedule].start,
                        period_end_date_time: activities[act_index_to_schedule].deadline,
                    };
                    calendar.impossible_activities.push(impossible_activity);
                    continue;
                }
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
            || activities[index].status == Status::Postponed
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

pub(crate) fn reset_postponed(mut base_activities: Vec<Activity>) -> Vec<Activity> {
    for activity in base_activities.iter_mut() {
        if activity.status == Status::Postponed {
            activity.status = Status::BestEffort;
        }
    }
    base_activities
}

pub(crate) fn place_tasks_completed_today(
    calendar: &mut Calendar,
    mut base_activities: Vec<Activity>,
    tasks_completed_today: &[TaskCompletedToday],
) -> Vec<Activity> {
    dbg!(&calendar);
    for task in tasks_completed_today.iter() {
        let start_index = calendar.get_index_of(task.start);
        let end_index = calendar.get_index_of(task.deadline);

        for hour_index in start_index..end_index {
            let mut act_index_to_schedule: Option<usize> = None;
            for (act_index, activity) in base_activities.iter().enumerate() {
                if activity.goal_id == task.goalid
                //by default, just pick the first
                //but if you find one that has is claiming the index - use that
                {
                    if act_index_to_schedule.is_none() {
                        act_index_to_schedule = Some(act_index);
                    }
                    if activity.calendar_overlay[hour_index].is_some() {
                        //check if hour is claimed and if so - override and break
                        act_index_to_schedule = Some(act_index);
                        break;
                    }
                }
            }

            //hardcode hours in calendar per hour:
            if let Some(act_index_to_schedule) = act_index_to_schedule {
                println!("hardcoding {:?} hours...", end_index - start_index);
                Rc::make_mut(&mut calendar.hours[hour_index]);
                calendar.hours[hour_index] = Rc::new(Hour::Occupied {
                    activity_index: act_index_to_schedule,
                    activity_title: base_activities[act_index_to_schedule].title.clone(),
                    activity_goalid: base_activities[act_index_to_schedule].goal_id.clone(),
                });
                calendar.update_budgets_for(
                    &base_activities[act_index_to_schedule].goal_id.clone(),
                    hour_index,
                );
                if base_activities[act_index_to_schedule].duration_left > 0 {
                    base_activities[act_index_to_schedule].duration_left -= 1;
                };
                if base_activities[act_index_to_schedule].duration_left == 0 {
                    base_activities[act_index_to_schedule].status = Status::Scheduled;
                    (base_activities[act_index_to_schedule]).release_claims();
                }
            }
        }
    }
    dbg!(&calendar);

    base_activities
}
