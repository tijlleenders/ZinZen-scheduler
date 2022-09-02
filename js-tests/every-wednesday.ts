import { schedule } from "../js-api/scheduler.js";
import { assertEquals } from "https://deno.land/std@0.141.0/testing/asserts.ts";

Deno.test("every wednesday", () => {
  assertEquals(
      schedule(
        {
          "startDate": "2022-09-01T00:00:00",
          "endDate": "2022-10-01T00:00:00",
          "goals": [
            {
              "id": 1,
              "title": "piano practice",
              "duration": 2,
              "repetition": "wednesdays",
              "after_time": 12,
              "before_time": 14
            }
          ]
        }
      ),
     [
      {
        "taskid": 10,
        "goalid": 1,
        "title": "piano practice",
        "duration": 2,
        "start": "2022-09-07T12:00:00",
        "deadline": "2022-09-07T14:00:00"
      },
      {
        "taskid": 13,
        "goalid": 1,
        "title": "piano practice",
        "duration": 2,
        "start": "2022-09-28T12:00:00",
        "deadline": "2022-09-28T14:00:00"
      },
      {
        "taskid": 12,
        "goalid": 1,
        "title": "piano practice",
        "duration": 2,
        "start": "2022-09-21T12:00:00",
        "deadline": "2022-09-21T14:00:00"
      },
      {
        "taskid": 11,
        "goalid": 1,
        "title": "piano practice",
        "duration": 2,
        "start": "2022-09-14T12:00:00",
        "deadline": "2022-09-14T14:00:00"
      }
    ]      
  );
});
