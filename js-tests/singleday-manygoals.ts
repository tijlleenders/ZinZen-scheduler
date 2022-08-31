import { schedule } from "../js-api/scheduler.js";
import { assertEquals } from "https://deno.land/std@0.141.0/testing/asserts.ts";

Deno.test("single day many goals", () => {
  assertEquals(
    schedule({
      "startDate": "2022-01-01T00:00:00",
      "endDate": "2022-01-02T00:00:00",
      "goals": [
        {
          "id": 1,
          "title": "shopping",
          "duration": 1,
          "start": "2022-01-01T00:00:00",
          "deadline": "2022-01-02T00:00:00",
          "after_time": 10,
          "before_time": 13
        },
        {
          "id": 2,
          "title": "dentist",
          "duration": 1,
          "start": "2022-01-01T00:00:00",
          "deadline": "2022-01-02T00:00:00",
          "after_time": 10,
          "before_time": 11
        },
        {
          "id": 3,
          "title": "exercise",
          "duration": 1,
          "start": "2022-01-01T00:00:00",
          "deadline": "2022-01-02T00:00:00",
          "after_time": 10,
          "before_time": 18
        },
        {
          "id": 4,
          "title": "meditate",
          "duration": 1,
          "start": "2022-01-01T00:00:00",
          "deadline": "2022-01-02T00:00:00",
          "after_time": 5,
          "before_time": 7 
        },
        {
          "id": 5,
          "title": "cook",
          "duration": 2,
          "start": "2022-01-01T00:00:00",
          "deadline": "2022-01-02T00:00:00",
          "after_time": 18,
          "before_time": 21 
        },
        {
          "id": 6,
          "title": "read",
          "duration": 1,
          "start": "2022-01-01T00:00:00",
          "deadline": "2022-01-02T00:00:00",
          "after_time": 20,
          "before_time": 22 
        },
        {
          "id": 7,
          "title": "meeting",
          "duration": 2,
          "start": "2022-01-01T00:00:00",
          "deadline": "2022-01-02T00:00:00",
          "after_time": 13,
          "before_time": 17 
        },
        {
          "id": 8,
          "title": "code",
          "duration": 1,
          "start": "2022-01-01T00:00:00",
          "deadline": "2022-01-02T00:00:00"
        }
    ]
          }),
     [
          {
            "taskid": 20,
            "goalid": 2,
            "title": "dentist",
            "duration": 1,
            "start": "2022-01-01T10:00:00",
            "deadline": "2022-01-01T11:00:00"
          },
          {
            "taskid": 80,
            "goalid": 8,
            "title": "code",
            "duration": 1,
            "start": "2022-01-01T00:00:00",
            "deadline": "2022-01-01T01:00:00"
          },
          {
            "taskid": 30,
            "goalid": 3,
            "title": "exercise",
            "duration": 1,
            "start": "2022-01-01T17:00:00",
            "deadline": "2022-01-01T18:00:00"
          },
          {
            "taskid": 70,
            "goalid": 7,
            "title": "meeting",
            "duration": 2,
            "start": "2022-01-01T13:00:00",
            "deadline": "2022-01-01T14:00:00"
          },
          {
            "taskid": 50,
            "goalid": 5,
            "title": "cook",
            "duration": 2,
            "start": "2022-01-01T18:00:00",
            "deadline": "2022-01-01T19:00:00"
          },
          {
            "taskid": 10,
            "goalid": 1,
            "title": "shopping",
            "duration": 1,
            "start": "2022-01-01T11:00:00",
            "deadline": "2022-01-01T12:00:00"
          },
          {
            "taskid": 60,
            "goalid": 6,
            "title": "read",
            "duration": 1,
            "start": "2022-01-01T20:00:00",
            "deadline": "2022-01-01T21:00:00"
          },
          {
            "taskid": 40,
            "goalid": 4,
            "title": "meditate",
            "duration": 1,
            "start": "2022-01-01T05:00:00",
            "deadline": "2022-01-01T06:00:00"
          }
    ]   
  );
});
