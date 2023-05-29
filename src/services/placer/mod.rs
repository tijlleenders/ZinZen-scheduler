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
        if tasks_to_place.tasks[0].status != TaskStatus::ReadyToSchedule {
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
    let mut remaining_hours = tasks_to_place.tasks[0].duration;
    let mut template_task = tasks_to_place.tasks[0].clone();
    template_task.status = TaskStatus::Scheduled;
    template_task.duration = 1;
    template_task.id = tasks_to_place.tasks.len();
    template_task.slots.clear();

    for slot in chosen_slots.iter() {
        let slot_allowed = tasks_to_place
            .task_budgets
            .is_allowed_by_budget(slot, &template_task.goal_id);
        if !slot_allowed {
            continue;
        }
        remaining_hours -= slot.duration_as_hours();
        template_task.id += 1;
        template_task.start = Some(slot.start);
        template_task.deadline = Some(slot.end);
        tasks_to_place.tasks.push(template_task.clone());
    }
    for task in tasks_to_place.tasks.iter_mut() {
        for slot in chosen_slots.iter() {
            task.remove_slot(slot.to_owned());
        }
    }
    //Todo remove chosen_slots from TaskBudgets
    if remaining_hours > 0 {
        tasks_to_place.tasks[0].duration = remaining_hours;
        tasks_to_place.tasks[0].status = TaskStatus::Impossible;
    } else {
        let task_scheduled_goal_id = tasks_to_place.tasks[0].goal_id.clone();
        tasks_to_place.tasks.remove(0);
        for task in tasks_to_place.tasks.iter_mut() {
            task.remove_from_blocked_by(task_scheduled_goal_id.clone());
        }
    }
}
