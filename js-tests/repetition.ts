import { schedule } from "../js-api/scheduler.js";
import {
  assertEquals,
  assertThrows,
} from "https://deno.land/std@0.141.0/testing/asserts.ts";

Deno.test("can duplicate with daily repetition", () => {
  assertEquals(
    schedule({
      "startDate": "2022-01-01T00:00:00",
      "endDate": "2022-01-04T00:00:00",
      "goals": [
        {
          "id": 1,
          "title": "walk",
          "duration": 1,
          "repetition": "daily",
        },
      ],
    }),
     [
      {
        "taskid": 12,
        "goalid": 1,
        "title": "walk",
        "duration": 1,
        "start": "2022-01-03T00:00:00",
        "deadline": "2022-01-03T01:00:00",
      },
      {
        "taskid": 11,
        "goalid": 1,
        "title": "walk",
        "duration": 1,
        "start": "2022-01-02T00:00:00",
        "deadline": "2022-01-02T01:00:00",
      },
      {
        "taskid": 10,
        "goalid": 1,
        "title": "walk",
        "duration": 1,
        "start": "2022-01-01T00:00:00",
        "deadline": "2022-01-01T01:00:00",
      },
     ] 
  );
});

Deno.test("daily repetition with daily bounds", () => {
  assertEquals(
    schedule({
      "startDate": "2022-01-01T00:00:00",
      "endDate": "2022-01-04T00:00:00",
      "goals": [
        {
          "id": 1,
          "title": "walk",
          "duration": 1,
          "repetition": "daily",
          "start": "2022-01-01T00:00:00",
          "deadline": "2022-01-04T00:00:00",
          "after_time": 10,
          "before_time": 16
        },
      ],
    }),
    [
      {
        "taskid": 12,
        "goalid": 1,
        "title": "walk",
        "duration": 1,
        "start": "2022-01-03T10:00:00",
        "deadline": "2022-01-03T11:00:00",
      },
      {
        "taskid": 11,
        "goalid": 1,
        "title": "walk",
        "duration": 1,
        "start": "2022-01-02T10:00:00",
        "deadline": "2022-01-02T11:00:00",
      },
      {
        "taskid": 10,
        "goalid": 1,
        "title": "walk",
        "duration": 1,
        "start": "2022-01-01T10:00:00",
        "deadline": "2022-01-01T11:00:00",
      },
    ],
  );
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
