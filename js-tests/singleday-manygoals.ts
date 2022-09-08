import { schedule } from "../js-api/scheduler.js";
import { assertEquals } from "https://deno.land/std@0.141.0/testing/asserts.ts";
import input from "./singleday-manygoals/input.json" assert {type: "json"};
import output from "./singleday-manygoals/output.json" assert {type: "json"};

Deno.test("single day many goals", () => {
  assertEquals(
    schedule(input),output);
});
