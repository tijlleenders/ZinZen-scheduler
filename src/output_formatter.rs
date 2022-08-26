//new module for outputting the result of task_placer in
//whichever format required by front-end
use crate::task::Task;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct Output {
	taskid: usize,
	goalid: usize,
	title: String,
	duration: usize,
	start: NaiveDateTime,
	deadline: NaiveDateTime,
}

pub fn output_formatter(tasks: Vec<Task>) -> Result<Vec<Output>> {
	let mut outputs = Vec::new();
	for task in tasks {
		let output = Output {
			taskid: task.id,
			goalid: task.goal_id,
			title: task.title,
			duration: task.duration,
			start: task.confirmed_start.unwrap(),
			deadline: task.confirmed_deadline.unwrap(),
		};
		outputs.push(output);
	}
	Ok(outputs)
}
