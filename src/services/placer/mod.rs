mod conflicts;

//For a visual step-by-step breakdown of the scheduler algorithm see https://docs.google.com/presentation/d/1Tj0Bg6v_NVkS8mpa-aRtbDQXM-WFkb3MloWuouhTnAM/edit?usp=sharing
use crate::models::goal::{Goal, Tag};
use crate::models::input::{PlacedSteps, StepsToPlace};
use crate::models::slot::Slot;
use crate::models::step::{NewStep, Step, StepStatus};
use crate::models::timeline::Timeline;

/// The Step Placer receives a list of steps from the Step Generator and attempts to assign each
/// step a confirmed start and deadline.
/// The scheduler optimizes for the minimum amount of Impossible steps.
pub fn step_placer(mut steps_to_place: StepsToPlace) -> PlacedSteps {
    dbg!(&steps_to_place);
    //first pass of scheduler while steps are unsplit
    schedule(&mut steps_to_place);
    dbg!(&steps_to_place);

    // TODO 2023-04-29: Fix function adjust_min_budget_steps based on debug feedback check:
    // https://github.com/tijlleenders/ZinZen-scheduler/issues/300#issuecomment-1528727445

    adjust_min_budget_step(&mut steps_to_place); //TODO
    dbg!(&steps_to_place);

    schedule(&mut steps_to_place); //TODO
    dbg!(&steps_to_place);

    PlacedSteps {
        calendar_start: steps_to_place.calendar_start,
        calendar_end: steps_to_place.calendar_end,
        steps: steps_to_place.steps,
    }
}

