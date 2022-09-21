#[derive(Debug)]
pub enum Error {
    NoConfirmedDate(String, usize),
    CannotSplit,
}
