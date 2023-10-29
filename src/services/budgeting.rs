use crate::models::{
    budget::{BudgetType, StepBudget, StepBudgets},
    goal::{Goal, GoalsMap, Tag},
    repetition::Repetition,
    slot::Slot,
    slots_iterator::TimeSlotsIterator,
    step::{NewStep, Step, StepStatus},
};
use chrono::NaiveDateTime;

impl StepBudgets {
    pub fn configure_budgets(&mut self, goals: &mut GoalsMap) {
        if goals.is_empty() {
            panic!("expected goals for making StepBudgets");
        }

        tag_budgeted_goals(goals);
        self.insert_budgeted_goals(goals);
        self.add_descendants(goals);

        for step_budgets in self.budget_map.values_mut() {
            for step_budget in step_budgets {
                step_budget.initialize(self.calendar_start, self.calendar_end);
            }
        }

        configure_goals_repeatance(goals, Some(self));
    }

    /// Insert budgeted goals into given StepBudgets
    fn insert_budgeted_goals(&mut self, goals: &GoalsMap) {
        // TODO 2023-07-04: create unit tests
        goals
            .iter()
            .filter(|(_, goal)| goal.budgets.is_some())
            .for_each(|(_, goal)| self.insert_goal(goal));
    }

    /// For each budget add all descendants
    fn add_descendants(&mut self, goals: &GoalsMap) {
        // TODO 2023-07-04: create unit tests
        for goal_id in self.budget_map.keys() {
            let mut parents_to_go: Vec<String> = vec![goal_id.clone()]; //start with the goal that initiates the budget
            self.budget_ids_map
                .insert(goal_id.clone(), vec![goal_id.clone()]); //add itself for when budget filler min-max need to be checked with budget
            while !parents_to_go.is_empty() {
                let children = &goals.get(&parents_to_go[0]).unwrap().children;
                if children.is_some() {
                    for child_id in children.as_ref().unwrap() {
                        let temp_to_update = self.budget_ids_map.get_mut(child_id);
                        if let Some(temp) = temp_to_update {
                            temp.push(goal_id.clone());
                        } else {
                            self.budget_ids_map
                                .insert(child_id.clone(), vec![goal_id.clone()]);
                        }
                        parents_to_go.push(child_id.clone());
                    }
                }
                parents_to_go.remove(0);
            }
        }
    }

    pub(crate) fn is_allowed_by_budget(&mut self, slot: &Slot, goal_id: &String) -> bool {
        let mut result: bool = false;
        let budget_ids = self.budget_ids_map.get(goal_id);
        //decrement all budgets or none => check first - then do
        if budget_ids.is_none() {
            return true;
        }
        let mut decrement_all = true;
        for budget_id in budget_ids.unwrap().iter() {
            for budget in &self.budget_map[budget_id] {
                if !budget.test_decrement(slot) {
                    decrement_all = false;
                    break;
                }
            }
        }
        if decrement_all {
            for budget_id in budget_ids.unwrap().iter() {
                for step_budget in self.budget_map.get_mut(budget_id).unwrap().iter_mut() {
                    step_budget.decrement(slot);
                }
            }
            result = true;
        }
        result
    }

