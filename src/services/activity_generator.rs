use crate::models::{activity::Activity, budget::TimeBudgetType, calendar::Calendar, goal::Goal};

pub fn generate_simple_goal_activities(calendar: &Calendar, goals: &Vec<Goal>) -> Vec<Activity> {
    dbg!(&goals);
    let mut activities: Vec<Activity> = Vec::with_capacity(goals.capacity());
    for goal in goals {
        let parent_goal = goal.get_parent_goal(&goals);

        let mut goal_activities =
            Activity::get_activities_from_simple_goal(goal, calendar, parent_goal);
        dbg!(&goal_activities);
        activities.append(&mut goal_activities);
    }
    activities
}

pub fn generate_budget_goal_activities(calendar: &Calendar, goals: &Vec<Goal>) -> Vec<Activity> {
    dbg!(&goals);
    let mut activities: Vec<Activity> = Vec::with_capacity(goals.capacity());
    for goal in goals {
        let mut goal_activities = Activity::get_activities_from_budget_goal(goal, calendar);
        dbg!(&goal_activities);
        activities.append(&mut goal_activities);
    }
    activities
}

pub fn generate_get_to_week_min_budget_activities(
    calendar: &Calendar,
    goals: &[Goal],
) -> Vec<Activity> {
    let mut get_to_week_min_budget_activities = vec![];
    for budget in &calendar.budgets {
        let mut is_min_week_reached = true;
        for time_budget in &budget.time_budgets {
            if time_budget.time_budget_type == TimeBudgetType::Week { //TODO: Assuming only one week time_budget per budget - need to make multi-week compatilble
                 // Good
            } else {
                continue;
            }
            if time_budget.scheduled < time_budget.min_scheduled {
                is_min_week_reached = false;
            }
        }
        if is_min_week_reached {
            //Fine
            continue;
        } else {
            let goal_to_use: &Goal = goals
                .iter()
                .find(|g| g.id.eq(&budget.originating_goal_id))
                .unwrap();
            for time_budget in &budget.time_budgets {
                if time_budget.time_budget_type == TimeBudgetType::Day
                    && time_budget.scheduled == time_budget.min_scheduled
                    && time_budget.max_scheduled > time_budget.min_scheduled
                {
                    get_to_week_min_budget_activities.extend(
                        Activity::get_activities_to_get_min_week_budget(
                            goal_to_use,
                            calendar,
                            time_budget,
                        ),
                    );
                }
            }
        }
    }
    dbg!(&get_to_week_min_budget_activities);
    get_to_week_min_budget_activities
}

pub fn generate_top_up_week_budget_activities(
    calendar: &Calendar,
    goals: &[Goal],
) -> Vec<Activity> {
    let mut top_up_activities = vec![];
    for budget in &calendar.budgets {
        let goal_to_use: &Goal = goals
            .iter()
            .find(|g| g.id.eq(&budget.originating_goal_id))
            .unwrap();
        for time_budget in &budget.time_budgets {
            if time_budget.time_budget_type == TimeBudgetType::Day
                && time_budget.min_scheduled < time_budget.max_scheduled
                && time_budget.scheduled < time_budget.max_scheduled
            {
                top_up_activities.extend(Activity::get_activities_to_top_up_week_budget(
                    goal_to_use,
                    calendar,
                    time_budget,
                ));
            }
        }
    }
    dbg!(&top_up_activities);
    top_up_activities
}

