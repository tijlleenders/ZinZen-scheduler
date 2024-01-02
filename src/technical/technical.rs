use crate::models::goal::Goal;
use chrono::NaiveDateTime;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Input {
    start_date: NaiveDateTime,
    end_date: NaiveDateTime,
    goals: Vec<Goal>,
}

pub fn get_input_from_json<P: AsRef<Path>>(path: P) -> Result<Input, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let input = serde_json::from_reader(reader)?;
    Ok(input)
}

pub fn get_output_string_from_json<P: AsRef<Path>>(path: P) -> Result<String, serde_json::Error> {
    let file = File::open(path).expect("Error reading file");
    let reader = BufReader::new(file);
    // let output: FinalTasks = serde_json::from_reader(reader)?;
    let output = "".to_string();
    serde_json::to_string_pretty(&output)
}

pub fn write_to_file<P: AsRef<Path>>(path: P, actual_output: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(path)?;
    file.write_all(actual_output.as_bytes())?;
    Ok(())
}
