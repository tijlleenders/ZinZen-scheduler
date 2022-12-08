import { schedule } from "../js-api/scheduler.js";
import { assertEquals, assertThrows } from "https://deno.land/std@0.141.0/testing/asserts.ts";
import basic_test_input from "./jsons/basic-1/input.json" assert {type: "json"};
import basic_test_output from "./jsons/basic-1/output.json" assert {type: "json"};
import every_wednesday_input from "./jsons/every-wednesday-1/input.json" assert {type: "json"};
import every_wednesday_output from "./jsons/every-wednesday-1/output.json" assert {type: "json"};
import non_rep_input from "./jsons/non-rep-bounds-multipledays-1/input.json" assert {type: "json"};
import non_rep_output from "./jsons/non-rep-bounds-multipledays-1/output.json" assert {type: "json"};
import realistic_input from "./jsons/realistic-schedule-1/input.json" assert {type: "json"};
import realistic_output from "./jsons/realistic-schedule-1/output.json" assert {type: "json"};
import repetition_input from "./jsons/repetition-daily-1/input.json" assert {type: "json"};
import repetition_output from "./jsons/repetition-daily-1/output.json" assert {type: "json"};
import rep_bounds_input from "./jsons/repetition-daily-bounds-1/input.json" assert {type: "json"};
import rep_bounds_output from "./jsons/repetition-daily-bounds-1/output.json" assert {type: "json"};
import singleday_input from "./jsons/singleday-manygoals-1/input.json" assert {type: "json"};
import singleday_output from "./jsons/singleday-manygoals-1/output.json" assert {type: "json"};
import simple_split_task_input from "./jsons/split-1/input.json" assert {type: "json"};
import simple_split_task_output from "./jsons/split-1/output.json" assert {type: "json"};
import split_task_variant_input from "./jsons/split-2/input.json" assert {type: "json"};
import split_task_variant_output from "./jsons/split-2/output.json" assert {type: "json"};
import split_tasks_simple_2_input from "./jsons/split-3/input.json" assert {type: "json"}
import split_tasks_simple_2_output from "./jsons/split-3/output.json" assert {type: "json"}
import split_tasks_simple_3_input from "./jsons/split-4/input.json" assert {type: "json"}
import split_tasks_simple_3_output from "./jsons/split-4/output.json" assert {type: "json"}
import repetition_weekend_1_input from "./jsons/repetition-weekends-1/input.json" assert {type: "json"}
import repetition_weekend_1_output from "./jsons/repetition-weekends-1/output.json" assert {type: "json"}
import realistic_weekend_repetition_1_input from "./jsons/realistic-weekend-repetition-1/input.json" assert {type: "json"}
import realistic_weekend_repetition_1_output from "./jsons/realistic-weekend-repetition-1/output.json" assert {type: "json"}
import every_3_days_input from "./jsons/every-3-days/input.json" assert {type: "json"}
import every_3_days_output from "./jsons/every-3-days/output.json" assert {type: "json"}
import weekly1_input from "./jsons/weekly-1/input.json" assert {type: "json"}
import weekly1_output from "./jsons/weekly-1/output.json" assert {type: "json"}
import weekly2_input from "./jsons/weekly-2/input.json" assert {type: "json"}
import weekly2_output from "./jsons/weekly-2/output.json" assert {type: "json"}
import every_2_days_input from "./jsons/every-2-days/input.json" assert {type: "json"}
import every_2_days_output from "./jsons/every-2-days/output.json" assert {type: "json"}
import every_60_days_input from "./jsons/every-60-days/input.json" assert {type: "json"}
import every_60_days_output from "./jsons/every-60-days/output.json" assert {type: "json"}
import after_12_input from "./jsons/after-12/input.json" assert {type: "json"}
import after_12_output from "./jsons/after-12/output.json" assert {type: "json"}
import before_7_input from "./jsons/before-7/input.json" assert {type: "json"}
import before_7_output from "./jsons/before-7/output.json" assert {type: "json"}
import ui_test_case_without_start_or_deadline_input from "./jsons/ui-test-case-without-start-or-deadline/input.json" assert {type: "json"}
import ui_test_case_without_start_or_deadline_output from "./jsons/ui-test-case-without-start-or-deadline/output.json" assert {type: "json"}
import slot_reduced_1_input from "./jsons/slot-reduced-1/input.json" assert {type: "json"}
import slot_reduced_1_output from "./jsons/slot-reduced-1/output.json" assert {type: "json"}
import x_per_week_1_input from "./jsons/x-per-week-1/input.json" assert {type: "json"}
import x_per_week_1_output from "./jsons/x-per-week-1/output.json" assert {type: "json"}
import x_per_week_2_input from "./jsons/x-per-week-2/input.json" assert {type: "json"}
import x_per_week_2_output from "./jsons/x-per-week-2/output.json" assert {type: "json"}
import x_per_week_3_input from "./jsons/x-per-week-3/input.json" assert {type: "json"}
import x_per_week_3_output from "./jsons/x-per-week-3/output.json" assert {type: "json"}
import x_per_day_1_input from "./jsons/x-per-day-1/input.json" assert {type: "json"}
import x_per_day_1_output from "./jsons/x-per-day-1/output.json" assert {type: "json"}
import x_per_day_2_input from "./jsons/x-per-day-2/input.json" assert {type: "json"}
import x_per_day_2_output from "./jsons/x-per-day-2/output.json" assert {type: "json"}
import x_per_day_3_input from "./jsons/x-per-day-3/input.json" assert {type: "json"}
import x_per_day_3_output from "./jsons/x-per-day-3/output.json" assert {type: "json"}
import x_per_day_4_input from "./jsons/x-per-day-4/input.json" assert {type: "json"}
import x_per_day_4_output from "./jsons/x-per-day-4/output.json" assert {type: "json"}
import every_2_hours_input from "./jsons/every-2-hours/input.json" assert {type: "json"}
import every_2_hours_output from "./jsons/every-2-hours/output.json" assert {type: "json"}
import every_8_hours_input from "./jsons/every-8-hours/input.json" assert {type: "json"}
import every_8_hours_output from "./jsons/every-8-hours/output.json" assert {type: "json"}
import every_2_hours_overlap_input from "./jsons/every-2-hours-overlap/input.json" assert {type: "json"}
import every_2_hours_overlap_output from "./jsons/every-2-hours-overlap/output.json" assert {type: "json"}
import every_6_hours_input from "./jsons/every-6-hours/input.json" assert {type: "json"}
import every_6_hours_output from "./jsons/every-6-hours/output.json" assert {type: "json"}
import impossible_1_input from "./jsons/impossible-1/input.json" assert {type: "json"}
import impossible_1_output from "./jsons/impossible-1/output.json" assert {type: "json"}
import impossible_2_input from "./jsons/impossible-2/input.json" assert {type: "json"}
import impossible_2_output from "./jsons/impossible-2/output.json" assert {type: "json"}
import non_midnight_start_deadline_input from "./jsons/non-midnight-start-deadline/input.json" assert {type: "json"}
import non_midnight_start_deadline_output from "./jsons/non-midnight-start-deadline/output.json" assert {type: "json"}
import every_6_hours_2_input from "./jsons/every-6-hours-2/input.json" assert {type: "json"}
import every_6_hours_2_output from "./jsons/every-6-hours-2/output.json" assert {type: "json"}
import flex_repeat_1_input from "./jsons/flex-repeat-1/input.json" assert {type: "json"}
import flex_repeat_1_output from "./jsons/flex-repeat-1/output.json" assert {type: "json"}
import flex_repeat_2_input from "./jsons/flex-repeat-2/input.json" assert {type: "json"}
import flex_repeat_2_output from "./jsons/flex-repeat-2/output.json" assert {type: "json"}
import flex_repeat_3_input from "./jsons/flex-repeat-3/input.json" assert {type: "json"}
import flex_repeat_3_output from "./jsons/flex-repeat-3/output.json" assert {type: "json"}

