extern crate scheduler;
use scheduler::models::input::Input;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

/// To generate a flamegraph of the scheduler on your machine, follow the platform-specific instructions [here](https://github.com/flamegraph-rs/flamegraph).
fn main() {
    let path = Path::new("./tests/jsons/after-12/input.json");
    let path2 = Path::new("./tests/jsons/before-7/input.json");
    let paths = vec![path, path2];
    for path in paths {
        let input: Input = get_input_from_json(path).unwrap();
        let _output = scheduler::run_scheduler(input);
    }
}

pub fn get_input_from_json<P: AsRef<Path>>(path: P) -> Result<Input, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let input = serde_json::from_reader(reader)?;
    Ok(input)
}
