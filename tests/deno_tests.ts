import { schedule } from "../js-api/scheduler.js";
import { assertEquals, assertThrows } from "https://deno.land/std@0.141.0/testing/asserts.ts";

const getFiles = (directory: string): [string, string] => {
  const dir = './tests/jsons/' + directory;
  const inputFile = Deno.readTextFileSync(dir + '/input.json');
  const outputFile = Deno.readTextFileSync(dir + '/output.json');
  return [inputFile, outputFile];
};
import goals_dependency_input from "./jsons/goals-dependency/input.json" assert {type: "json"};
import goals_dependency_output from "./jsons/goals-dependency/output.json" assert {type: "json"};

Deno.test("basic test from issue #3 (https://github.com/tijlleenders/ZinZen-scheduler/issues/3)", () => {
  const [inputFile, outputFile] = getFiles('basic-1');
  assertEquals(
    schedule(JSON.parse(inputFile)),JSON.parse(outputFile));
});

Deno.test("every wednesday", () => {
  const [inputFile, outputFile] = getFiles('every-wednesday-1');
  assertEquals(
    schedule(JSON.parse(inputFile)),JSON.parse(outputFile));
});

Deno.test("non repetitive spanning multiple days with time bound", () => {
  const [inputFile, outputFile] = getFiles('non-rep-bounds-multipledays-1');
  assertEquals(
    schedule(JSON.parse(inputFile)),JSON.parse(outputFile));
});

Deno.test("realistic schedule", () => {
  const [inputFile, outputFile] = getFiles('realistic-schedule-1');
  assertEquals(
    schedule(JSON.parse(inputFile)),JSON.parse(outputFile));
});

Deno.test("can duplicate with daily repetition", () => {
  const [inputFile, outputFile] = getFiles('repetition-daily-1');
  assertEquals(
    schedule(JSON.parse(inputFile)),JSON.parse(outputFile));
});

