use chrono::{NaiveDate, NaiveDateTime};
use serde::Deserialize;
use serde_json::{self, Value};
use std::{fs, path::Path};
extern crate scheduler;
use scheduler::{
    models::{calendar::Calendar, goal::Goal},
    services::activity_placer,
};
fn main() {
    println!("Running!");
    let path = Path::new("./tests/jsons/stable/algorithm-challenge/input.json");
    let file = fs::File::open(path).expect("file should open read only");
    let json: Value = serde_json::from_reader(file).expect("file should be proper JSON");
    dbg!(&json);
    let input: Input = serde_json::from_value(json).unwrap();
    dbg!(&input);
    let calendar = Calendar::new(input.start_date, input.end_date);
    dbg!(&calendar);
    let activities =
        scheduler::services::activity_generator::generate_activities(&calendar, input.goals);
    let _output = activity_placer::place(calendar, activities);
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct Input {
    start_date: NaiveDateTime,
    end_date: NaiveDateTime,
    goals: Vec<Goal>,
}
