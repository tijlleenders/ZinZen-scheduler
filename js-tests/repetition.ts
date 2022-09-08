import { schedule } from "../js-api/scheduler.js";
import {
  assertEquals,
  assertThrows,
} from "https://deno.land/std@0.141.0/testing/asserts.ts";
import input from "./repetition-daily/input.json" assert {type: "json"};
import output from "./repetition-daily/output.json" assert {type: "json"};
import input2 from "./repetition-daily-bounds/input.json" assert {type: "json"};
import output2 from "./repetition-daily-bounds/output.json" assert {type: "json"};

Deno.test("can duplicate with daily repetition", () => {
  assertEquals(
    schedule(input),output);
});

Deno.test("daily repetition with daily bounds", () => {
  assertEquals(
    schedule(input2),output2);
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
