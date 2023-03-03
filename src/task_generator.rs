use crate::goal::{add_filler, Goal, Tag};
use crate::input::{Input, TasksToPlace};
use crate::Repetition;

/// # Task Generator
/// Takes an [Input](../input/index.html) and outputs a vector of TaskStatus::Blocked and TaskStatus::ReadyToSchedule [Tasks](../task/index.html).
pub fn task_generator(
    Input {
        calendar_start,
        calendar_end,
        goals,
    }: Input,
) -> TasksToPlace {
    let mut counter: usize = 0;
    let mut tasks = vec![];
    let mut goals = add_filler(goals);
    add_repeat(&mut goals);

    for goal in goals {
        let goals_vec = goal.generate_tasks(calendar_start, calendar_end, &mut counter);
        tasks.extend(goals_vec);
    }
    TasksToPlace {
        calendar_start,
        calendar_end,
        tasks,
    }
}

fn add_repeat(goals: &mut Vec<Goal>) {
    let mut generated_goals = vec![];
    for goal in goals.iter_mut() {
        if let Some(Repetition::FlexWeekly(min, max)) = goal.repeat {
            //Flex repeat goals are handled as follows:
            //If given a goal with 3-5/week, create two goals: One with 3/week and another
            //with 2/week. The 2/week goal will be tagged optional so won't show up in impossible
            //tasks in case any aren't scheduled.
            let mut goal2 = goal.clone();
            goal.repeat = Some(Repetition::Weekly(min));
            goal2.repeat = Some(Repetition::Weekly(max - min));
            goal2.tags.push(Tag::Optional);
            generated_goals.push(goal2.to_owned());
        } else if goal.duration.1.is_some() {
            //if this is a flex duration e.g. '35-40h weekly', create two goals: One with 35/week
            //and another with 5/week. The 5/week goal should be tagged optional. Then turn each
            //of these goals into 1hr goals and generate tasks from each.
            goal.tags.push(Tag::FlexDur);
            goal.tags.push(Tag::Weekly);
            let mut goal2 = goal.clone();
            (goal.duration.0, goal.duration.1) = (goal.duration.0, None);

            (goal2.duration.0, goal2.duration.1) =
                ((goal2.duration.1.unwrap() - goal2.duration.0), None);
            goal2.tags.push(Tag::Optional);

            //turn into 1hr goals
            // let mut goals: Vec<Goal> = vec![];
            let g = get_1_hr_goals(goal.to_owned());
            generated_goals.extend(g.into_iter());
            let g = get_1_hr_goals(goal2);
            goal.tags.push(Tag::Remove);
            generated_goals.extend(g.into_iter());
        }
    }

    goals.retain(|g| !g.tags.contains(&Tag::Remove));
    goals.extend(generated_goals);
}

fn get_1_hr_goals(goal: Goal) -> Vec<Goal> {
    let mut goals = vec![];
    let dur = goal.duration.0;
    for _ in 0..dur {
        let mut g = goal.clone();
        g.duration.0 = 1;
        goals.push(g);
    }
    goals
}
