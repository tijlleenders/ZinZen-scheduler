import { schedule } from "../js-api/scheduler.js";
import { assertEquals } from "https://deno.land/std@0.141.0/testing/asserts.ts";
import input from "./realistic-schedule/input.json" assert {type: "json"};
import output from "./realistic-schedule/output.json" assert {type: "json"};

Deno.test("realistic schedule", () => {
  assertEquals(
      schedule(input),output);
});
