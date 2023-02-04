use crate::goal::{handle_dependency, handle_hierarchy, Goal, Tag};
use crate::input::Input;
use crate::task::Task;
use crate::Repetition;

/// # Task Generator
/// Takes an [Input](../input/index.html) and outputs a vector of Unscheduled [Tasks](../task/index.html).
pub fn task_generator(
    Input {
        calendar_start,
        calendar_end,
        goals,
    }: Input,
) -> Vec<Task> {
    let mut counter: usize = 0;
    let mut tasks = vec![];
    let goals = handle_hierarchy(goals);
    let goals = handle_dependency(goals);
    for goal in goals {
        if let Some(Repetition::FlexWeekly(min, max)) = goal.repeat {
            //Flex repeat goals are handled as follows:
            //If given a goal with 3-5/week, create two goals: One with 3/week and another
            //with 2/week. The 2/week goal will be tagged optional so won't show up in impossible
            //tasks in case any aren't scheduled.
            let mut goal1 = goal.clone();
            goal1.repeat = Some(Repetition::Weekly(min));
            let mut goal2 = goal.clone();
            goal2.repeat = Some(Repetition::Weekly(max - min));
            goal2.tags.push(Tag::Optional);
            tasks.extend(goal1.generate_tasks(calendar_start, calendar_end, &mut counter));
            tasks.extend(goal2.generate_tasks(calendar_start, calendar_end, &mut counter));
        } else if goal.duration.1.is_some() {
            //if this is a flex duration e.g. '35-40h weekly', create two goals: One with 35/week
            //and another with 5/week. The 5/week goal should be tagged optional. Then turn each
            //of these goals into 1hr goals and generate tasks from each.
            let mut goal1 = goal.clone();
            (goal1.duration.0, goal1.duration.1) = (goal.duration.0, None);
            goal1.tags.push(Tag::FlexDur);
            let mut goal2 = goal.clone();
            (goal2.duration.0, goal2.duration.1) =
                ((goal.duration.1.unwrap() - goal.duration.0), None);
            goal2.tags.push(Tag::Optional);
            goal2.tags.push(Tag::FlexDur);

            //turn into 1hr goals
            let mut goals: Vec<Goal> = vec![];
            let g = get_1_hr_goals(goal1);
            goals.extend(g.into_iter());
            let g = get_1_hr_goals(goal2);
            goals.extend(g.into_iter());
            for goal in goals {
                tasks.extend(goal.generate_tasks(calendar_start, calendar_end, &mut counter));
            }
        } else {
            let goals_vec = goal.generate_tasks(calendar_start, calendar_end, &mut counter);
            tasks.extend(goals_vec);
        }
    }
    tasks
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
