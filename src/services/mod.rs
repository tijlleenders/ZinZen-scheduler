pub mod filter;
/// Merges all FinalSlots Timelines from all Tasks into a single Timeline, formatted for the frontend
pub mod output;
/// Generates Tasks from Goals, giving each Task their [MECE part](https://en.wikipedia.org/wiki/MECE_principle) of the Goal MaybeSlots Timeline
pub mod task_generator;
/// For each Task, reduce the MaybeSlots Timeline into a FinalSlots Timeline, where the next Task to pick is calculated anew after every handled Task
pub mod task_placer;
