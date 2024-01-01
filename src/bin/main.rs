use serde::Deserialize;
use serde_json::{self, Value};
use std::{fs, path::Path};
extern crate scheduler;
use scheduler::models::goal::Goal;
fn main() {
    println!("Running!");
    let path = Path::new("./tests/jsons/stable/algorithm-challenge/input.json");
    let file = fs::File::open(path).expect("file should open read only");
    let json: Value = serde_json::from_reader(file).expect("file should be proper JSON");
    dbg!(&json);
    let input: Input = serde_json::from_value(json).unwrap();
    dbg!(&input);
    let _output = scheduler::run_scheduler(input.start_date, input.end_date);
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct Input {
    start_date: String,
    end_date: String,
    goals: Vec<Goal>,
}
