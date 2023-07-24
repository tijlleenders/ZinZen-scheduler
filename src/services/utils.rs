use std::collections::HashSet;

use rand::Rng;

use crate::models::step::Step;

/// Generate random Step ID number
/// - Created mainly to provide a consistent unique numbering for step.id
pub fn generate_step_id() -> usize {
    let mut rng = rand::thread_rng();
    rng.gen()
}

/// Unifies steps IDs by making sure all steps IDs are unique and not duplicated
pub fn unify_steps_ids(steps_list: &mut Vec<Step>) {
    let ids_set: HashSet<usize> = (0..steps_list.len()).map(|_| generate_step_id()).collect();
    dbg!(&ids_set);
    // steps_list.iter_mut().enumerate().for_each(|(index, step)| {

    //     step.id = generate_step_id();
    // });
    let mut id_iter = ids_set.iter();
    dbg!(&id_iter);
    steps_list.iter_mut().for_each(|step| match id_iter.next() {
        Some(next_id) => step.id = *next_id,
        None => step.id = generate_step_id(),
    });
}

#[cfg(test)]
mod tests {
    use crate::{
        models::step::{Step, StepStatus},
        services::utils::{generate_step_id, unify_steps_ids},
    };

    #[test]
    fn test_generate_step_id() {
        // Store ids in a HashSet to confirm uniqueness
        let mut id_set = std::collections::HashSet::new();
        for _ in 0..1000 {
            let step_id = generate_step_id();
            assert!(id_set.insert(step_id), "Duplicate ID generated");
        }
    }

    #[test]
    fn test_unify_steps_ids() {
        let sample_str = String::new();
        let sample_status = StepStatus::Scheduled;

        let mut steps_list = vec![
            Step {
                id: 1,
                goal_id: sample_str.clone(),
                title: sample_str.clone(),
                duration: 0,
                status: sample_status.clone(),
                flexibility: 0,
                start: None,
                deadline: None,
                slots: vec![],
                tags: vec![],
                after_goals: None,
            },
            Step {
                id: 1,
                goal_id: sample_str.clone(),
                title: sample_str.clone(),
                duration: 0,
                status: sample_status.clone(),
                flexibility: 0,
                start: None,
                deadline: None,
                slots: vec![],
                tags: vec![],
                after_goals: None,
            },
            Step {
                id: 2,
                goal_id: sample_str.clone(),
                title: sample_str.clone(),
                duration: 0,
                status: sample_status.clone(),
                flexibility: 0,
                start: None,
                deadline: None,
                slots: vec![],
                tags: vec![],
                after_goals: None,
            },
            Step {
                id: 3,
                goal_id: sample_str.clone(),
                title: sample_str.clone(),
                duration: 0,
                status: sample_status.clone(),
                flexibility: 0,
                start: None,
                deadline: None,
                slots: vec![],
                tags: vec![],
                after_goals: None,
            },
            Step {
                id: 3,
                goal_id: sample_str.clone(),
                title: sample_str.clone(),
                duration: 0,
                status: sample_status.clone(),
                flexibility: 0,
                start: None,
                deadline: None,
                slots: vec![],
                tags: vec![],
                after_goals: None,
            },
            Step {
                id: 4,
                goal_id: sample_str.clone(),
                title: sample_str.clone(),
                duration: 0,
                status: sample_status.clone(),
                flexibility: 0,
                start: None,
                deadline: None,
                slots: vec![],
                tags: vec![],
                after_goals: None,
            },
        ];
        dbg!(&steps_list);

        unify_steps_ids(&mut steps_list);
        dbg!(&steps_list);

        let mut id_set = std::collections::HashSet::new();

        // Check if all IDs are unique and not duplicated
        for step in &steps_list {
            assert!(!id_set.contains(&step.id), "Duplicate ID found");
            id_set.insert(step.id);
        }
    }
}
