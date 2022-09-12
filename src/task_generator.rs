use crate::input::Input;
use crate::task::Task;

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
        tasks.extend(goal.generate_tasks(calendar_start, calendar_end, &mut counter));
    }
    tasks
}
