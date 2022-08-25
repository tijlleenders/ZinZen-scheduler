import { schedule } from "../js-api/scheduler.js";
import { assertEquals } from "https://deno.land/std@0.141.0/testing/asserts.ts";

Deno.test("basic test from issue #3 (https://github.com/tijlleenders/ZinZen-scheduler/issues/3)", () => {
  assertEquals(
    schedule({
      "startDate": "2022-01-01T00:00:00",
      "endDate": "2022-01-02T00:00:00",
      "goals": [
        {
          "id": 1,
          "title": "shopping",
          "duration": 1,
          "start": "2022-01-01T10:00:00",
          "deadline": "2022-01-01T13:00:00",
        },
        {
          "id": 2,
          "title": "dentist",
          "duration": 1,
          "start": "2022-01-01T10:00:00",
          "deadline": "2022-01-01T11:00:00",
        },
        {
          "id": 3,
          "title": "exercise",
          "duration": 1,
          "start": "2022-01-01T10:00:00",
          "deadline": "2022-01-01T18:00:00",
        },
      ],
    }),
    [
      {
        "taskid": 20,
        "goalid": 2,
        "title": "dentist",
        "duration": 1,
        "start": "2022-01-01T10:00:00",
        "deadline": "2022-01-01T11:00:00",
      },
      {
        "taskid": 30,
        "goalid": 3,
        "title": "exercise",
        "duration": 1,
        "start": "2022-01-01T13:00:00",
        "deadline": "2022-01-01T14:00:00",
      },
      {
        "taskid": 10,
        "goalid": 1,
        "title": "shopping",
        "duration": 1,
        "start": "2022-01-01T11:00:00",
        "deadline": "2022-01-01T12:00:00",
      },
    ],
  );
});
