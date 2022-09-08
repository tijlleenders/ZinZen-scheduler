use crate::task::Task;
use crate::time_slice_iterator::{Repetition, TimeSliceIterator};
use chrono::Duration;
use chrono::NaiveDateTime;
use serde::Deserialize;
use std::option::Option;

#[derive(Deserialize, Debug, Default)]
pub struct Goal {
	pub id: usize,
	pub title: String,
	/// How much total time should a user put into their goal, eg "I want to learn how to code, and I want to code 6 hours per day"
	pub duration: usize,
	pub repetition: Option<Repetition>,
	/// start date bound for this Goal's Tasks
	#[serde(default)]
	pub start: Option<NaiveDateTime>,
	/// deadline date bound for this Goal's Tasks
	#[serde(default)]
	pub deadline: Option<NaiveDateTime>,
	/// start time bound after which activity should be done
	#[serde(default)]
	pub after_time: Option<usize>,
	/// deadline time bound before which activity should be done
	#[serde(default)]
	pub before_time: Option<usize>,
}

//#[cfg(test)]
impl Goal {
	pub fn new(id: usize) -> Self {
		Self {
			id,
			title: String::from("Test"),
			..Default::default()
		}
	}

	pub fn duration(mut self, duration: usize) -> Self {
		self.duration = duration;
		self
	}

	pub fn repetition(mut self, repetition: Repetition) -> Self {
		self.repetition = Some(repetition);
		self
	}

	pub fn start(mut self, start: NaiveDateTime) -> Self {
		self.start = Some(start);
		self
	}

	pub fn deadline(mut self, deadline: NaiveDateTime) -> Self {
		self.deadline = Some(deadline);
		self
	}

	pub fn after_time(mut self, after_time: usize) -> Self {
		self.after_time = Some(after_time);
		self
	}

	pub fn before_time(mut self, before_time: usize) -> Self {
		self.before_time = Some(before_time);
		self
	}

	pub fn generate_tasks(self, calendar_start: NaiveDateTime, calendar_end: NaiveDateTime) -> Vec<Task> {
		let mut tasks = Vec::new();
		//If there is a repetion in the goal, a different task will be generated for each day of the repetition.
		//e.g. if the repetition is DAILY, a different task will be generated for each day.
		//e.g. if the repetition is MONDAYS, a different task will be generated for each monday.
		match self.repetition {
			Some(rep) => {
				let time_slices = TimeSliceIterator {
					start: self.start.unwrap_or(calendar_start),
					end: self.deadline.unwrap_or(calendar_end),
					repetition: rep,
				};
				let mut id = 0;
				for (start, deadline) in time_slices {
					let task_id = format!("{}{}", self.id, id);
					let deadline = if self.before_time.unwrap_or(24) < self.after_time.unwrap_or(0) {
						deadline + Duration::days(1)
					} else {
						deadline
					};
					let t = Task::new(
						task_id.parse::<usize>().unwrap(),
						self.id,
						self.title.clone(),
						self.duration,
						start,
						deadline,
						self.after_time.unwrap_or(0),
						self.before_time.unwrap_or(24),
					);
					tasks.push(t);
					id = id + 1;
				}
			}
			//If there is no repetition, the task's start and deadline are equivalent to the goal's
			//start/deadline.
			None => {
				let task_id = format!("{}{}", self.id, 0);
				let t = Task::new(
					task_id.parse::<usize>().unwrap(),
					self.id,
					self.title,
					self.duration,
					self.start.unwrap_or(calendar_start),
					self.deadline.unwrap_or(calendar_end),
					self.after_time.unwrap_or(0),
					self.before_time.unwrap_or(24),
				);
				tasks.push(t);
			}
		}
		tasks
	}
}
