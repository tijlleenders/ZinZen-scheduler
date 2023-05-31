/// Generate and Handle budgeting for tasks
pub mod budgeting;
/// New filter functions which using SlotIterator and TimelineItertor
pub mod filter;
/// Calculate flexibility for tasks
pub mod flexibility;
/// Merges all FinalSlots Timelines from all Tasks into a single Timeline, formatted for the frontend
pub mod output;
/// For each Task, reduce the MaybeSlots Timeline into a FinalSlots Timeline, where the next Task to pick is calculated anew after every handled Task
pub mod placer;
/// Generates Tasks from Goals, giving each Task their [MECE part](https://en.wikipedia.org/wiki/MECE_principle) of the Goal MaybeSlots Timeline
pub mod task_generator;