pub fn adjust_parent_activities(activities: &Vec<Activity>, goals: &Vec<Goal>) -> Vec<Activity> {
    let mut activities_to_return = vec![];
    // >>>

    let mut child_activities: Vec<Activity> = vec![];

    // Get activities for parent and child goals
    let mut parent_activities: Vec<Activity> = goals
        .iter()
        .filter_map(|goal| {
            if let Some(children) = &goal.children {
                let activities_for_parent: Vec<Activity> = activities
                    .iter()
                    .filter(|activity| activity.goal_id == goal.id)
                    .cloned()
                    .collect();
                let child_activities_for_parent: Vec<Activity> = children
                    .iter()
                    .filter_map(|child_id| {
                        activities
                            .iter()
                            .find(|activity| activity.goal_id == *child_id)
                            .cloned()
                    })
                    .collect();
                child_activities.extend(child_activities_for_parent);
                return Some(activities_for_parent);
            }
            None
        })
        .flat_map(|activities| activities)
        .collect();

    if parent_activities.is_empty() {
        return activities.clone();
    }

    // For each parent_activity
    parent_activities.iter_mut().for_each(|parent_activity| {
        let mut parent_duraton = parent_activity.total_duration;

        // For each child_activity
        // get total_duration of child goals
        // deduct it from parent_activity.total_duration
        child_activities.iter().for_each(|child_activity| {
            let child_duration = child_activity.total_duration;
            parent_duraton -= child_duration;
        });
        parent_activity.total_duration = parent_duraton;
        parent_activity.min_block_size = parent_duraton;
        parent_activity.max_block_size = parent_duraton;
        parent_activity.duration_left = parent_duraton;
    });

    // Unify parent_activities and child_activities based on Activity.id into single list which is activities_to_return
    activities_to_return.extend(parent_activities.into_iter());
    activities_to_return.extend(child_activities.into_iter());

    // Sort activities_to_return like the incoming parameter activities
    // activities_to_return.sort_by_key(|activity| activities.iter().position(|&x| x == activity));

    // <<<
    activities_to_return
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::activity::{ActivityType, Status};
    use chrono::NaiveDateTime;

    #[test]
    fn test_adjust_parent_activities() {
        let start_date =
            NaiveDateTime::parse_from_str("2022-01-01T10:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
        let deadline_date =
            NaiveDateTime::parse_from_str("2022-01-01T18:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
        // Sample input data
        let goal1 = Goal {
            id: "1".to_string(),
            title: "Plan a party".to_string(),
            min_duration: Some(4),
            start: start_date,
            deadline: deadline_date,
            children: Some(vec!["2".to_string(), "3".to_string()]),
            budget_config: None,
            filters: None,
        };
        let goal2 = Goal {
            id: "2".to_string(),
            title: "Buy stuff".to_string(),
            min_duration: Some(1),
            start: start_date,
            deadline: deadline_date,
            children: None,
            budget_config: None,
            filters: None,
        };
        let goal3 = Goal {
            id: "3".to_string(),
            title: "Invite friends".to_string(),
            min_duration: Some(1),
            start: start_date,
            deadline: deadline_date,
            children: None,
            budget_config: None,
            filters: None,
        };
        let goals = vec![goal1.clone(), goal2.clone(), goal3.clone()];

        let activity1 = Activity {
            goal_id: "1".to_string(),
            activity_type: ActivityType::SimpleGoal,
            title: "Plan a party".to_string(),
            min_block_size: 4,
            max_block_size: 4,
            calendar_overlay: vec![],
            time_budgets: vec![],
            total_duration: 4,
            duration_left: 4,
            status: Status::Unprocessed,
        };
        let activity2 = Activity {
            goal_id: "2".to_string(),
            activity_type: ActivityType::SimpleGoal,
            title: "Buy stuff".to_string(),
            min_block_size: 1,
            max_block_size: 1,
            calendar_overlay: vec![],
            time_budgets: vec![],
            total_duration: 1,
            duration_left: 1,
            status: Status::Unprocessed,
        };
        let activity3 = Activity {
            goal_id: "3".to_string(),
            activity_type: ActivityType::SimpleGoal,
            title: "Invite friends".to_string(),
            min_block_size: 1,
            max_block_size: 1,
            calendar_overlay: vec![],
            time_budgets: vec![],
            total_duration: 1,
            duration_left: 1,
            status: Status::Unprocessed,
        };
        let expected_activities = vec![activity1.clone(), activity2.clone(), activity3.clone()];

        // Call the function
        let adjusted_activities = adjust_parent_activities(&expected_activities, &goals);

        // Make sure sort of activities are the same as expected
        assert!(adjusted_activities.len() == 3);
        assert_eq!(
            adjusted_activities[0].goal_id,
            expected_activities[0].goal_id
        );
        assert_eq!(
            adjusted_activities[1].goal_id,
            expected_activities[1].goal_id
        );
        assert_eq!(
            adjusted_activities[2].goal_id,
            expected_activities[2].goal_id
        );

        // Make sure adjusted parent activities are as expected
        assert_eq!(adjusted_activities[0].total_duration, 2);
        assert_eq!(adjusted_activities[0].max_block_size, 2);
        assert_eq!(adjusted_activities[0].min_block_size, 2);
        assert_eq!(adjusted_activities[0].duration_left, 2);
    }
}
