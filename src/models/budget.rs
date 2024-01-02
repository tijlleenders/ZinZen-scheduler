#[derive(Debug)]
pub struct Budget {
    calendar_start_index: usize,
    calendar_end_index: usize,
    scheduled: usize,
    min_max_required: usize,
}
