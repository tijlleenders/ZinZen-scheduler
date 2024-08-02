use crate::models::goal::Goal;
use crate::models::task::TaskCompletedToday;
use chrono::NaiveDateTime;
use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use crate::models::goal::Slot;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Input {
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub goals: Vec<Goal>,
    pub tasks_completed_today: Vec<TaskCompletedToday>,
    pub global_not_on: Option<Vec<Slot>>,
}

pub fn get_input_from_json<P: AsRef<Path>>(path: P) -> Result<Input, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let input= serde_json::from_reader(reader)?;
    Ok(input)
}

pub fn get_output_string_from_json<P: AsRef<Path>>(path: P) -> String {
    println!("get_output_string_from_json\n");
    fs::read_to_string(path).unwrap()
}

pub fn write_to_file<P: AsRef<Path>>(path: P, output: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(path)?;
    file.write_all(output.as_bytes())?;
    Ok(())
}
