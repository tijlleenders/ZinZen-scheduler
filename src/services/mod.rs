/// Generate and Handle budgeting for steps
pub mod budgeting;
/// New filter functions which using SlotIterator and TimelineItertor
pub mod filter;
/// Calculate flexibility for steps
pub mod flexibility;
/// Merges all FinalSlots Timelines from all Steps into a single Timeline, formatted for the frontend
pub mod output;
/// For each Step, reduce the MaybeSlots Timeline into a FinalSlots Timeline, where the next Step to pick is calculated anew after every handled Step
pub mod placer;
/// Preprocessing new Input Goals data to be Steps ready to be placed StepsToPlace
pub mod preprocess;
/// Generates Steps from Goals, giving each Step their [MECE part](https://en.wikipedia.org/wiki/MECE_principle) of the Goal MaybeSlots Timeline
pub mod step_generator;
