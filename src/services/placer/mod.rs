mod find_best_slots;

//For a visual step-by-step breakdown of the scheduler algorithm see https://docs.google.com/presentation/d/1Tj0Bg6v_NVkS8mpa-aRtbDQXM-WFkb3MloWuouhTnAM/edit?usp=sharing
use crate::models::goal::{Goal, Tag};
use crate::models::input::{PlacedTasks, TasksToPlace};
use crate::models::slot::Slot;
use crate::models::task::{NewTask, Task, TaskStatus};
use crate::models::timeline::Timeline;

/// The Task Placer receives a list of tasks from the Task Generator and attempts to assign each
/// task a confirmed start and deadline.
/// The scheduler optimizes for the minimum amount of Impossible tasks.
pub fn task_placer(mut tasks_to_place: TasksToPlace) -> PlacedTasks {
    dbg!(&tasks_to_place);
    //first pass of scheduler while tasks are unsplit
    schedule(&mut tasks_to_place);
    dbg!(&tasks_to_place);

    // TODO 2023-04-29: Fix function adjust_min_budget_tasks based on debug feedback check:
    // https://github.com/tijlleenders/ZinZen-scheduler/issues/300#issuecomment-1528727445

    adjust_min_budget_tasks(&mut tasks_to_place); //TODO
    dbg!(&tasks_to_place);

    schedule(&mut tasks_to_place); //TODO
    dbg!(&tasks_to_place);

    PlacedTasks {
        calendar_start: tasks_to_place.calendar_start,
        calendar_end: tasks_to_place.calendar_end,
        tasks: tasks_to_place.tasks,
    }
}

fn adjust_min_budget_tasks(tasks_to_place: &mut TasksToPlace) {
    let mut tasks_to_add: Vec<Task> = Vec::new();
    dbg!(&tasks_to_place);

    for index in 0..tasks_to_place.tasks.len() {
        if tasks_to_place.tasks[index].status == TaskStatus::BudgetMinWaitingForAdjustment {
            for slot_budget in &tasks_to_place
                .task_budgets
                .budget_id_to_budget
                .get(&tasks_to_place.tasks[index].goal_id)
                .unwrap()
                .slot_budgets
            {
                //TODO If any remaining hours in slot_budget:
                // Loop through BudgetTaskMinWaitingForAdjustment Task Vec<Slot> and chop off anything that is outside of the slot_budget Slot
                // Make Task with those slots and remaining hours
                // If not enough hours - mark impossible? No will happen during scheduling.
                // TODO 2023-06-20: idea to refactor below code into a separate function
                let mut task_slots_to_adjust = tasks_to_place.tasks[index].slots.clone();
                for slot in task_slots_to_adjust.iter_mut() {
                    if slot.start.lt(&slot_budget.slot.start) {
                        slot.start = slot_budget.slot.start;
                    }
                    if slot.end.lt(&slot_budget.slot.start) {
                        slot.end = slot_budget.slot.start;
                    }
                    if slot.end.gt(&slot_budget.slot.end) {
                        slot.end = slot_budget.slot.end;
                    }
                    if slot.start.gt(&slot_budget.slot.end) {
                        slot.start = slot_budget.slot.end;
                    }
                }
                let mut result_slots: Vec<Slot> = Vec::new();
                for task_slot in task_slots_to_adjust {
                    if task_slot.start.ne(&task_slot.end) {
                        result_slots.push(task_slot);
                    }
                }

                // TODO 2023-06-04  | fix this by using retain instead of this way
                tasks_to_place.tasks[index].tags.push(Tag::Remove);

                let new_title = tasks_to_place.tasks[index].title.clone();
                if tasks_to_place.tasks[index].duration >= slot_budget.used {
                    let new_duration = tasks_to_place.tasks[index].duration - slot_budget.used;
                    let task_id = tasks_to_place.tasks[index].id;
                    let goal = Goal {
                        id: tasks_to_place.tasks[index].goal_id.clone(),
                        title: new_title.clone(),
                        tags: tasks_to_place.tasks[index].tags.clone(),
                        after_goals: tasks_to_place.tasks[index].after_goals.clone(),
                        ..Default::default()
                    };
                    let timeline = Timeline {
                        slots: result_slots.into_iter().collect(),
                    };

                    let new_task = NewTask {
                        task_id,
                        title: new_title,
                        duration: new_duration,
                        goal,
                        timeline,
                        status: TaskStatus::ReadyToSchedule,
                        timeframe: None,
                    };

                    let task_to_add = Task::new(new_task);

                    tasks_to_add.push(task_to_add);
                }
            }
        }
    }
    tasks_to_place
        .tasks
        .retain(|x| !x.tags.contains(&Tag::Remove));
    tasks_to_place.tasks.extend(tasks_to_add);
}

