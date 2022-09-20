import { schedule } from "../js-api/scheduler.js";
import { assertEquals, assertThrows } from "https://deno.land/std@0.141.0/testing/asserts.ts";
import basic_test_input from "./jsons/basic-test/input.json" assert {type: "json"};
import basic_test_output from "./jsons/basic-test/output.json" assert {type: "json"};
import every_wednesday_input from "./jsons/every-wednesday/input.json" assert {type: "json"};
import every_wednesday_output from "./jsons/every-wednesday/output.json" assert {type: "json"};
import non_rep_input from "./jsons/non-rep-bounds-multipledays/input.json" assert {type: "json"};
import non_rep_output from "./jsons/non-rep-bounds-multipledays/output.json" assert {type: "json"};
import realistic_input from "./jsons/realistic-schedule/input.json" assert {type: "json"};
import realistic_output from "./jsons/realistic-schedule/output.json" assert {type: "json"};
import repetition_input from "./jsons/repetition-daily/input.json" assert {type: "json"};
import repetition_output from "./jsons/repetition-daily/output.json" assert {type: "json"};
import rep_bounds_input from "./jsons/repetition-daily-bounds/input.json" assert {type: "json"};
import rep_bounds_output from "./jsons/repetition-daily-bounds/output.json" assert {type: "json"};
import singleday_input from "./jsons/singleday-manygoals/input.json" assert {type: "json"};
import singleday_output from "./jsons/singleday-manygoals/output.json" assert {type: "json"};
import split_tasks_simple_2_input from "./jsons/split-tasks-simple-2/input.json" assert {type: "json"}
import split_tasks_simple_2_output from "./jsons/split-tasks-simple-2/output.json" assert {type: "json"}


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

Deno.tes("new split task variant with 1hr duration", () => {
  assertEquals(
    schedule(split_tasks_simple_2_input, split_tasks_simple_2_output)
  );
});