#[derive(Debug)]
pub struct Budget {
    calendar_start_index: usize,
    calendar_end_index: usize,
    scheduled: usize,
    minimum: usize,
    maximum: usize,
}