Deno.test("basic test from issue #3 (https://github.com/tijlleenders/ZinZen-scheduler/issues/3)", () => {
  assertEquals(
    schedule(basic_test_input),basic_test_output);
});

Deno.test("every wednesday", () => {
  assertEquals(
      schedule(every_wednesday_input),every_wednesday_output);
});

Deno.test("non repetitive spanning multiple days with time bound", () => {
  assertEquals(
    schedule(non_rep_input),non_rep_output);
});

Deno.test("realistic schedule", () => {
  assertEquals(
      schedule(realistic_input),realistic_output);
});

Deno.test("can duplicate with daily repetition", () => {
  assertEquals(
    schedule(repetition_input),repetition_output);
});

Deno.test("daily repetition with daily bounds", () => {
  assertEquals(
    schedule(rep_bounds_input),rep_bounds_output);
});

// TODO should check the error for this test, see issue #26,
// if it is fixed please change this test to check the error
Deno.test("invalid repetition",() => {
  assertThrows(
    () =>
      schedule({
        "startDate": "2022-01-01",
        "endDate": "2022-01-02",
        "goals": [
          {
            "id": 1,
            "title": "shopping",
            "duration": 1,
            "start": "2022-01-01T10:00:00",
            "deadline": "2022-01-01T13:00:00",
            "repetition": "invalid-value-AAAAAA",
          },
        ],
      }),
  );
});

