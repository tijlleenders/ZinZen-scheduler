use std::cmp::{max, min};

use crate::models::activity::ActivityStatus::{BestEffort, Postponed, Scheduled, Unprocessed};
use crate::models::activity::ActivityType;
use crate::models::activity::ActivityType::{BudgetMinDay, GetToMinWeekBudget, TopUpWeekBudget};
use crate::models::budget::TimeBudgetType::Week;
use crate::models::calendar_interval::CalIntStatus;
use crate::models::calendar_interval::CalIntStatus::Claimable;
use crate::models::interval::Interval;
use crate::models::{activity::Activity, calendar::Calendar};

#[derive(Debug)]
struct LeastConflict {
    start: usize,
    end: usize,
    claims: usize,
}
pub(crate) fn place(calendar: &mut Calendar, activities: &mut [Activity]) {
    println!("Starting placing...");
    //Todo first check if there are any tasks_done_today
    calendar.register_activities(activities);
    dbg!(&calendar);
    postpone(calendar, activities);

    while let Some(act_index) = find_next_act_index(calendar, activities) {
        println!(
            "Found activity {} to schedule, act_index {}",
            activities[act_index].title, act_index,
        );
        println!("  with flex {}.", activities[act_index].flex());
        let least_conflict: Option<LeastConflict> =
            get_best_index_for(calendar, &activities[act_index]);
        match least_conflict {
            None => {
                println!(
                    "No suitable position found for activity {}...",
                    activities[act_index].title
                );
                activities[act_index].mark_impossible();
                continue;
            }
            Some(least_conflict_position) => {
                dbg!(&least_conflict_position);
                let interval_to_use = Interval {
                    start: least_conflict_position.start,
                    end: least_conflict_position.end,
                };
                calendar.register(&interval_to_use, act_index);
                if calendar.is_participating_in_a_budget(&activities[act_index].goal_id) {
                    calendar.reduce_budgets_for(
                        &activities[act_index].goal_id,
                        least_conflict_position.start,
                        least_conflict_position.end,
                    );
                }
                //Adjust activity internals
                match activities[act_index].activity_type {
                    ActivityType::SimpleGoal => {
                        activities[act_index].duration_left -=
                            least_conflict_position.end - least_conflict_position.start;
                        if activities[act_index].duration_left == 0 {
                            activities[act_index].status = Scheduled; //all at once, not per hour scheduling like before
                            activities[act_index].reset_compatible_intervals();
                        }
                    }
                    BudgetMinDay => {
                        activities[act_index].duration_left -=
                            interval_to_use.end - interval_to_use.start;
                        if activities[act_index].duration_left == 0 {
                            activities[act_index].status = Scheduled;
                            activities[act_index].reset_compatible_intervals();
                        }
                    }
                    GetToMinWeekBudget => {
                        //Only mark it scheduled if budget got to min per week amount
                        activities[act_index].duration_left -=
                            interval_to_use.end - interval_to_use.start;
                        if activities[act_index].duration_left == 0 {
                            activities[act_index].status = Scheduled;
                            activities[act_index].reset_compatible_intervals();
                        }
                    }
                    TopUpWeekBudget => {
                        activities[act_index].duration_left -=
                            interval_to_use.end - interval_to_use.start;
                        for budget in &calendar.budgets {
                            if budget
                                .participating_goals
                                .contains(&activities[act_index].goal_id)
                            {
                                for time_budget in &budget.time_budgets {
                                    if time_budget.time_budget_type == Week
                                        && time_budget.max_scheduled == time_budget.scheduled
                                    {
                                        activities[act_index].status = Scheduled;
                                        activities[act_index].reset_compatible_intervals();
                                    }
                                }
                            }
                        }
                        if activities[act_index].duration_left == 0 {
                            activities[act_index].status = Scheduled;
                            activities[act_index].reset_compatible_intervals();
                        }
                    }
                    ActivityType::SimpleFiller => {}
                }
                //Now we know if the activity has been scheduled - even if it is a budget_min_week
                //This helps us in de decision to let go of other claims inside occupy function
                calendar.occupy(&interval_to_use, act_index, activities);
            }
        }
        println!("Finding next activity to schedule...");
        dbg!(&calendar);
    }
    println!("No more activities to schedule.");
}