fn schedule(tasks_to_place: &mut TasksToPlace) {
    loop {
        tasks_to_place.sort_on_flexibility();
        dbg!(&tasks_to_place);
        let first_task = tasks_to_place.tasks[0].clone();
        dbg!(&first_task);
        if first_task.status != TaskStatus::ReadyToSchedule {
            break;
        }
        match find_best_slots::find_best_slots(&tasks_to_place.tasks) {
            Some(chosen_slots) => {
                dbg!(&chosen_slots);
                do_the_scheduling(tasks_to_place, chosen_slots);
                dbg!(&tasks_to_place);
            }
            None => break,
        }
    }
}

fn do_the_scheduling(tasks_to_place: &mut TasksToPlace, chosen_slots: Vec<Slot>) {
    dbg!(&tasks_to_place, &chosen_slots);
    /*
    TODO 2023-06-06 | Debug notes
    - for code `template_task.duration`, expected causing inaccurate duration for tasks
    - think to initialize `template_task.duration` to remaining_hours
    - create a function to initialize scheduled task to minimize effort and clean code
    - for code `template_task.id`, make it realistic numbering. Idea to create function inside Task to generate a new number which not duplicated with current list of tasks
    - Todo 2023-06-04  | for code `template_task.id += 1;`, have issue which multiple tasks with the same id
    - for code `for slot in chosen_slots.iter()`, just make it function and call it

    Todo 2023-06-11: Need to refactor this function to be testable
    */

    let mut remaining_hours = tasks_to_place.tasks[0].duration;
    let mut template_task = tasks_to_place.tasks[0].clone();
    template_task.status = TaskStatus::Scheduled;
    // template_task.duration = 1;
    template_task.id = tasks_to_place.tasks.len();
    template_task.slots.clear();

    for slot in chosen_slots.iter() {
        dbg!(&slot);
        if remaining_hours <= 0 {
            break;
        }

        if !tasks_to_place
            .task_budgets
            .is_allowed_by_budget(slot, &template_task.goal_id)
        {
            continue;
        }
        remaining_hours -= slot.duration_as_hours();
        template_task.id += 1;
        template_task.start = Some(slot.start);
        template_task.deadline = Some(slot.end);
        tasks_to_place.tasks.push(template_task.clone());
    }

    let chosen_slot = chosen_slots[0];
    for task in tasks_to_place.tasks.iter_mut() {
        task.remove_conflicted_slots(chosen_slot.to_owned());
    }

    //Todo remove chosen_slots from TaskBudgets
    if remaining_hours > 0 {
        tasks_to_place.tasks[0].duration = remaining_hours;
        tasks_to_place.tasks[0].status = TaskStatus::Impossible;
    } else {
        tasks_to_place.tasks.remove(0);

        // TODO 2023-06-06  | apply function Task::remove_from_blocked_by when it is developed
        let _task_scheduled_goal_id = tasks_to_place.tasks[0].goal_id.clone();
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::models::budget::TaskBudgets;

    use super::*;
    use chrono::Duration;

    /// Simulating test case bug_215 when coming to the function `task_placer`
    #[test]
    #[ignore]
    fn test_task_placer_to_simulate_bug_215() {
        /*
        TODO 2023-06-05  | Debug notes
        flexiblity calculation is not accurate as below:
        - For task: "water the plants indoors", correct flexiblity is 14 but it is calculated as 34.
            - FIXME 2023-06-06 | For task: "water the plants indoors", it added slots out of budget. It is noticed inside funciton `schedule`, after function `tasks_to_place.sort_on_flexibility()` and before calling function `find_best_slots`
        - For task: "sleep", correct flexibility is 19 but it is calculated as 22.

        # 2023-06-08
        - Tasks after function `task_placer` have inaccurate fields "id" and "goal_id"
        */

        let calendar_timing = Slot::mock(Duration::days(7), 2023, 01, 03, 0, 0);

        let tasks: Vec<Task> = vec![
            Task::mock(
                "water the plants indoors",
                1,
                0,
                TaskStatus::ReadyToSchedule,
                vec![
                    Slot::mock(chrono::Duration::hours(2), 2023, 1, 3, 1, 0),
                    Slot::mock(chrono::Duration::hours(2), 2023, 1, 4, 1, 0),
                    Slot::mock(chrono::Duration::hours(2), 2023, 1, 5, 1, 0),
                    Slot::mock(chrono::Duration::hours(2), 2023, 1, 6, 1, 0),
                    Slot::mock(chrono::Duration::hours(2), 2023, 1, 7, 1, 0),
                    Slot::mock(chrono::Duration::hours(2), 2023, 1, 8, 1, 0),
                    Slot::mock(chrono::Duration::hours(2), 2023, 1, 9, 1, 0),
                ],
                None,
            ),
            Task::mock(
                "dinner",
                1,
                0,
                TaskStatus::ReadyToSchedule,
                vec![
                    Slot::mock(chrono::Duration::hours(3), 2023, 1, 3, 18, 0),
                    Slot::mock(chrono::Duration::hours(3), 2023, 1, 4, 18, 0),
                    Slot::mock(chrono::Duration::hours(3), 2023, 1, 5, 18, 0),
                    Slot::mock(chrono::Duration::hours(3), 2023, 1, 6, 18, 0),
                    Slot::mock(chrono::Duration::hours(3), 2023, 1, 7, 18, 0),
                    Slot::mock(chrono::Duration::hours(3), 2023, 1, 8, 18, 0),
                    Slot::mock(chrono::Duration::hours(3), 2023, 1, 9, 18, 0),
                ],
                None,
            ),
            Task::mock(
                "walk",
                1,
                0,
                TaskStatus::ReadyToSchedule,
                vec![
                    Slot::mock(chrono::Duration::hours(6), 2023, 1, 3, 14, 0),
                    Slot::mock(chrono::Duration::hours(6), 2023, 1, 4, 14, 0),
                    Slot::mock(chrono::Duration::hours(6), 2023, 1, 5, 14, 0),
                    Slot::mock(chrono::Duration::hours(6), 2023, 1, 6, 14, 0),
                    Slot::mock(chrono::Duration::hours(6), 2023, 1, 7, 14, 0),
                    Slot::mock(chrono::Duration::hours(6), 2023, 1, 8, 14, 0),
                    Slot::mock(chrono::Duration::hours(6), 2023, 1, 9, 14, 0),
                ],
                None,
            ),
            Task::mock(
                "breakfast",
                1,
                0,
                TaskStatus::ReadyToSchedule,
                vec![
                    Slot::mock(chrono::Duration::hours(3), 2023, 1, 3, 06, 0),
                    Slot::mock(chrono::Duration::hours(3), 2023, 1, 4, 06, 0),
                    Slot::mock(chrono::Duration::hours(3), 2023, 1, 5, 06, 0),
                    Slot::mock(chrono::Duration::hours(3), 2023, 1, 6, 06, 0),
                    Slot::mock(chrono::Duration::hours(3), 2023, 1, 7, 06, 0),
                    Slot::mock(chrono::Duration::hours(3), 2023, 1, 8, 06, 0),
                    Slot::mock(chrono::Duration::hours(3), 2023, 1, 9, 06, 0),
                ],
                None,
            ),
            Task::mock(
                "me time",
                1,
                0,
                TaskStatus::ReadyToSchedule,
                vec![Slot::mock(chrono::Duration::days(7), 2023, 1, 3, 0, 0)],
                None,
            ),
            Task::mock(
                "lunch",
                1,
                0,
                TaskStatus::ReadyToSchedule,
                vec![
                    Slot::mock(chrono::Duration::hours(2), 2023, 1, 3, 12, 0),
                    Slot::mock(chrono::Duration::hours(2), 2023, 1, 4, 12, 0),
                    Slot::mock(chrono::Duration::hours(2), 2023, 1, 5, 12, 0),
                    Slot::mock(chrono::Duration::hours(2), 2023, 1, 6, 12, 0),
                    Slot::mock(chrono::Duration::hours(2), 2023, 1, 7, 12, 0),
                    Slot::mock(chrono::Duration::hours(2), 2023, 1, 8, 12, 0),
                    Slot::mock(chrono::Duration::hours(2), 2023, 1, 9, 12, 0),
                ],
                None,
            ),
            Task::mock(
                "hurdle",
                2,
                0,
                TaskStatus::ReadyToSchedule,
                vec![
                    Slot::mock(chrono::Duration::hours(2), 2023, 1, 3, 1, 0),
                    Slot::mock(chrono::Duration::hours(2), 2023, 1, 4, 1, 0),
                    Slot::mock(chrono::Duration::hours(2), 2023, 1, 5, 1, 0),
                    Slot::mock(chrono::Duration::hours(2), 2023, 1, 6, 1, 0),
                    Slot::mock(chrono::Duration::hours(2), 2023, 1, 7, 1, 0),
                    Slot::mock(chrono::Duration::hours(2), 2023, 1, 8, 1, 0),
                    Slot::mock(chrono::Duration::hours(2), 2023, 1, 9, 1, 0),
                ],
                None,
            ),
            Task::mock(
                "sleep",
                8,
                0,
                TaskStatus::ReadyToSchedule,
                vec![
                    Slot::mock(Duration::hours(8), 2023, 01, 03, 0, 0),
                    Slot::mock(Duration::hours(10), 2023, 01, 03, 22, 0),
                    Slot::mock(Duration::hours(10), 2023, 01, 04, 22, 0),
                    Slot::mock(Duration::hours(10), 2023, 01, 05, 22, 0),
                    Slot::mock(Duration::hours(10), 2023, 01, 06, 22, 0),
                    Slot::mock(Duration::hours(10), 2023, 01, 07, 22, 0),
                    Slot::mock(Duration::hours(10), 2023, 01, 08, 22, 0),
                    Slot::mock(Duration::hours(2), 2023, 01, 09, 22, 0),
                ],
                None,
            ),
        ];

        let task_budgets = TaskBudgets {
            calendar_start: calendar_timing.start,
            calendar_end: calendar_timing.end,
            goal_id_to_budget_ids: HashMap::new(),
            budget_id_to_budget: HashMap::new(),
        };
        dbg!(&task_budgets);

        let tasks_to_place = TasksToPlace {
            calendar_start: calendar_timing.start,
            calendar_end: calendar_timing.end,
            tasks,
            task_budgets,
        };
        dbg!(&tasks_to_place);

        let expected_tasks: Vec<Task> = vec![
            Task::mock_scheduled(
                9,
                "1",
                "me time",
                1,
                168,
                Slot::mock(chrono::Duration::hours(1), 2023, 1, 3, 09, 0),
            ),
            Task::mock_scheduled(
                9,
                "1",
                "walk",
                1,
                42,
                Slot::mock(chrono::Duration::hours(1), 2023, 1, 3, 14, 0),
            ),
            Task::mock_scheduled(
                9,
                "1",
                "dinner",
                1,
                21,
                Slot::mock(chrono::Duration::hours(1), 2023, 1, 3, 18, 0),
            ),
            Task::mock_scheduled(
                9,
                "1",
                "breakfast",
                1,
                21,
                Slot::mock(chrono::Duration::hours(1), 2023, 1, 3, 08, 0),
            ),
            Task::mock_scheduled(
                9,
                "1",
                "sleep",
                8,
                19,
                Slot::mock(Duration::hours(8), 2023, 01, 03, 0, 0),
            ),
            Task::mock_scheduled(
                9,
                "1",
                "water the plants indoors",
                1,
                14,
                Slot::mock(chrono::Duration::hours(1), 2023, 1, 4, 1, 0),
            ),
            Task::mock_scheduled(
                9,
                "1",
                "lunch",
                1,
                14,
                Slot::mock(chrono::Duration::hours(1), 2023, 1, 3, 12, 0),
            ),
            Task::mock_scheduled(
                9,
                "1",
                "hurdle",
                2,
                7,
                Slot::mock(chrono::Duration::hours(2), 2023, 1, 5, 1, 0),
            ),
        ];

        let placed_tasks = task_placer(tasks_to_place);
        dbg!(&placed_tasks);
        dbg!(&expected_tasks);

        assert_eq!(expected_tasks, placed_tasks.tasks);
    }
}