fn adjust_min_budget_step(steps_to_place: &mut StepsToPlace) {
    let mut steps_to_add: Vec<Step> = Vec::new();
    dbg!(&steps_to_place);

    for index in 0..steps_to_place.steps.len() {
        if steps_to_place.steps[index].status == StepStatus::BudgetMinWaitingForAdjustment {
            for slot_budget in &steps_to_place
                .step_budgets
                .budget_map
                .get(&steps_to_place.steps[index].goal_id)
                .unwrap()
                .slot_budgets
            {
                //TODO If any remaining hours in slot_budget:
                // Loop through BudgetStepMinWaitingForAdjustment Step Vec<Slot> and chop off anything that is outside of the slot_budget Slot
                // Make Step with those slots and remaining hours
                // If not enough hours - mark impossible? No will happen during scheduling.
                // TODO 2023-06-20: idea to refactor below code into a separate function
                let mut step_slots_to_adjust = steps_to_place.steps[index].slots.clone();
                for slot in step_slots_to_adjust.iter_mut() {
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
                for step_slot in step_slots_to_adjust {
                    if step_slot.start.ne(&step_slot.end) {
                        result_slots.push(step_slot);
                    }
                }

                // TODO 2023-06-04  | fix this by using retain instead of this way
                steps_to_place.steps[index].tags.push(Tag::Remove);

                let new_title = steps_to_place.steps[index].title.clone();
                if steps_to_place.steps[index].duration >= slot_budget.used {
                    let new_duration = steps_to_place.steps[index].duration - slot_budget.used;
                    let step_id = steps_to_place.steps[index].id;
                    let goal = Goal {
                        id: steps_to_place.steps[index].goal_id.clone(),
                        title: new_title.clone(),
                        tags: steps_to_place.steps[index].tags.clone(),
                        after_goals: steps_to_place.steps[index].after_goals.clone(),
                        ..Default::default()
                    };
                    let timeline = Timeline {
                        slots: result_slots.into_iter().collect(),
                    };

                    let new_step = NewStep {
                        step_id,
                        title: new_title,
                        duration: new_duration,
                        goal,
                        timeline,
                        status: StepStatus::ReadyToSchedule,
                        timeframe: None,
                    };

                    let step_to_add = Step::new(new_step);

                    steps_to_add.push(step_to_add);
                }
            }
        }
    }
    steps_to_place
        .steps
        .retain(|x| !x.tags.contains(&Tag::Remove));
    steps_to_place.steps.extend(steps_to_add);
}

fn schedule(steps_to_place: &mut StepsToPlace) {
    loop {
        steps_to_place.sort_on_flexibility();
        dbg!(&steps_to_place);
        let first_step = steps_to_place.steps[0].clone();
        dbg!(&first_step);
        if first_step.status != StepStatus::ReadyToSchedule {
            break;
        }
        match conflicts::find_best_slots(&steps_to_place.steps) {
            Some(chosen_slots) => {
                dbg!(&chosen_slots);
                do_the_scheduling(steps_to_place, chosen_slots);
                dbg!(&steps_to_place);
            }
            None => break,
        }
    }
}

fn do_the_scheduling(steps_to_place: &mut StepsToPlace, chosen_slots: Vec<Slot>) {
    dbg!(&steps_to_place, &chosen_slots);
    /*
    TODO 2023-06-06 | Debug notes
    - for code `template_step.duration`, expected causing inaccurate duration for steps
    - think to initialize `template_step.duration` to remaining_hours
    - create a function to initialize scheduled step to minimize effort and clean code
    - for code `template_step.id`, make it realistic numbering. Idea to create function inside Step to generate a new number which not duplicated with current list of steps
    - Todo 2023-06-04  | for code `template_step.id += 1;`, have issue which multiple steps with the same id
    - for code `for slot in chosen_slots.iter()`, just make it function and call it

    Todo 2023-06-11: Need to refactor this function to be testable
    */

    let mut remaining_hours = steps_to_place.steps[0].duration;
    let mut template_step = steps_to_place.steps[0].clone();
    template_step.status = StepStatus::Scheduled;
    // template_step.duration = 1;
    template_step.id = steps_to_place.steps.len();
    template_step.slots.clear();

    for slot in chosen_slots.iter() {
        dbg!(&slot);
        if remaining_hours <= 0 {
            break;
        }

        if !steps_to_place
            .step_budgets
            .is_allowed_by_budget(slot, &template_step.goal_id)
        {
            continue;
        }
        remaining_hours -= slot.duration_as_hours();
        template_step.id += 1;
        template_step.start = Some(slot.start);
        template_step.deadline = Some(slot.end);
        steps_to_place.steps.push(template_step.clone());
    }

    let chosen_slot = chosen_slots[0];
    for step in steps_to_place.steps.iter_mut() {
        dbg!(&step);
        step.remove_conflicted_slots(chosen_slot.to_owned());
        dbg!(&step);
    }
    dbg!(&steps_to_place);

    //Todo remove chosen_slots from StepBudgets
    if remaining_hours > 0 {
        steps_to_place.steps[0].duration = remaining_hours;
        steps_to_place.steps[0].status = StepStatus::Impossible;
    } else {
        steps_to_place.steps.remove(0);

        // TODO 2023-06-06  | apply function Step::remove_from_blocked_by when it is developed
        let _step_scheduled_goal_id = steps_to_place.steps[0].goal_id.clone();
    }

    dbg!(&steps_to_place);
    let _i = 0;
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::models::budget::StepBudgets;

    use super::*;
    use chrono::Duration;

    /// Simulating test case bug_215 when coming to the function `step_placer`
    #[test]
    #[ignore]
    fn test_step_placer_to_simulate_bug_215() {
        /*
        TODO 2023-06-05  | Debug notes
        flexiblity calculation is not accurate as below:
        - For step: "water the plants indoors", correct flexiblity is 14 but it is calculated as 34.
            - FIXME 2023-06-06 | For step: "water the plants indoors", it added slots out of budget. It is noticed inside funciton `schedule`, after function `steps_to_place.sort_on_flexibility()` and before calling function `find_best_slots`
        - For step: "sleep", correct flexibility is 19 but it is calculated as 22.

        # 2023-06-08
        - Steps after function `step_placer` have inaccurate fields "id" and "goal_id"
        */

        let calendar_timing = Slot::mock(Duration::days(7), 2023, 01, 03, 0, 0);

        let steps: Vec<Step> = vec![
            Step::mock(
                "water the plants indoors",
                1,
                0,
                StepStatus::ReadyToSchedule,
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
            Step::mock(
                "dinner",
                1,
                0,
                StepStatus::ReadyToSchedule,
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
            Step::mock(
                "walk",
                1,
                0,
                StepStatus::ReadyToSchedule,
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
            Step::mock(
                "breakfast",
                1,
                0,
                StepStatus::ReadyToSchedule,
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
            Step::mock(
                "me time",
                1,
                0,
                StepStatus::ReadyToSchedule,
                vec![Slot::mock(chrono::Duration::days(7), 2023, 1, 3, 0, 0)],
                None,
            ),
            Step::mock(
                "lunch",
                1,
                0,
                StepStatus::ReadyToSchedule,
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
            Step::mock(
                "hurdle",
                2,
                0,
                StepStatus::ReadyToSchedule,
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
            Step::mock(
                "sleep",
                8,
                0,
                StepStatus::ReadyToSchedule,
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

        let step_budgets = StepBudgets {
            calendar_start: calendar_timing.start,
            calendar_end: calendar_timing.end,
            budget_ids_map: HashMap::new(),
            budget_map: HashMap::new(),
        };
        dbg!(&step_budgets);

        let steps_to_place = StepsToPlace {
            calendar_start: calendar_timing.start,
            calendar_end: calendar_timing.end,
            steps,
            step_budgets,
        };
        dbg!(&steps_to_place);

        let expected_steps: Vec<Step> = vec![
            Step::mock_scheduled(
                9,
                "1",
                "me time",
                1,
                168,
                Slot::mock(chrono::Duration::hours(1), 2023, 1, 3, 09, 0),
            ),
            Step::mock_scheduled(
                9,
                "1",
                "walk",
                1,
                42,
                Slot::mock(chrono::Duration::hours(1), 2023, 1, 3, 14, 0),
            ),
            Step::mock_scheduled(
                9,
                "1",
                "dinner",
                1,
                21,
                Slot::mock(chrono::Duration::hours(1), 2023, 1, 3, 18, 0),
            ),
            Step::mock_scheduled(
                9,
                "1",
                "breakfast",
                1,
                21,
                Slot::mock(chrono::Duration::hours(1), 2023, 1, 3, 08, 0),
            ),
            Step::mock_scheduled(
                9,
                "1",
                "sleep",
                8,
                19,
                Slot::mock(Duration::hours(8), 2023, 01, 03, 0, 0),
            ),
            Step::mock_scheduled(
                9,
                "1",
                "water the plants indoors",
                1,
                14,
                Slot::mock(chrono::Duration::hours(1), 2023, 1, 4, 1, 0),
            ),
            Step::mock_scheduled(
                9,
                "1",
                "lunch",
                1,
                14,
                Slot::mock(chrono::Duration::hours(1), 2023, 1, 3, 12, 0),
            ),
            Step::mock_scheduled(
                9,
                "1",
                "hurdle",
                2,
                7,
                Slot::mock(chrono::Duration::hours(2), 2023, 1, 5, 1, 0),
            ),
        ];

        let placed_steps = step_placer(steps_to_place);
        dbg!(&placed_steps);
        dbg!(&expected_steps);

        assert_eq!(expected_steps, placed_steps.steps);
    }
}
