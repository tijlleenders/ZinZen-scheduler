import { schedule } from "../js-api/scheduler.js";
import { assertEquals } from "https://deno.land/std@0.141.0/testing/asserts.ts";
import input from "./basic-test/input.json" assert {type: "json"};
import output from "./basic-test/output.json" assert {type: "json"};

Deno.test("basic test from issue #3 (https://github.com/tijlleenders/ZinZen-scheduler/issues/3)", () => {
  assertEquals(
    schedule(input),output);
});
