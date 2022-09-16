#[derive(Debug)]
pub enum Error {
    NoConfirmedDate(usize),
    CannotSplit,
}
