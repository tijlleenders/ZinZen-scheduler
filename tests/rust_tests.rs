extern crate scheduler;
mod common;
#[cfg(test)]
use pretty_assertions::assert_eq;
use scheduler::{Input, Output};
use std::path::Path;

fn run_test(directory: &str) -> (String, String) {
    let i = format!("./tests/jsons/{}/input.json", directory);
    let o = format!("./tests/jsons/{}/output.json", directory);
    let input_path = Path::new(&i[..]);
    let output_path = Path::new(&o[..]);
    let input: Input = common::get_input_from_json(input_path).unwrap();
    let desired_output: String = common::get_output_string_from_json(output_path).unwrap();
    let output: Vec<Output> = scheduler::run_scheduler(input);

    (
        serde_json::to_string_pretty(&output).unwrap(),
        desired_output,
    )
}

#[test]
fn basic_test_works() {
    let (actual_output, desired_output) = run_test("basic-1");
    assert_eq!(actual_output, desired_output);
}

#[test]
fn repetition_daily_works() {
    let (actual_output, desired_output) = run_test("repetition-daily-1");
    assert_eq!(actual_output, desired_output);
}

#[test]
fn repetition_daily_bounds_works() {
    let (actual_output, desired_output) = run_test("repetition-daily-bounds-1");
    assert_eq!(actual_output, desired_output);
}

#[test]
fn non_repetitive_bounds_multipledays_works() {
    let (actual_output, desired_output) = run_test("non-rep-bounds-multipledays-1");
    assert_eq!(actual_output, desired_output);
}

#[test]
fn single_day_many_goals_works() {
    let (actual_output, desired_output) = run_test("singleday-manygoals-1");
    assert_eq!(actual_output, desired_output);
}

#[test]
fn every_wednesday_works() {
    let (actual_output, desired_output) = run_test("every-wednesday-1");
    assert_eq!(actual_output, desired_output);
}

#[test]
fn realistic_schedule_works() {
    let (actual_output, desired_output) = run_test("realistic-schedule-1");
    assert_eq!(actual_output, desired_output);
}

#[test]
fn sleep() {
    let (actual_output, desired_output) = run_test("sleep-1");
    assert_eq!(actual_output, desired_output);
}

#[test]
fn split_1_works() {
    let (actual_output, desired_output) = run_test("split-1");
    assert_eq!(actual_output, desired_output);
}

#[test]
fn split_2_works() {
    let (actual_output, desired_output) = run_test("split-2");
    assert_eq!(actual_output, desired_output);
}

#[test]
fn split_3_works() {
    let (actual_output, desired_output) = run_test("split-3");
}

#[test]
fn split_4_works() {
    let (actual_output, desired_output) = run_test("split-4");
    assert_eq!(actual_output, desired_output);
}

#[test]
fn repetition_weekends_1_works() {
    let (actual_output, desired_output) = run_test("repetition-weekends-1");
    assert_eq!(actual_output, desired_output);
}

#[test]
fn realistic_weekend_repetition_1_works() {
    let (actual_output, desired_output) = run_test("realistic-weekend-repetition-1");
    assert_eq!(actual_output, desired_output);
}

#[test]
fn every_3_days_works() {
    let (actual_output, desired_output) = run_test("every-3-days");
    assert_eq!(actual_output, desired_output);
}

#[test]
fn weekly_works() {
    let (actual_output, desired_output) = run_test("weekly-1");
    assert_eq!(actual_output, desired_output);
}
