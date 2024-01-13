use serde::Deserialize;

//Can a budget have other budgets?

//Budget per day/week in one budget?
//   NO, separate list of budget per period (each day and each week) - per goal-id
//remove datetime completely? YES

// check with budget should be on the whole range of the block, taking into account the min_block

//activity has budgets it belongs to, placer checks each budget ...
//   NO, flex should do that and remove any that are not allowed
// ... is that enough ... or do we need an extra check on placing?

#[derive(Debug, Clone, Deserialize)]
pub struct Budget {
    pub calendar_start_index: usize,
    pub calendar_end_index: usize,
    pub scheduled: usize,
    pub min_scheduled: usize,
    pub max_scheduled: usize,
}
