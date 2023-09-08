use super::{Step, StepStatus};
use crate::models::{goal::Tag, slot::Slot};
use std::cmp::Ordering;

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
    /// - ReadyToSchedule with Remove Tag
    ///
    /// 1. Sort on Step Status first using following order:
    /// - ReadyToSchedule without Optional Tag,  without Filler Tag
    /// - ReadyToSchedule without Optional Tag, with Filler Tag
    /// - ReadyToSchedule with Optional Tag - with or without FlexDur/FlexNumber Tag
    /// - BudgetMaxWaitingForAdjustment
    ///
    ///
    /// 2. Then apply custom sort on flexibility within the Steps with highest Status:
    /// - If there is a Steps with flexibility 1, pick that one
    /// - If there are no more Steps with flexibility 1 - pick the Step with **highest** flexibility
    fn cmp(&self, other: &Self) -> Ordering {
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

impl Step {
    pub fn flexibility(&mut self) -> usize {
        self.flexibility
    }

    pub fn get_slots(&self) -> Vec<Slot> {
        self.slots.clone()
    }

    /// Remove conflicted step slots with a given slot [slot_to_remove]
    /// - Function will do nothing with Scheduled steps
    pub fn remove_conflicted_slots(&mut self, slot_to_remove: Slot) {
        /*
        TODO 2023-06-10: Add test case to guerntee not adding extra hours for the Step.slot
        Todo: duplicate of remove_taken_slots? (NOTE: This todo need to be reviewed)
        ====
        Todo 2023-06-08:
        - create a test case and avoid using remove_taken_slots or btter approach.
        Todo 2023-06-09:
        - removed calling Step::remove_taken_slots in case StepStatus is Blocked
        becasue it is not functional properly and need to be fixed.
        */

        if self.status == StepStatus::Scheduled {
            return;
        }

        let mut slots_after_filter = Vec::new();
        for slot in &mut self.slots {
            let step_slot = *slot;
            let subtracted_slot = step_slot - slot_to_remove;
            slots_after_filter.extend(subtracted_slot);
        }

        self.slots = slots_after_filter;
        self.calculate_flexibility();
    }

    pub fn remove_taken_slots(&mut self, slot_to_remove: Slot) {
        // TODO 2023-06-09  | This function is not accurate which need to be fixed and create test cases.
        let mut slots_after_filter = Vec::new();
        for step_slot in &mut self.slots {
            if step_slot.start >= slot_to_remove.end {
                slots_after_filter.push(*step_slot);
            }
            if step_slot.end > slot_to_remove.end && step_slot.start < slot_to_remove.start {
                step_slot.start = slot_to_remove.start;
                slots_after_filter.push(*step_slot);
            }
            if step_slot.end > slot_to_remove.end && step_slot.start >= slot_to_remove.start {
                slots_after_filter.push(*step_slot);
            }
            if !(step_slot.end <= slot_to_remove.end && step_slot.start <= slot_to_remove.start) {
                slots_after_filter.push(*step_slot);
            }
        }

        self.slots = slots_after_filter;
    }

    pub fn remove_from_blocked_by(&mut self, _id_string: String) {
        // TODO | 2023-06-06 | Seeking more info about this function
        // if self.after_goals.is_none() {
        //     return;
        // }
        // let mut ids = self.after_goals.clone().unwrap();
        // let index = ids.clone().iter().position(|x| x.eq(&id_string));
        // if index.is_some() {
        //     ids.remove(index.unwrap());
        //     if ids.is_empty() {
        //         self.after_goals = None;
        //         self.status = StepStatus::ReadyToSchedule;
        //     } else {
        //         self.after_goals = Some(ids);
        //     }
        // }
    }
}

#[cfg(test)]
mod tests {

    mod remove_conflicted_slots {
        use crate::models::{
            slot::Slot,
            step::{Step, StepStatus},
        };
        use chrono::Duration;

        /// Testing edge case in bug_215 which slot_to_remove
        /// is bigger than step_slot and step_slot is contained in slot_to_remove
        ///
        /// ```markdown
        /// # "chosen_slot" to be removed from all steps:
        /// slot_to_remove: 2023-01-03 00 to 08 (8 hours)
        /// # "step_slot" which has less duration than chosen_slot but not removed
        /// step_slot: 2023-01-03 01 to 03 (2 hour)
        /// # Also slot_to_remove contains step_slot
        ///
        /// # Output should be:
        /// step.slots.len(): 0
        /// ```
        #[test]
        fn test_intersected_bigger_slot() {
            let slot_to_remove = Slot::mock(Duration::hours(8), 2023, 1, 3, 0, 0);
            let mut step = Step::mock(
                "test",
                1,
                12,
                StepStatus::ReadyToSchedule,
                vec![Slot::mock(Duration::hours(2), 2023, 1, 3, 1, 0)],
                None,
            );

            step.remove_conflicted_slots(slot_to_remove);

            assert_eq!(step.slots.len(), 0);
        }

        #[test]
        fn test_step_is_scheduled() {
            let slot_to_remove = Slot::mock(Duration::hours(8), 2023, 1, 3, 0, 0);
            let mut step = Step::mock_scheduled(
                1,
                "1",
                "test",
                1,
                12,
                Slot::mock(Duration::hours(2), 2023, 1, 3, 1, 0),
            );

            step.remove_conflicted_slots(slot_to_remove);
            let step_after_remove = step.clone();

            assert_eq!(step_after_remove, step);
        }

        /// Test non intersected step's slots with a given slot (slot_to_remove)
        /// which returns the same step_slot
        #[test]
        fn test_nonintersected_slot() {
            let slot_to_remove = Slot::mock(Duration::hours(8), 2023, 1, 3, 0, 0);
            let step_slot = Slot::mock(Duration::hours(10), 2023, 1, 4, 1, 0);
            let mut step = Step::mock(
                "test",
                1,
                12,
                StepStatus::ReadyToSchedule,
                vec![step_slot],
                None,
            );

            step.remove_conflicted_slots(slot_to_remove);

            assert_eq!(step.slots[0], step_slot);
        }

        /// Testing normal case which removing conflicted step's slots with
        /// slot_to_remove
        /// ```markdown
        /// slot_to_remove: 2023-01-03 00 to 03 (3 hours)
        ///
        /// step_slot: 2023-01-03 01 to 11 (10 hour)
        ///
        /// Expected:
        /// step_slot: 2023-01-03 03 to 11 (8 hour)
        /// ```
        #[test]
        fn test_normal() {
            let slot_to_remove = Slot::mock(Duration::hours(3), 2023, 1, 3, 0, 0);
            let step_slot = Slot::mock(Duration::hours(10), 2023, 1, 3, 1, 0);
            let mut step = Step::mock(
                "test",
                1,
                12,
                StepStatus::ReadyToSchedule,
                vec![step_slot],
                None,
            );

            step.remove_conflicted_slots(slot_to_remove);

            let expected_step_slot = Slot::mock(Duration::hours(8), 2023, 1, 3, 3, 0);

            assert_eq!(step.slots[0], expected_step_slot);
        }
    }
}
