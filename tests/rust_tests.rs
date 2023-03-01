extern crate scheduler;
use simple_logger::SimpleLogger;
extern crate soft;
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
fn split_3() {
    let (actual_output, desired_output) = run_test("split-3");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn basic_1() {
    let (actual_output, desired_output) = run_test("basic-1");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn every_wednesday_1() {
    let (actual_output, desired_output) = run_test("every-wednesday-1");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn flex_duration_2() {
    let (actual_output, desired_output) = run_test("flex-duration-2");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn every_21_days() {
    let (actual_output, desired_output) = run_test("every-21-days");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn singleday_manygoals_1() {
    let (actual_output, desired_output) = run_test("singleday-manygoals-1");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn goals_dependency_2() {
    let (actual_output, desired_output) = run_test("goals-dependency-2");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn split_1() {
    let (actual_output, desired_output) = run_test("split-1");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn after_12() {
    let (actual_output, desired_output) = run_test("after-12");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn x_per_week_3() {
    let (actual_output, desired_output) = run_test("x-per-week-3");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn every_6_hours_2() {
    let (actual_output, desired_output) = run_test("every-6-hours-2");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn weekdays_1() {
    let (actual_output, desired_output) = run_test("weekdays-1");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn every_2_hours_overlap() {
    let (actual_output, desired_output) = run_test("every-2-hours-overlap");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn every_3_days() {
    let (actual_output, desired_output) = run_test("every-3-days");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn x_per_day_1() {
    let (actual_output, desired_output) = run_test("x-per-day-1");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn options_1() {
    let (actual_output, desired_output) = run_test("options-1");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn different_repetition_goals() {
    let (actual_output, desired_output) = run_test("different-repetition-goals");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn every_8_hours() {
    let (actual_output, desired_output) = run_test("every-8-hours");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn every_60_days() {
    let (actual_output, desired_output) = run_test("every-60-days");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn non_rep_bounds_multipledays_1() {
    let (actual_output, desired_output) = run_test("non-rep-bounds-multipledays-1");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn slot_reduced_1() {
    let (actual_output, desired_output) = run_test("slot-reduced-1");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn every_2_days() {
    let (actual_output, desired_output) = run_test("every-2-days");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn before_7() {
    let (actual_output, desired_output) = run_test("before-7");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn flex_duration_1() {
    let (actual_output, desired_output) = run_test("flex-duration-1");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn x_per_week_2() {
    let (actual_output, desired_output) = run_test("x-per-week-2");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn flex_repeat_2() {
    let (actual_output, desired_output) = run_test("flex-repeat-2");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn x_per_day_4() {
    let (actual_output, desired_output) = run_test("x-per-day-4");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn every_6_hours() {
    let (actual_output, desired_output) = run_test("every-6-hours");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn sleep_1() {
    let (actual_output, desired_output) = run_test("sleep-1");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn split_2() {
    let (actual_output, desired_output) = run_test("split-2");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn flex_repeat_1() {
    let (actual_output, desired_output) = run_test("flex-repeat-1");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn every_2_hours() {
    let (actual_output, desired_output) = run_test("every-2-hours");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn children_with_over_duration() {
    let (actual_output, desired_output) = run_test("children-with-over-duration");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn planned_goals_hierarchy() {
    let (actual_output, desired_output) = run_test("planned-goals-hierarchy");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn repetition_weekends_1() {
    let (actual_output, desired_output) = run_test("repetition-weekends-1");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn goals_hierarchy() {
    let (actual_output, desired_output) = run_test("goals-hierarchy");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn split_4() {
    let (actual_output, desired_output) = run_test("split-4");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn bug_215() {
    let (actual_output, desired_output) = run_test("bug-215");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn basic_2() {
    let (actual_output, desired_output) = run_test("basic-2");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn realistic_schedule_1() {
    let (actual_output, desired_output) = run_test("realistic-schedule-1");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn realistic_weekend_repetition_1() {
    let (actual_output, desired_output) = run_test("realistic-weekend-repetition-1");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn x_per_week_1() {
    let (actual_output, desired_output) = run_test("x-per-week-1");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn impossible_1() {
    let (actual_output, desired_output) = run_test("impossible-1");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn flex_repeat_3() {
    let (actual_output, desired_output) = run_test("flex-repeat-3");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn goals_dependency() {
    let (actual_output, desired_output) = run_test("goals-dependency");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn show_sleep() {
    let (actual_output, desired_output) = run_test("show-sleep");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn ui_test_case_without_start_or_deadline() {
    let (actual_output, desired_output) = run_test("ui-test-case-without-start-or-deadline");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn impossible_2() {
    let (actual_output, desired_output) = run_test("impossible-2");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn x_per_day_3() {
    let (actual_output, desired_output) = run_test("x-per-day-3");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn weekly_1() {
    let (actual_output, desired_output) = run_test("weekly-1");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn x_per_day_2() {
    let (actual_output, desired_output) = run_test("x-per-day-2");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn demo_test() {
    let (actual_output, desired_output) = run_test("demo-test");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn weekly_2() {
    let (actual_output, desired_output) = run_test("weekly-2");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn bug_236() {
    let (actual_output, desired_output) = run_test("bug-236");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn repetition_daily_bounds_1() {
    let (actual_output, desired_output) = run_test("repetition-daily-bounds-1");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn non_midnight_start_deadline() {
    let (actual_output, desired_output) = run_test("non-midnight-start-deadline");
    assert_eq!(actual_output, desired_output);
}
#[test]
fn repetition_daily_1() {
    let (actual_output, desired_output) = run_test("repetition-daily-1");
    assert_eq!(actual_output, desired_output);
}
