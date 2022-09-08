import { schedule } from "../js-api/scheduler.js";
import { assertEquals } from "https://deno.land/std@0.141.0/testing/asserts.ts";
import input from "./non-rep-bounds-multipledays/input.json" assert {type: "json"};

import output from "./non-rep-bounds-multipledays/output.json" assert {type: "json"};
Deno.test("non repetitive spanning multiple days with time bound", () => {
  assertEquals(
    schedule(input),output);
});