    /// Generate Steps only for goals which have budgets
    pub fn generate_steps(&mut self, goals: &GoalsMap, counter: &mut usize) -> Vec<Step> {
        let mut steps_result: Vec<Step> = Vec::new();

        //for each budget create a min step (and optional max step) per corresponding time period
        for (goal_id, step_budgets) in &self.budget_map {
            let goal = goals.get(goal_id).unwrap();

            if let Some(children) = &goal.children {
                if !children.is_empty() {
                    // If a goal is not a 'leaf node' (it has children),
                    // we do not want to generate steps from this goal
                    continue;
                }
            }

            let start: NaiveDateTime = goal.start.unwrap();
            let deadline: NaiveDateTime = goal.deadline.unwrap();

            // if there's no min_duration, steps are scheduled based on the min for a step_budget
            let duration = if let Some(duration) = goal.min_duration {
                duration
            } else {
                step_budgets
                    .iter()
                    .map(|step_budget| step_budget.min.unwrap_or(0))
                    .sum()
            };

            if duration <= 0 {
                continue;
            }

            // TODO 2023-09-27: related to issue https://github.com/tijlleenders/ZinZen-scheduler/issues/300

            // use the most constraining budget time primarily
            let mut minimum_budget_step_size = step_budgets
                .iter()
                .filter(|sb| sb.step_budget_type == BudgetType::Daily)
                .map(|sb| sb.min.unwrap_or(0))
                .max();

            // if there's no applicable daily constraint, constrain based on weekly constraints
            if minimum_budget_step_size.is_none() {
                minimum_budget_step_size = step_budgets
                    .iter()
                    .filter(|sb| sb.step_budget_type == BudgetType::Weekly)
                    .map(|sb| sb.min.unwrap_or(0))
                    .max();
            }

            /*
            TODO 2023-07-4: Found issue that goal.repeat doesn't consider budget_type, which means will not repeat based on budget_type
            - Related to PR https://github.com/tijlleenders/ZinZen-scheduler/pull/358
            */
            let time_slots_iterator =
                TimeSlotsIterator::new(start, deadline, goal.repeat, goal.filters.clone());

            for timeline in time_slots_iterator {
                if !timeline.slots.is_empty() {
                    let step_id = *counter;
                    *counter += 1;

                    let new_step = NewStep {
                        step_id,
                        title: goal.title.clone(),
                        duration,
                        goal: goal.clone(),
                        timeline,
                        status: StepStatus::ReadyToSchedule,
                        timeframe: None,
                    };

                    let step = Step::new(new_step);

                    // split the step based on the budget-imposed constraint
                    let steps = if let Some(min_size) = minimum_budget_step_size {
                        step.split_into_duration(min_size, counter)
                    } else {
                        vec![step]
                    };

                    // apply the duration threshold to the resulting steps
                    let mut thresholded_steps = steps
                        .iter()
                        .map(|step| step.apply_duration_threshold(counter))
                        .flatten()
                        .collect();

                    steps_result.append(&mut thresholded_steps);
                } else {
                    panic!("No timeline slots found")
                }
            }
        }
        steps_result
    }
}

/// Configure each goal.repeat in case goal.repeat is none and step_budgets.
fn configure_goals_repeatance(goals: &mut GoalsMap, step_budgets: Option<&StepBudgets>) {
    // TODO 2023-07-05: create unit tests
    if let Some(step_budgets) = step_budgets {
        goals.iter_mut().for_each(|(goal_id, goal)| {
            if let Some(step_budgets) = step_budgets.budget_map.get(goal_id) {
                for step_budget in step_budgets {
                    goal.configure_repeatance(Some(step_budget));
                }
            }
        })
    } else {
        goals.iter_mut().for_each(|(_, goal)| {
            goal.configure_repeatance(None);
        })
    }
}

/// Tag budgeted goals with Tag::Budget
fn tag_budgeted_goals(goals: &mut GoalsMap) {
    goals
        .iter_mut()
        .filter(|(_, goal)| goal.budgets.is_some())
        .fold((), |_, (_, goal)| {
            goal.tags.push(Tag::Budget);
        });
}