Deno.test("daily repetition with daily bounds", () => {
  const [inputFile, outputFile] = getFiles('repetition-daily-bounds-1');
  assertEquals(
    schedule(JSON.parse(inputFile)),JSON.parse(outputFile));
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

Deno.test("single day many goals", () => {
  const [inputFile, outputFile] = getFiles('singleday-manygoals-1');
  assertEquals(
    schedule(JSON.parse(inputFile)),JSON.parse(outputFile));
});

Deno.test("new split task variant with 1hr duration", () => {
  const [inputFile, outputFile] = getFiles('split-1');
  assertEquals(
    schedule(JSON.parse(inputFile)),JSON.parse(outputFile));
});

// test for splitting tasks
Deno.test("single day with split tasks", () => {
  const [inputFile, outputFile] = getFiles('split-2');
  assertEquals(
    schedule(JSON.parse(inputFile)),JSON.parse(outputFile));
});

// test for splitting tasks variant
Deno.test("single day with three goals to be split", () => {
  const [inputFile, outputFile] = getFiles('split-3');
    assertEquals(
      schedule(JSON.parse(inputFile)),JSON.parse(outputFile));
  });

// test for splitting tasks third variant
Deno.test("single day goals with same flexibility", () => {
  const [inputFile, outputFile] = getFiles('split-4');
    assertEquals(
      schedule(JSON.parse(inputFile)),JSON.parse(outputFile));
  });

// test for single task repeating on weekends
Deno.test("repetition weekend simple", () => {
  const [inputFile, outputFile] = getFiles('repetition-weekends-1');
    assertEquals(
      schedule(JSON.parse(inputFile)),JSON.parse(outputFile));
  });

Deno.test("realistic weekend repetition", () => {
  const [inputFile, outputFile] = getFiles('realistic-weekend-repetition-1');
    assertEquals(
      schedule(JSON.parse(inputFile)),JSON.parse(outputFile));
  });

Deno.test("every 3 days", () => {
  const [inputFile, outputFile] = getFiles('every-3-days');
    assertEquals(
      schedule(JSON.parse(inputFile)),JSON.parse(outputFile));
  });


Deno.test("weekly1", () => {
  const [inputFile, outputFile] = getFiles('weekly-1');
    assertEquals(
      schedule(JSON.parse(inputFile)),JSON.parse(outputFile));
  });

Deno.test("weekly2", () => {
  const [inputFile, outputFile] = getFiles('weekly-2');
    assertEquals(
      schedule(JSON.parse(inputFile)),JSON.parse(outputFile));
  });

Deno.test("every 2 days", () => {
  const [inputFile, outputFile] = getFiles('every-2-days');
    assertEquals(
      schedule(JSON.parse(inputFile)),JSON.parse(outputFile));
  });

Deno.test("every 60 days", () => {
  const [inputFile, outputFile] = getFiles('every-60-days');
    assertEquals(
      schedule(JSON.parse(inputFile)),JSON.parse(outputFile));
  });

Deno.test("after 12", () => {
  const [inputFile, outputFile] = getFiles('after-12');
    assertEquals(
      schedule(JSON.parse(inputFile)),JSON.parse(outputFile));
  });

Deno.test("before 7", () => {
  const [inputFile, outputFile] = getFiles('before-7');
    assertEquals(
      schedule(JSON.parse(inputFile)),JSON.parse(outputFile));
  });

Deno.test("ui without start or deadline", () => {
  const [inputFile, outputFile] = getFiles('ui-test-case-without-start-or-deadline');
    assertEquals(
      schedule(JSON.parse(inputFile)),JSON.parse(outputFile));
  });

Deno.test("slot reduced by other task", () => {
  const [inputFile, outputFile] = getFiles('slot-reduced-1');
    assertEquals(
      schedule(JSON.parse(inputFile)),JSON.parse(outputFile));
  });

Deno.test("x per day 1", () => {
  const [inputFile, outputFile] = getFiles('x-per-day-1');
    assertEquals(
      schedule(JSON.parse(inputFile)),JSON.parse(outputFile));
  });
  

Deno.test("x per day 2", () => {
  const [inputFile, outputFile] = getFiles('x-per-day-2');
    assertEquals(
      schedule(JSON.parse(inputFile)),JSON.parse(outputFile));
  });

Deno.test("x per day 3", () => {
  const [inputFile, outputFile] = getFiles('x-per-day-3');
    assertEquals(
      schedule(JSON.parse(inputFile)),JSON.parse(outputFile));
  });

Deno.test("every 2 hours", () => {
  const [inputFile, outputFile] = getFiles('every-2-hours');
    assertEquals(
      schedule(JSON.parse(inputFile)),JSON.parse(outputFile));
  });

Deno.test("every 8 hours", () => {
  const [inputFile, outputFile] = getFiles('every-8-hours');
    assertEquals(
      schedule(JSON.parse(inputFile)),JSON.parse(outputFile));
  });

Deno.test("every 2 hours overlap", () => {
  const [inputFile, outputFile] = getFiles('every-2-hours-overlap');
    assertEquals(
      schedule(JSON.parse(inputFile)),JSON.parse(outputFile));
  });

Deno.test("every 6 hours", () => {
  const [inputFile, outputFile] = getFiles('every-6-hours');
    assertEquals(
      schedule(JSON.parse(inputFile)),JSON.parse(outputFile));
  });

Deno.test("impossible 1", () => {
  const [inputFile, outputFile] = getFiles('impossible-1');
    assertEquals(
      schedule(JSON.parse(inputFile)),JSON.parse(outputFile));
  });

Deno.test("impossible 2", () => {
  const [inputFile, outputFile] = getFiles('impossible-2');
    assertEquals(
      schedule(JSON.parse(inputFile)),JSON.parse(outputFile));
  });

Deno.test("non midnight start deadline", () => {
  const [inputFile, outputFile] = getFiles('non-midnight-start-deadline');
    assertEquals(
      schedule(JSON.parse(inputFile)),JSON.parse(outputFile));
  });

Deno.test("every 6 hours 2", () => {
  const [inputFile, outputFile] = getFiles('every-6-hours-2');
    assertEquals(
      schedule(JSON.parse(inputFile)),JSON.parse(outputFile));
  });

Deno.test("flex repeat 1", () => {
  const [inputFile, outputFile] = getFiles('flex-repeat-1');
    assertEquals(
      schedule(JSON.parse(inputFile)),JSON.parse(outputFile));
  });

Deno.test("flex repeat 2", () => {
  const [inputFile, outputFile] = getFiles('flex-repeat-2');
    assertEquals(
      schedule(JSON.parse(inputFile)),JSON.parse(outputFile));
  });

Deno.test("flex repeat 3", () => {
  const [inputFile, outputFile] = getFiles('flex-repeat-3');
    assertEquals(
      schedule(JSON.parse(inputFile)),JSON.parse(outputFile));
  });

Deno.test("flex duration 1", () => {
  const [inputFile, outputFile] = getFiles('flex-duration-1');
    assertEquals(
      schedule(JSON.parse(inputFile)),JSON.parse(outputFile));
  });

Deno.test("flex duration 2", () => {
  const [inputFile, outputFile] = getFiles('flex-duration-1');
    assertEquals(
      schedule(JSON.parse(inputFile)),JSON.parse(outputFile));
  });

Deno.test("options 1", () => {
  const [inputFile, outputFile] = getFiles('options-1');
    assertEquals(
      schedule(JSON.parse(inputFile)),JSON.parse(outputFile));
  });
  
Deno.test("goals hierarchy", () => {
  const [inputFile, outputFile] = getFiles('goals-hierarchy');
    assertEquals(
      schedule(JSON.parse(inputFile)),JSON.parse(outputFile));
  });
Deno.test("goals dependency", () => {
  const [inputFile, outputFile] = getFiles('goals-dependency');
    assertEquals(
      schedule(JSON.parse(inputFile)),JSON.parse(outputFile));
  });
Deno.test("bug 236", () => {
  const [inputFile, outputFile] = getFiles('bug-236');
    assertEquals(
      schedule(JSON.parse(inputFile)),JSON.parse(outputFile));
  });

Deno.test("planned goals hierarchy", () => {
  const [inputFile, outputFile] = getFiles('planned-goals-hierarchy');
    assertEquals(
      schedule(JSON.parse(inputFile)),JSON.parse(outputFile));
  });