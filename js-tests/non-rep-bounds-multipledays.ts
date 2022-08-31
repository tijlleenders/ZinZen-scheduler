import { schedule } from "../js-api/scheduler.js";
import { assertEquals } from "https://deno.land/std@0.141.0/testing/asserts.ts";

Deno.test("basic test from issue #3 (https://github.com/tijlleenders/ZinZen-scheduler/issues/3)", () => {
  assertEquals(
    schedule({
        "startDate": "2022-01-01T00:00:00",
        "endDate": "2022-01-06T00:00:00",
        "goals": [
          {
            "id": 1,
            "title": "home repair",
            "duration": 1,
            "start": "2022-01-01T00:00:00",
            "deadline": "2022-01-06T00:00:00",
            "after_time": 15,
            "before_time": 19 
          },
        ],
        }),
    [
      {
        "taskid": 10,
        "goalid": 1,
        "title": "home repair",
        "duration": 1,
        "start": "2022-01-01T15:00:00",
        "deadline": "2022-01-01T16:00:00",
      },
    ],
  );
});
