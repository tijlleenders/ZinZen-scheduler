#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Budget {
    min: usize,
    diff: usize,
}
impl Budget {
    pub fn new(min: usize, max: usize) -> Self {
        Self {
            min,
            diff: max - min,
        }
    }
}
