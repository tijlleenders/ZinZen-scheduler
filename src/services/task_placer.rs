//For a visual step-by-step breakdown of the scheduler algorithm see https://docs.google.com/presentation/d/1Tj0Bg6v_NVkS8mpa-aRtbDQXM-WFkb3MloWuouhTnAM/edit?usp=sharing
use crate::models::goal::Tag;
use crate::models::input::{PlacedTasks, TasksToPlace};
use crate::models::slot::{Slot, SlotConflict};
use crate::models::task::{Task, TaskStatus};

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
                    // TODO 2023-04-28 | shouldn't add tasks in task_placer. Refactor this (conceptually in task_generator).
                    let task_to_add = Task {
                        id: tasks_to_place.tasks[index].id,
                        goal_id: tasks_to_place.tasks[index].goal_id.clone(),
                        title: new_title,
                        duration: new_duration,
                        start: None,
                        deadline: None,
                        calendar_start: tasks_to_place.tasks[index].calendar_start,
                        calendar_end: tasks_to_place.tasks[index].calendar_end,
                        slots: result_slots,
                        status: TaskStatus::ReadyToSchedule,
                        tags: tasks_to_place.tasks[index].tags.clone(),
                        after_goals: tasks_to_place.tasks[index].after_goals.clone(),
                        flexibility: 0,
                    };
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
        if tasks_to_place.tasks[0].status != TaskStatus::ReadyToSchedule {
            break;
        }
        match find_best_slots(&tasks_to_place.tasks) {
            Some(chosen_slots) => do_the_scheduling(tasks_to_place, chosen_slots),
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
        remaining_hours -= slot.calc_duration_in_hours();
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

fn find_best_slots(tasks_to_place: &Vec<Task>) -> Option<Vec<Slot>> {
    let mut slot_conflicts: Vec<SlotConflict> = vec![];
    let task = &tasks_to_place[0];

    for slot in task.slots.iter() {
        for hour_slot in slot.divide_into_1h_slots() {
            let mut count: usize = 0;
            'outer: for t in tasks_to_place {
                if t.status != TaskStatus::ReadyToSchedule {
                    continue;
                }
                if t.id == task.id {
                    continue;
                }

                for s in t.slots.iter() {
                    if s.is_intersect_with_slot(&hour_slot) {
                        count += 1;
                        continue 'outer;
                    }
                }
            }
            slot_conflicts.push(SlotConflict {
                slot: hour_slot,
                num_conflicts: count,
            });
        }
    }
    slot_conflicts.sort_by(|a, b| b.slot.start.partial_cmp(&a.slot.start).unwrap());

    slot_conflicts.sort_by(|a, b| b.num_conflicts.partial_cmp(&a.num_conflicts).unwrap());

    let mut result = vec![];
    for _dur in 0..task.duration {
        match slot_conflicts.pop() {
            Some(s) => result.push(s.slot),
            None => break,
        }
    }

    Some(result)
}
