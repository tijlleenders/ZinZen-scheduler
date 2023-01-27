extern crate scheduler;
mod common;
#[cfg(test)]
use pretty_assertions::assert_eq;
use scheduler::{FinalOutput, Input};
use std::path::Path;

fn run_test(directory: &str) -> (String, String) {
    let i = format!("./tests/jsons/{}/input.json", directory);
    let o = format!("./tests/jsons/{}/output.json", directory);
    let input_path = Path::new(&i[..]);
    let output_path = Path::new(&o[..]);
    let input: Input = common::get_input_from_json(input_path).unwrap();
    let desired_output: String = common::get_output_string_from_json(output_path).unwrap();
    let output: FinalOutput = scheduler::run_scheduler(input);

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
    assert_eq!(actual_output, desired_output);
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
fn weekly1_works() {
    let (actual_output, desired_output) = run_test("weekly-1");
    assert_eq!(actual_output, desired_output);
}

#[test]
fn weekly2_works() {
    let (actual_output, desired_output) = run_test("weekly-2");
    assert_eq!(actual_output, desired_output);
}

#[test]
fn every_2_days_works() {
    let (actual_output, desired_output) = run_test("every-2-days");
    assert_eq!(actual_output, desired_output);
}

#[test]
fn every_60_days_works() {
    let (actual_output, desired_output) = run_test("every-60-days");
    assert_eq!(actual_output, desired_output);
}

#[test]
fn after_12_works() {
    let (actual_output, desired_output) = run_test("after-12");
    assert_eq!(actual_output, desired_output);
}

#[test]
fn before_7_works() {
    let (actual_output, desired_output) = run_test("before-7");
    assert_eq!(actual_output, desired_output);
}

#[test]
fn ui_test_case_without_start_or_deadline_works() {
    let (actual_output, desired_output) = run_test("ui-test-case-without-start-or-deadline");
    assert_eq!(actual_output, desired_output);
}

#[test]
fn slots_reduced_by_other_tasks_works() {
    let (actual_output, desired_output) = run_test("slot-reduced-1");
    assert_eq!(actual_output, desired_output);
}

#[test]
fn x_per_week_1_works() {
    let (actual_output, desired_output) = run_test("x-per-week-1");
    assert_eq!(actual_output, desired_output);
}

#[test]
fn x_per_week_2_works() {
    let (actual_output, desired_output) = run_test("x-per-week-2");
    assert_eq!(actual_output, desired_output);
}

#[test]
fn x_per_week_3_works() {
    let (actual_output, desired_output) = run_test("x-per-week-3");
    assert_eq!(actual_output, desired_output);
}

#[test]
fn x_per_day_1_works() {
    let (actual_output, desired_output) = run_test("x-per-day-1");
    assert_eq!(actual_output, desired_output);
}

#[test]
fn x_per_day_2_works() {
    let (actual_output, desired_output) = run_test("x-per-day-2");
    assert_eq!(actual_output, desired_output);
}

#[test]
fn x_per_day_3_works() {
    let (actual_output, desired_output) = run_test("x-per-day-3");
    assert_eq!(actual_output, desired_output);
}

#[test]
fn x_per_day_4_works() {
    let (actual_output, desired_output) = run_test("x-per-day-4");
    assert_eq!(actual_output, desired_output);
}

#[test]
fn every_2_hours_works() {
    let (actual_output, desired_output) = run_test("every-2-hours");
    assert_eq!(actual_output, desired_output);
}

#[test]
fn every_8_hours_works() {
    let (actual_output, desired_output) = run_test("every-8-hours");
    assert_eq!(actual_output, desired_output);
}

#[test]
fn every_2_hours_overlap_works() {
    let (actual_output, desired_output) = run_test("every-2-hours-overlap");
    assert_eq!(actual_output, desired_output);
}

#[test]
fn every_6_hours_works() {
    let (actual_output, desired_output) = run_test("every-6-hours");
    assert_eq!(actual_output, desired_output);
}

#[test]
fn impossible_1_works() {
    let (actual_output, desired_output) = run_test("impossible-1");
    assert_eq!(actual_output, desired_output);
}

#[test]
fn impossible_2_works() {
    let (actual_output, desired_output) = run_test("impossible-2");
    assert_eq!(actual_output, desired_output);
}

#[test]
fn non_midnight_start_deadline_works() {
    let (actual_output, desired_output) = run_test("non-midnight-start-deadline");
    assert_eq!(actual_output, desired_output);
}

#[test]
fn every_6_hours_2_works() {
    let (actual_output, desired_output) = run_test("every-6-hours-2");
    assert_eq!(actual_output, desired_output);
}

#[test]
fn flex_repeat_1_works() {
    let (actual_output, desired_output) = run_test("flex-repeat-1");
    assert_eq!(actual_output, desired_output);
}

#[test]
fn flex_repeat_2_works() {
    let (actual_output, desired_output) = run_test("flex-repeat-2");
    assert_eq!(actual_output, desired_output);
}

#[test]
fn flex_repeat_3_works() {
    let (actual_output, desired_output) = run_test("flex-repeat-3");
    assert_eq!(actual_output, desired_output);
}

#[test]
fn flex_duration_1_works() {
    let (actual_output, desired_output) = run_test("flex-duration-1");
    assert_eq!(actual_output, desired_output);
}

#[test]
fn flex_duration_2_works() {
    let (actual_output, desired_output) = run_test("flex-duration-2");
    assert_eq!(actual_output, desired_output);
}

#[test]
fn bug_215_works() {
    let (actual_output, desired_output) = run_test("bug-215");
    assert_eq!(actual_output, desired_output);
}

#[test]
fn options_1_works() {
    let (actual_output, desired_output) = run_test("options-1");
    assert_eq!(actual_output, desired_output);
}

#[test]
fn goals_dependency_works() {
    let (actual_output, desired_output) = run_test("goals-dependency");
    assert_eq!(actual_output, desired_output);
}

#[test]
fn goals_dependency_2_works() {
    let (actual_output, desired_output) = run_test("goals-dependency-2");
    assert_eq!(actual_output, desired_output);
}