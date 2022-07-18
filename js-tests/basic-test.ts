import {schedule} from "../js-api/scheduler.js"
import {assertEquals} from "https://deno.land/std@0.141.0/testing/asserts.ts";

Deno.test("basic test from issue #3 (https://github.com/tijlleenders/ZinZen-scheduler/issues/3)", () => {
    assertEquals(
        schedule({
            "startDate": "2022-01-01T00:00:00Z",
            "endDate": "2022-01-02T00:00:00Z",
            "goals": [
                {
                    "id": 1,
                    "title": "shopping",
                    "duration": 1,
                    "start": "2022-01-01T10:00:00Z",
                    "deadline": "2022-01-01T13:00:00Z"
                },
                {
                    "id": 2,
                    "title": "dentist",
                    "duration": 1,
                    "start": "2022-01-01T10:00:00Z",
                    "deadline": "2022-01-01T11:00:00Z"
                },
                {
                    "id": 3,
                    "title": "exercise",
                    "duration": 1,
                    "start": "2022-01-01T10:00:00Z",
                    "deadline": "2022-01-01T18:00:00Z"
                }
            ]
        }),
        {
            "tasks": [
                {
                    "id": 0,
                    "goal_id": 1,
                    "duration_to_schedule": 0,
                    "duration_scheduled": 1,
                    "status": "SCHEDULED"
                },
                {
                    "id": 1,
                    "goal_id": 2,
                    "duration_to_schedule": 0,
                    "duration_scheduled": 1,
                    "status": "SCHEDULED"
                },
                {
                    "id": 2,
                    "goal_id": 3,
                    "duration_to_schedule": 0,
                    "duration_scheduled": 1,
                    "status": "SCHEDULED"
                }
            ],
            "slots": [
                {
                    "task_id": 0,
                    "start": 11,
                    "end": 12
                },
                {
                    "task_id": 1,
                    "start": 10,
                    "end": 11
                },
                {
                    "task_id": 2,
                    "start": 13,
                    "end": 14
                }
            ]
        }
    )
});