Deno.test("single day many goals", () => {
  assertEquals(
    schedule(singleday_input),singleday_output);
});

Deno.test("new split task variant with 1hr duration", () => {
  assertEquals(
    schedule(split_tasks_simple_2_input), split_tasks_simple_2_output)
  ;
});

// test for splitting tasks
Deno.test("single day with split tasks", () => {
  assertEquals(
    schedule(simple_split_task_input),simple_split_task_output);
});

// test for splitting tasks variant
Deno.test("single day with three goals to be split", () => {
  assertEquals(
    schedule(split_task_variant_input),split_task_variant_output);
});

// test for splitting tasks third variant
Deno.test("single day goals with same flexibility", () => {
  assertEquals(
    schedule(split_tasks_simple_3_input),split_tasks_simple_3_output);
});

// test for single task repeating on weekends
Deno.test("repetition weekend simple", () => {
  assertEquals(
    schedule(repetition_weekend_1_input),repetition_weekend_1_output);
});

Deno.test("realistic weekend repetition", () => {
  assertEquals(
    schedule(realistic_weekend_repetition_1_input),realistic_weekend_repetition_1_output);
});

Deno.test("every 3 days", () => {
  assertEquals(
    schedule(every_3_days_input),every_3_days_output);
});

Deno.test("weekly1", () => {
  assertEquals(
    schedule(weekly1_input),weekly1_output);
});

Deno.test("weekly2", () => {
  assertEquals(
    schedule(weekly2_input),weekly2_output);
});

Deno.test("every 2 days", () => {
  assertEquals(
    schedule(every_2_days_input),every_2_days_output);
});

Deno.test("every 60 days", () => {
  assertEquals(
    schedule(every_60_days_input),every_60_days_output);
});

Deno.test("after 12", () => {
  assertEquals(
    schedule(after_12_input),after_12_output);
});

Deno.test("before 7", () => {
  assertEquals(
    schedule(before_7_input),before_7_output);
});

Deno.test("ui without start or deadline", () => {
  assertEquals(
    schedule(before_7_input),before_7_output);
});

Deno.test("slot reduced by other task", () => {
  assertEquals(
    schedule(slot_reduced_1_input),slot_reduced_1_output);
});

Deno.test("x per day 1", () => {
  assertEquals(
    schedule(x_per_day_1_input),x_per_day_1_output);
});

Deno.test("x per day 2", () => {
  assertEquals(
    schedule(x_per_day_2_input),x_per_day_2_output);
});

Deno.test("x per day 3", () => {
  assertEquals(
    schedule(x_per_day_3_input),x_per_day_3_output);
});

Deno.test("x per day 1", () => {
  assertEquals(
    schedule(x_per_day_1_input),x_per_day_1_output);
});

Deno.test("x per day 2", () => {
  assertEquals(
    schedule(x_per_day_2_input),x_per_day_2_output);
});

Deno.test("x per day 3", () => {
  assertEquals(
    schedule(x_per_day_3_input),x_per_day_3_output);
});

Deno.test("every 2 hours", () => {
  assertEquals(
    schedule(every_2_hours_input),every_2_hours_output);
});

Deno.test("every 8 hours", () => {
  assertEquals(
    schedule(every_8_hours_input),every_8_hours_output);
});

Deno.test("every 2 hours overlap", () => {
  assertEquals(
    schedule(every_2_hours_overlap_input),every_2_hours_overlap_output);
});

Deno.test("every 6 hours", () => {
  assertEquals(
    schedule(every_6_hours_input),every_6_hours_output);
});

Deno.test("impossible 1", () => {
  assertEquals(
    schedule(impossible_1_input),impossible_1_output);
});

Deno.test("impossible 2", () => {
  assertEquals(
    schedule(impossible_2_input),impossible_2_output);
});

Deno.test("non midnight start deadline", () => {
  assertEquals(
    schedule(non_midnight_start_deadline_input),non_midnight_start_deadline_output);
});

Deno.test("every 6 hours 2", () => {
  assertEquals(
    schedule(every_6_hours_2_input),every_6_hours_2_output);
});

Deno.test("flex repeat 1", () => {
  assertEquals(
    schedule(flex_repeat_1_input),flex_repeat_1_output);
});

Deno.test("flex repeat 2", () => {
  assertEquals(
    schedule(flex_repeat_2_input),flex_repeat_2_output);
});

Deno.test("flex repeat 3", () => {
  assertEquals(
    schedule(flex_repeat_3_input),flex_repeat_3_output);
});