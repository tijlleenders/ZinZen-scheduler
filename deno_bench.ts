import { schedule } from "./js-api/scheduler.js";

Deno.bench("basic bench for issue #3 (https://github.com/tijlleenders/ZinZen-scheduler/issues/3)", () => {
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
      })
  });