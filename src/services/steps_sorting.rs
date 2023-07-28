use std::cmp::Ordering;

use crate::{
    models::{
        goal::{Goal, Tag},
        step::{Step, StepStatus},
    },
    GOALS,
};

impl PartialEq for Step {
    fn eq(&self, other: &Self) -> bool {
        self.flexibility == other.flexibility
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Step {
    /// ### Custom ordering for collections of Steps:
    ///
    /// TODO!: Rething Tags/Statusses to simplify and make this easier to understand
    ///
    /// **Careful!:** Recalculate flexibilities and re-sort after every Step placement
    /// This is required because finalizing the place(s) on the Calendar of Step makes
    /// those Slots unavailable for other Steps, thus changing their flexibility. Also,
    /// some Steps are waiting for others to be placed, and at some point they are ready to go too.
    ///
    /// 0. Exclude the following Steps from being picked:
    /// - Scheduled
    /// - Impossible
    /// - Uninitialized (should not be there - panic if you find it!)
    /// - Blocked
    /// - BudgetMinWaitingForAdjustment
    /// - ReadyToSchedule with Remove Tag
    ///
    /// 1. Sort on Step Status first using following order:
    /// - ReadyToSchedule without Optional Tag,  without Filler Tag
    /// - ReadyToSchedule without Optional Tag, with Filler Tag
    /// - BudgetMinWaitingForAdjustment - should always be without Optional Tag
    /// - ReadyToSchedule with Optional Tag - with or without FlexDur/FlexNumber Tag
    /// - BudgetMaxWaitingForAdjustment
    ///
    ///
    /// 2. Then apply custom sort on flexibility within the Steps with highest Status:
    /// - If there is a Steps with flexibility 1, pick that one
    /// - If there are no more Steps with flexibility 1 - pick the Step with **highest** flexibility
    fn cmp(&self, other: &Self) -> Ordering {
        if let Ok(goals_lock) = GOALS.try_read() {
            let goals = goals_lock.clone();
            self.custom_compare(other, &goals)
        } else {
            self.original_compare(other)
        }
    }
}

impl Step {
    /// Check if step is parent of other given step
    /// - List of goals is used to check if step is parent of other step
    pub fn is_parent_of_other(&self, other: &Step, goals: &Vec<Goal>) -> bool {
        let self_goal = goals.iter().find(|goal| goal.id == self.goal_id).cloned();
        if self_goal.is_some() {
            if self_goal.clone().unwrap().children.is_some() {
                let is_parent = self_goal
                    .unwrap()
                    .children
                    .unwrap()
                    .iter()
                    .any(|child| *child == other.goal_id);
                if is_parent {
                    return true;
                }
            }
        }
        false
    }

    pub fn is_child_of_other(&self, other: &Step, goals: &Vec<Goal>) -> bool {
        let other_goal = goals.iter().find(|goal| goal.id == other.goal_id).cloned();
        if other_goal.is_some() {
            if other_goal.clone().unwrap().children.is_some() {
                let is_child = other_goal
                    .unwrap()
                    .children
                    .unwrap()
                    .iter()
                    .any(|child| *child == self.goal_id);
                if is_child {
                    return true;
                }
            }
        }
        false
    }

    /// Compare a step with another one using custom compare logic
    /// - Currently it give a priority to child step over parent step
    pub fn custom_compare(&self, other: &Step, goals: &Vec<Goal>) -> Ordering {
        if self.is_parent_of_other(other, goals) {
            return Ordering::Less;
        } else if self.is_child_of_other(other, goals) {
            return Ordering::Greater;
        }

        if (self.status == StepStatus::ReadyToSchedule)
            && !(other.status == StepStatus::ReadyToSchedule)
        {
            Ordering::Less
        } else if (other.status == StepStatus::ReadyToSchedule)
            && !(self.status == StepStatus::ReadyToSchedule)
        {
            Ordering::Greater
        } else if !self.tags.contains(&Tag::Optional) && other.tags.contains(&Tag::Optional) {
            Ordering::Less
        } else if self.tags.contains(&Tag::Optional) && !other.tags.contains(&Tag::Optional) {
            Ordering::Greater
        } else if !self.tags.contains(&Tag::Filler) && other.tags.contains(&Tag::Filler) {
            Ordering::Less
        } else if self.tags.contains(&Tag::Filler) && !other.tags.contains(&Tag::Filler) {
            Ordering::Greater
        } else if self.flexibility == other.flexibility {
            Ordering::Equal
        } else if self.flexibility == 1 {
            Ordering::Less
        } else if other.flexibility == 1 {
            Ordering::Greater
        } else if self.flexibility > other.flexibility {
            Ordering::Less
        } else if other.flexibility > self.flexibility {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }

    /// Compare a step with another one using original comparison when
    ///  developed this system
    pub fn original_compare(&self, other: &Step) -> Ordering {
        // TODO 2023-06-01  | Refactor for readability
        if (self.status == StepStatus::ReadyToSchedule)
            && !(other.status == StepStatus::ReadyToSchedule)
        {
            Ordering::Less
        } else if (other.status == StepStatus::ReadyToSchedule)
            && !(self.status == StepStatus::ReadyToSchedule)
        {
            Ordering::Greater
        } else if !self.tags.contains(&Tag::Optional) && other.tags.contains(&Tag::Optional) {
            Ordering::Less
        } else if self.tags.contains(&Tag::Optional) && !other.tags.contains(&Tag::Optional) {
            Ordering::Greater
        } else if !self.tags.contains(&Tag::Filler) && other.tags.contains(&Tag::Filler) {
            Ordering::Less
        } else if self.tags.contains(&Tag::Filler) && !other.tags.contains(&Tag::Filler) {
            Ordering::Greater
        } else if self.flexibility == other.flexibility {
            Ordering::Equal
        } else if self.flexibility == 1 {
            Ordering::Less
        } else if other.flexibility == 1 {
            Ordering::Greater
        } else if self.flexibility > other.flexibility {
            Ordering::Less
        } else if other.flexibility > self.flexibility {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

mod tests {

    mod parential_relation {
        use crate::{
            mocking::goals::parent_and_childs,
            models::{
                goal::Goal,
                slot::Slot,
                step::{Step, StepStatus},
            },
        };

        #[test]
        fn test_step_have_parent() {
            let goals = parent_and_childs(1);
            let mut parent_step = Step::mock(
                "parent step",
                1,
                1,
                StepStatus::ReadyToSchedule,
                vec![],
                None,
            );
            parent_step.id = 1;
            parent_step.goal_id = goals[0].id.to_string();

            let mut child_step = Step::mock(
                "child step",
                1,
                1,
                StepStatus::ReadyToSchedule,
                vec![],
                None,
            );
            child_step.id = 2;
            child_step.goal_id = goals[1].id.to_string();

            assert!(parent_step.is_parent_of_other(&child_step, &goals));
            assert!(!child_step.is_parent_of_other(&child_step, &goals));
        }

        #[test]
        fn test_step_have_child() {
            let goals = parent_and_childs(1);

            let mut parent_step = Step::mock(
                "parent step",
                1,
                1,
                StepStatus::ReadyToSchedule,
                vec![],
                None,
            );
            parent_step.id = 1;
            parent_step.goal_id = goals[0].id.to_string();

            let mut child_step = Step::mock(
                "child step",
                1,
                1,
                StepStatus::ReadyToSchedule,
                vec![],
                None,
            );
            child_step.id = 2;
            child_step.goal_id = goals[1].id.to_string();

            assert!(child_step.is_child_of_other(&parent_step, &goals));
            assert!(!parent_step.is_child_of_other(&child_step, &goals));
        }
    }

    mod sorting {
        use crate::{
            mocking::goals::parent_and_childs,
            models::{
                goal::{impls::initialize_goals_globally, Goal},
                slot::Slot,
                step::{Step, StepStatus},
            },
        };

        /// Test sorting list of steps wihtout initialize static GOALS,
        /// so it will compare based on original comparison logic which
        /// doesn't check parential relation between steps
        #[test]
        fn test_sort_steps_without_global_goals() {
            let mut goals = parent_and_childs(1);
            goals.extend(parent_and_childs(1));

            let mut parent_step_g1 = Step::mock(
                "parent step",
                1,
                10,
                StepStatus::ReadyToSchedule,
                vec![],
                None,
            );
            parent_step_g1.id = 1;
            parent_step_g1.goal_id = goals[0].id.to_string();

            let mut child_step_g1 = Step::mock(
                "child step",
                1,
                10,
                StepStatus::ReadyToSchedule,
                vec![],
                None,
            );
            child_step_g1.id = 2;
            child_step_g1.goal_id = goals[1].id.to_string();

            let mut parent_step_g2 = Step::mock(
                "parent step",
                1,
                10,
                StepStatus::ReadyToSchedule,
                vec![],
                None,
            );
            parent_step_g2.id = 3;
            parent_step_g2.goal_id = goals[2].id.to_string();

            let mut child_step_g2 = Step::mock(
                "child step",
                1,
                20,
                StepStatus::ReadyToSchedule,
                vec![],
                None,
            );
            child_step_g2.id = 4;
            child_step_g2.goal_id = goals[3].id.to_string();

            let mut steps = vec![
                parent_step_g1,
                child_step_g1,
                parent_step_g2,
                child_step_g2.clone(),
            ];
            dbg!(&steps);
            steps.sort();
            dbg!(&steps);
            assert!(steps[0].id == child_step_g2.id);
        }

        /// Test sorting list of steps wihtout initialize static GOALS,
        /// so it will compare based on original comparison logic which
        /// doesn't check parential relation between steps
        #[test]
        fn test_sort_steps_with_initialize_global_goals() {
            // simulate part of case children_with_over_duration

            let mut goals = parent_and_childs(2);
            goals[0].title = "Project B".to_string();
            goals[1].title = "Research".to_string();
            goals[2].title = "Write report".to_string();
            dbg!(&goals);

            initialize_goals_globally(goals.clone());

            let mut parent_step = Step::mock(
                "Project B",
                1,
                168,
                StepStatus::ReadyToSchedule,
                vec![],
                None,
            );
            parent_step.id = 1;
            parent_step.goal_id = goals[0].id.to_string();

            let mut child_step_1 = Step::mock(
                "Research ",
                3,
                126,
                StepStatus::ReadyToSchedule,
                vec![],
                None,
            );
            child_step_1.id = 2;
            child_step_1.goal_id = goals[1].id.to_string();

            let mut child_step_2 = Step::mock(
                "Write report",
                1,
                168,
                StepStatus::ReadyToSchedule,
                vec![],
                None,
            );
            child_step_2.id = 3;
            child_step_2.goal_id = goals[2].id.to_string();

            let mut steps = vec![parent_step, child_step_1, child_step_2];
            dbg!(&steps);
            steps.sort();
            dbg!(&steps);
            // assert!(steps[0].id == child_step_g1.id);
        }
    }
}
