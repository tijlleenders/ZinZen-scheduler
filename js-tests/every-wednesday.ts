import { schedule } from "../js-api/scheduler.js";
import { assertEquals } from "https://deno.land/std@0.141.0/testing/asserts.ts";
import input from "./every-wednesday/input.json" assert {type: "json"};
import output from "./every-wednesday/output.json" assert {type: "json"};

Deno.test("every wednesday", () => {
  assertEquals(
      schedule(input),output);
});
