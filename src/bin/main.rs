extern crate scheduler;

use std::{fs, path::Path};

use chrono::NaiveDateTime;
use serde::Deserialize;
use serde_json::Value;

use scheduler::{
    models::{goal::Goal, task::TaskCompletedToday},
    run_scheduler,
};

fn main() {
    println!("Running!");
    let path = Path::new("./tests/jsons/stable/algorithm-challenge/input.json");
    let file = fs::File::open(path).expect("file should open read only");
    let json: Value = serde_json::from_reader(file).expect("file should be proper JSON");
    dbg!(&json);
    let input: Input = serde_json::from_value(json).unwrap();
    dbg!(&input);
    let _ = run_scheduler(
        input.start_date,
        input.end_date,
        &input.goals,
        input.tasks_completed_today,
    );
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct Input {
    start_date: NaiveDateTime,
    end_date: NaiveDateTime,
    goals: Vec<Goal>,
    tasks_completed_today: Vec<TaskCompletedToday>,
}
