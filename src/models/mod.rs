/// Keeps track of the min and max time allowed and scheduled per time period for a collection of Steps.
pub mod budget;
/// An aim or desired result someone wants to reach.  
pub mod goal;
/// The JSON Value object passed in to the scheduler.
pub mod input;
/// The JSON Value object representing the final Calendar.
pub mod output;
/// Contains the deserialization for the repetition attribute of a Goal.
pub mod repetition;
/// A period of time defined by a start datetime and an end datetime.
pub mod slot;
/// The logic of partitioning a Goal Timeline into [MECE parts](https://en.wikipedia.org/wiki/MECE_principle).
pub mod slots_iterator;
/// An Step for the realization of a Goal.
pub mod step;

pub mod timeline;
