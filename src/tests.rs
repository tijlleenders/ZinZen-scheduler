#[allow(unused_imports)]
use crate::{
	goal::{Goal, Repetition},
	preprocessor::PreProcessor,
};

#[test]
pub(crate) fn test_preprocessor() {
	let goals = &[
		Goal {
			repetition: Repetition::Once,
			..Default::default()
		},
		Goal {
			repetition: Repetition::Daily,
			..Default::default()
		},
		Goal {
			repetition: Repetition::Weekly,
			..Default::default()
		},
		Goal {
			repetition: Repetition::Monthly,
			..Default::default()
		},
		Goal {
			repetition: Repetition::Monthly,
			..Default::default()
		},
		Goal {
			repetition: Repetition::Annually,
			..Default::default()
		},
	];

	let tasks = PreProcessor::generate_tasks(goals, 24 * 7 * 4 * 2);

	let once_count = tasks
		.iter()
		.filter(|task| matches!(task.goal.repetition, Repetition::Once))
		.count();
	assert_eq!(once_count, 1);

	let dailies_count = tasks
		.iter()
		.filter(|task| matches!(task.goal.repetition, Repetition::Daily))
		.count();
	assert_eq!(dailies_count, 7 * 4 * 2);

	let weekly_count = tasks
		.iter()
		.filter(|task| matches!(task.goal.repetition, Repetition::Weekly))
		.count();
	assert_eq!(weekly_count, 4 * 2);

	let monthly_count = tasks
		.iter()
		.filter(|task| matches!(task.goal.repetition, Repetition::Monthly))
		.count();
	assert_eq!(monthly_count, 2 * 2);

	let annually_count = tasks
		.iter()
		.filter(|task| matches!(task.goal.repetition, Repetition::Annually))
		.count();
	assert_eq!(annually_count, 1);
}
