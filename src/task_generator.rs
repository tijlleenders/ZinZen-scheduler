use crate::goal::Tag;
use crate::input::Input;
use crate::task::Task;
use crate::Repetition;

pub fn task_generator(
    Input {
        calendar_start,
        calendar_end,
        goals,
    }: Input,
) -> Vec<Task> {
    let mut counter: usize = 0;
    let mut tasks = vec![];
    for goal in goals {
        //Flex repeat goals are handled as follows:
        //If given a goal with 3-5/week, create two goals: One with 3/week and another
        //with 2/week. The 2/week goal will be tagged optional so won't show up in impossible
        //tasks in case any aren't scheduled.
        if let Some(Repetition::FlexWeekly(min, max)) = goal.repeat {
            let mut goal1 = goal.clone();
            goal1.repeat = Some(Repetition::WEEKLY(min));
            let mut goal2 = goal.clone();
            goal2.repeat = Some(Repetition::WEEKLY(max - min));
            goal2.tags.push(Tag::OPTIONAL);
            tasks.extend(goal1.generate_tasks(calendar_start, calendar_end, &mut counter));
            tasks.extend(goal2.generate_tasks(calendar_start, calendar_end, &mut counter));
        } else {
            tasks.extend(goal.generate_tasks(calendar_start, calendar_end, &mut counter));
        }
    }
    tasks
}
