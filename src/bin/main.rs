extern crate scheduler;
use scheduler::{FinalOutput, Input};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
fn main() {
    let path = Path::new("./tests/jsons/realistic-schedule-1/input.json");
    let input: Input = get_input_from_json(path).unwrap();
    let output = scheduler::run_scheduler(input);
}

pub fn get_input_from_json<P: AsRef<Path>>(path: P) -> Result<Input, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let input = serde_json::from_reader(reader)?;
    Ok(input)
}