fn postpone(calendar: &mut Calendar, activities: &mut [Activity]) {
    for activity in activities.iter_mut() {
        if activity.deadline.is_none()
            && activity.status != BestEffort
            && activity.activity_type != BudgetMinDay
            && activity.activity_type != GetToMinWeekBudget
            && activity.activity_type != TopUpWeekBudget
            && !calendar.is_participating_in_a_budget(&activity.goal_id.clone())
        {
            println!(
                "Skipping activity {} and setting to Postponed since there is no deadline.",
                activity.title
            );
            activity.status = Postponed;
            continue;
        }
    }
}

fn find_next_act_index(calendar: &mut Calendar, activities: &mut [Activity]) -> Option<usize> {
    //check budget validity for all positions of all cal_ints - for all activities
    //since last activity placed might put some positions over the max / day or max/week
    // todo later as possible optimization (measure/reason if useful!): not all activities have to be checked, just the ones that share the same budget(s)
    for activity in activities.iter_mut() {
        calendar.update_compatible_intervals(activity);
    }

    let mut highest_flex = 0;
    let mut act_index_next_to_schedule: Option<usize> = None;

    for (act_index, activity) in activities.iter_mut().enumerate() {
        if !(activity.status == Unprocessed || activity.status == BestEffort)
        //only get Flex for Unprocessed or BestEffort
        {
            continue;
        };
        println!(
            "Getting flex for {}, act_index {}.",
            activity.title, act_index
        );

        let flex = activity.flex();
        match flex {
            0 => {
                //no place possible
                activity.mark_impossible();
            }
            1 => {
                //only one place possible => need to fix_on_calendar
                println!("Flex of 1 found for activity {}", activity.title);
                act_index_next_to_schedule = Some(act_index);
                break;
            }
            _ => {
                if flex > highest_flex {
                    highest_flex = flex;
                    act_index_next_to_schedule = Some(act_index);
                }
            }
        }
    }
    act_index_next_to_schedule
}

fn get_conflicts_for(calendar: &Calendar, start_index: usize, end_index: usize) -> usize {
    let mut conflicts: Option<usize> = None;
    for cal_int in &calendar.intervals {
        if end_index <= cal_int.interval.start {
            //no more possible overlaps
            return conflicts.expect("When calling get conflicts a result is expected");
        }
        if start_index >= cal_int.interval.end {
            //no overlap yet
            continue;
        }
        //overlap
        let overlap_start = max(start_index, cal_int.interval.start);
        let overlap_end = min(end_index, cal_int.interval.end);
        match &cal_int.status {
            Claimable(claims) => match conflicts {
                None => {
                    conflicts = Some((overlap_end - overlap_start) * claims.len());
                }
                Some(number_of_conflicts) => {
                    conflicts =
                        Some(number_of_conflicts + (overlap_end - overlap_start) * claims.len());
                }
            },
            CalIntStatus::Occupied(_act_index, _goal_id) => {}
        }
    }
    conflicts.expect("When calling get conflicts a result is expected")
}
fn get_best_index_for(calendar: &Calendar, activity: &Activity) -> Option<LeastConflict> {
    let mut least_conflict: Option<LeastConflict> = None;

    //the activity intervals can cover multiple calendar intervals as the activity interval doesn't get fragmented
    for interval in &activity.compatible_intervals {
        let interval_len = interval.end - interval.start;
        let max_inner_offset = interval_len - activity.min_block_size;
        #[cfg(debug_assertions)]
        assert!(
            interval_len >= activity.min_block_size,
            "Length of compatible activity interval should be >= min_block_size"
        );
        for inner_offset in 0..=max_inner_offset {
            let new_conflicts = get_conflicts_for(
                calendar,
                interval.start + inner_offset,
                interval.start + inner_offset + activity.min_block_size,
            );
            //Todo Check if budget allows it - if not continue
            match least_conflict {
                None => {
                    least_conflict = Some(LeastConflict {
                        start: interval.start + inner_offset,
                        end: interval.start + inner_offset + activity.min_block_size,
                        claims: new_conflicts,
                    });
                }
                Some(ref mut least_conflict) => {
                    if new_conflicts < least_conflict.claims {
                        least_conflict.start = interval.start + inner_offset;
                        least_conflict.end =
                            interval.start + inner_offset + activity.min_block_size;
                        least_conflict.claims = new_conflicts;
                    }
                    if new_conflicts == 1 {
                        break;
                    }
                }
            }
        }
    }
    least_conflict
}

pub(crate) fn place_postponed_as_best_effort(calendar: &mut Calendar, activities: &mut [Activity]) {
    for activity in activities.iter_mut() {
        if activity.status == Postponed {
            println!(
                "Setting postponed activity {} to BestEffort.",
                activity.title
            );
            activity.status = BestEffort;
        }
    }
    place(calendar, activities);
}