impl Goal {
    /// Configure goal.repeat in case goal.repeat is none but goal have budgets
    /// - If step_budget is None, so Weekly or Daily value will 1
    fn configure_repeatance(self: &mut Goal, step_budget: Option<&StepBudget>) {
        // TODO 2023-07-05: create unit tests specially when step_budget is given
        /*
        Notes:
        - Use StepBudget to calculate count of repeatation whether
        daily or weekly.

        - if step_budget is None, consider repeatance is 1 for daily or weekly
        - if step_budget have value:
            - get count of slot_budgets "step_budget.slot_budgets.len()"
            - if step_budget.step_budget_type is Daily:
                - set goal.repeat to daily(count)
            - else:
                - set goal.repeat to weekly(count)
        */
        let goal = self;
        // if not budgets OR there is a goal.repeat value, do nothing
        if goal.budgets.is_none() || goal.repeat.is_some() {
            return;
        }
        let mut repeatance: usize = 1;

        if let Some(budget) = step_budget {
            repeatance = budget.slot_budgets.len();
            if budget.step_budget_type == BudgetType::Daily {
                goal.repeat = Some(Repetition::DAILY(repeatance));
            } else {
                goal.repeat = Some(Repetition::Weekly(repeatance));
            }
        } else {
            // get goal.budgets:
            //  - If found a daily: set goal.repeat to daily(1)
            //  - else: set goal.repeat to weekly(1)
            if let Some(budgets) = &goal.budgets {
                let found_daily_budget = budgets
                    .iter()
                    .any(|budget| budget.budget_type == BudgetType::Daily);

                if found_daily_budget {
                    goal.repeat = Some(Repetition::DAILY(repeatance));
                } else {
                    goal.repeat = Some(Repetition::Weekly(repeatance));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    mod generate_steps {
        use crate::models::{
            budget::{Budget, BudgetType, SlotBudget, StepBudget, StepBudgets},
            goal::{Goal, GoalsMap, Tag},
            repetition::Repetition,
            slot::Slot,
            step::{Step, StepStatus},
        };
        use chrono::Duration;
        use std::collections::HashMap;

        /// Test generate_steps with a budget with < 8 hours weekly.
        /// Should generate a single step corresponding to the minimal step needed to satisfy the budget min
        /// This step should be ready to schedule
        #[test]
        fn generate_steps_generates_minimal_step_to_satify_budget_min() {
            // Data
            let title: String = "4-work".to_string();
            let calendar = Slot::mock(Duration::days(7), 2018, 1, 1, 0, 0);

            let mut goal_id_to_budget_ids: HashMap<String, Vec<String>> = HashMap::new();
            goal_id_to_budget_ids.insert(title.clone(), vec![title.clone()]);

            let mut budget_id_to_budget: HashMap<String, Vec<StepBudget>> = HashMap::new();
            budget_id_to_budget.insert(
                title.clone(),
                vec![StepBudget {
                    step_budget_type: BudgetType::Weekly,
                    slot_budgets: vec![SlotBudget {
                        slot: calendar,
                        min: Some(5),
                        max: None,
                        used: 0,
                    }],
                    min: Some(5),
                    max: None,
                }],
            );

            let mut step_budgets = StepBudgets {
                calendar_start: calendar.start,
                calendar_end: calendar.end,
                budget_ids_map: goal_id_to_budget_ids,
                budget_map: budget_id_to_budget,
            };

            let work_goal = Goal {
                id: title.clone(),
                title: title.clone(),
                min_duration: Some(5),
                max_duration: None,
                budgets: Some(vec![Budget {
                    budget_type: BudgetType::Weekly,
                    min: Some(5),
                    max: None,
                }]),
                repeat: Some(Repetition::Weekly(1)),
                start: Some(calendar.start),
                deadline: Some(calendar.end),
                tags: vec![Tag::Budget],
                filters: None,
                children: None,
                after_goals: None,
            };

            let mut goals: GoalsMap = GoalsMap::new();
            goals.insert(work_goal.id.clone(), work_goal);
            let mut counter = 0;
            let result_steps = step_budgets.generate_steps(&goals, &mut counter);

            // Expected steps data
            let expected_steps: Vec<Step> = vec![Step {
                id: 0,
                goal_id: title.clone(),
                title: title.clone(),
                duration: 5,
                status: StepStatus::ReadyToSchedule,
                flexibility: 0,
                start: None,
                deadline: None,
                slots: vec![calendar],
                tags: vec![Tag::Budget],
                after_goals: None,
            }];

            assert_eq!(result_steps, expected_steps);
        }

        /// Test generate_steps with a budget with > 8 hours weekly.
        /// Should generate splitted steps in 1 hour blocks that combined add up to the budget min.
        /// This step should be ready to schedule
        #[test]
        fn generate_steps_generates_splitted_steps_to_satify_budget_min() {
            // Data
            let title: String = "4-work".to_string();
            let calendar = Slot::mock(Duration::days(7), 2018, 1, 1, 0, 0);

            let mut goal_id_to_budget_ids: HashMap<String, Vec<String>> = HashMap::new();
            goal_id_to_budget_ids.insert(title.clone(), vec![title.clone()]);

            let mut budget_id_to_budget: HashMap<String, Vec<StepBudget>> = HashMap::new();
            budget_id_to_budget.insert(
                title.clone(),
                vec![StepBudget {
                    step_budget_type: BudgetType::Weekly,
                    slot_budgets: vec![SlotBudget {
                        slot: calendar,
                        min: Some(9),
                        max: None,
                        used: 0,
                    }],
                    min: Some(9),
                    max: None,
                }],
            );

            let mut step_budgets = StepBudgets {
                calendar_start: calendar.start,
                calendar_end: calendar.end,
                budget_ids_map: goal_id_to_budget_ids,
                budget_map: budget_id_to_budget,
            };

            let work_goal = Goal {
                id: title.clone(),
                title: title.clone(),
                min_duration: Some(9),
                max_duration: None,
                budgets: Some(vec![Budget {
                    budget_type: BudgetType::Weekly,
                    min: Some(9),
                    max: None,
                }]),
                repeat: Some(Repetition::Weekly(1)),
                start: Some(calendar.start),
                deadline: Some(calendar.end),
                tags: vec![Tag::Budget],
                filters: None,
                children: None,
                after_goals: None,
            };

            let mut goals: GoalsMap = GoalsMap::new();
            goals.insert(work_goal.id.clone(), work_goal);
            let mut counter = 0;
            let result_steps = step_budgets.generate_steps(&goals, &mut counter);

            // Expected steps data: 9 different identical steps (differing only in id) with duration 1
            let indices: Vec<usize> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
            let expected_steps: Vec<Step> = indices
                .iter()
                .map(|index| Step {
                    id: *index,
                    goal_id: title.clone(),
                    title: title.clone(),
                    duration: 1,
                    status: StepStatus::ReadyToSchedule,
                    flexibility: 0,
                    start: None,
                    deadline: None,
                    slots: vec![calendar],
                    tags: vec![Tag::Budget],
                    after_goals: None,
                })
                .collect();

            assert_eq!(result_steps, expected_steps);
            assert_eq!(result_steps[0], expected_steps[0]);
        }

        #[test]
        fn generate_steps_does_not_consider_non_leaf_nodes() {
            // Data parent
            let parent_title: String = "parent".to_string();
            let child_title: String = "child".to_string();

            let calendar = Slot::mock(Duration::days(7), 2018, 1, 1, 0, 0);

            let mut goal_id_to_budget_ids: HashMap<String, Vec<String>> = HashMap::new();
            goal_id_to_budget_ids.insert(parent_title.clone(), vec![parent_title.clone()]);
            goal_id_to_budget_ids.insert(child_title.clone(), vec![child_title.clone()]);

            let mut budget_id_to_budget: HashMap<String, Vec<StepBudget>> = HashMap::new();
            budget_id_to_budget.insert(
                parent_title.clone(),
                vec![StepBudget {
                    step_budget_type: BudgetType::Weekly,
                    slot_budgets: vec![SlotBudget {
                        slot: calendar,
                        min: Some(5),
                        max: None,
                        used: 0,
                    }],
                    min: Some(5),
                    max: None,
                }],
            );
            budget_id_to_budget.insert(
                child_title.clone(),
                vec![StepBudget {
                    step_budget_type: BudgetType::Weekly,
                    slot_budgets: vec![SlotBudget {
                        slot: calendar,
                        min: Some(3),
                        max: None,
                        used: 0,
                    }],
                    min: Some(3),
                    max: None,
                }],
            );

            let mut step_budgets = StepBudgets {
                calendar_start: calendar.start,
                calendar_end: calendar.end,
                budget_ids_map: goal_id_to_budget_ids,
                budget_map: budget_id_to_budget,
            };

            let parent_goal = Goal {
                id: parent_title.clone(),
                title: parent_title.clone(),
                min_duration: Some(5),
                max_duration: None,
                budgets: Some(vec![Budget {
                    budget_type: BudgetType::Weekly,
                    min: Some(5),
                    max: None,
                }]),
                repeat: Some(Repetition::Weekly(1)),
                start: Some(calendar.start),
                deadline: Some(calendar.end),
                tags: vec![Tag::Budget],
                filters: None,
                children: Some(vec!["child".to_string()]),
                after_goals: None,
            };

            let child_goal = Goal {
                id: child_title.clone(),
                title: child_title.clone(),
                min_duration: Some(3),
                max_duration: None,
                budgets: Some(vec![Budget {
                    budget_type: BudgetType::Weekly,
                    min: Some(3),
                    max: None,
                }]),
                repeat: Some(Repetition::Weekly(1)),
                start: Some(calendar.start),
                deadline: Some(calendar.end),
                tags: vec![Tag::Budget],
                filters: None,
                children: None,
                after_goals: None,
            };

            let mut goals: GoalsMap = GoalsMap::new();
            goals.insert(parent_goal.id.clone(), parent_goal);
            goals.insert(child_goal.id.clone(), child_goal);
            let mut counter = 0;
            let result_steps = step_budgets.generate_steps(&goals, &mut counter);

            // Expected steps data
            let expected_steps: Vec<Step> = vec![Step {
                id: 0,
                goal_id: child_title.clone(),
                title: child_title.clone(),
                duration: 3,
                status: StepStatus::ReadyToSchedule,
                flexibility: 0,
                start: None,
                deadline: None,
                slots: vec![calendar],
                tags: vec![Tag::Budget],
                after_goals: None,
            }];

            assert_eq!(result_steps, expected_steps);
        }
    }

    mod goals_map {
        use crate::{
            models::{
                budget::{Budget, BudgetType},
                goal::{Goal, GoalsMap, Tag},
                slot::Slot,
            },
            services::budgeting::tag_budgeted_goals,
        };

        #[test]
        fn test_tag_budgeted_goals() {
            let mut goals: GoalsMap = GoalsMap::new();

            // Create budgeted goal and insert to goals
            let mut budgeted_goal = Goal::mock("1", "test goal", Slot::mock_sample());
            let budget = Budget {
                budget_type: BudgetType::Daily,
                min: Some(5),
                max: Some(10),
            };
            budgeted_goal.budgets = Some(vec![budget]);
            goals.insert(budgeted_goal.id.clone(), budgeted_goal.clone());

            // Create goal with no budgets and insert to goals
            let goal = Goal::mock("2", "test goal 2", Slot::mock_sample());
            goals.insert(goal.id.clone(), goal.clone());

            tag_budgeted_goals(&mut goals);
            let result_budgeted_goal = goals.get(&budgeted_goal.id).unwrap();
            assert!(result_budgeted_goal.tags.contains(&Tag::Budget));

            let result_normal_goal = goals.get(&goal.id).unwrap();
            assert!(!result_normal_goal.tags.contains(&Tag::Budget));
        }
    }

    mod goal {
        mod configure_repeatance {
            use crate::models::{
                budget::{Budget, BudgetType},
                goal::Goal,
                repetition::Repetition,
                slot::Slot,
            };

            #[test]
            fn test_no_goal_repeat_no_budget() {
                let mut goal = Goal::mock("1", "sample goal", Slot::mock_sample());

                goal.configure_repeatance(None);
                assert_eq!(goal.repeat, None);
            }

            #[test]
            fn test_have_budget_and_repeat() {
                let mut goal = Goal::mock("1", "sample goal", Slot::mock_sample());

                let budget = Budget {
                    budget_type: BudgetType::Daily,
                    min: Some(5),
                    max: Some(10),
                };
                let budgets = Some(vec![budget]);
                goal.budgets = budgets.clone();

                let repeat = Some(Repetition::Weekly(1));
                goal.repeat = repeat;

                goal.configure_repeatance(None);

                assert_eq!(goal.repeat, repeat);
                assert_eq!(goal.budgets, budgets);
            }

            #[test]
            fn test_have_weekly_budget_but_no_repeat() {
                let mut goal = Goal::mock("1", "sample goal", Slot::mock_sample());

                let budget = Budget {
                    budget_type: BudgetType::Weekly,
                    min: Some(30),
                    max: Some(40),
                };
                let budgets = Some(vec![budget]);
                goal.budgets = budgets.clone();
                goal.repeat = None;

                goal.configure_repeatance(None);

                let expected_repeat = Some(Repetition::Weekly(1));

                assert_eq!(goal.repeat, expected_repeat);
                assert_eq!(goal.budgets, budgets);
            }

            #[test]
            fn test_have_daily_budget_but_no_repeat() {
                let mut goal = Goal::mock("1", "sample goal", Slot::mock_sample());

                let budget = Budget {
                    budget_type: BudgetType::Daily,
                    min: Some(5),
                    max: Some(10),
                };
                let budgets = Some(vec![budget]);
                goal.budgets = budgets.clone();
                goal.repeat = None;

                goal.configure_repeatance(None);

                let expected_repeat = Some(Repetition::DAILY(1));

                assert_eq!(goal.repeat, expected_repeat);
                assert_eq!(goal.budgets, budgets);
            }

            #[test]
            fn test_have_daily_and_weekly_budget_but_no_repeat() {
                let mut goal = Goal::mock("1", "sample goal", Slot::mock_sample());

                let daily_budget = Budget {
                    budget_type: BudgetType::Daily,
                    min: Some(5),
                    max: Some(10),
                };
                let weekly_budget = Budget {
                    budget_type: BudgetType::Weekly,
                    min: Some(30),
                    max: Some(40),
                };
                let budgets = Some(vec![daily_budget, weekly_budget]);
                goal.budgets = budgets.clone();
                goal.repeat = None;

                goal.configure_repeatance(None);

                let expected_repeat = Some(Repetition::DAILY(1));

                assert_eq!(goal.repeat, expected_repeat);
                assert_eq!(goal.budgets, budgets);
            }
        }
    }
}
