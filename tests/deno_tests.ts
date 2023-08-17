import { schedule } from "../js-api/scheduler.js";
import { assertEquals, assertThrows } from "https://deno.land/std@0.141.0/testing/asserts.ts";
import {existsSync} from "https://deno.land/std/fs/mod.ts";

const testFolder = './tests/jsons/stable/';

const getFiles = (directory: string): [string, string] => {
  const dir = testFolder + directory;  
  const inputFile = Deno.readTextFileSync(dir + '/input.json');
  const outputFile = Deno.readTextFileSync(dir + '/observed.json');
  return [inputFile, outputFile];
};

const tests = Deno.readDirSync(testFolder);
for (const dirEntry of tests) {
  if( dirEntry.name === "benchmark-tests") {
    continue;
  }
  const input_path =testFolder+`${dirEntry.name}`+"/input.json";
  const output_path =testFolder+`${dirEntry.name}`+"/observed.json";
  
  if (dirEntry.isDirectory ) {
    if ( existsSync(input_path) && existsSync(output_path)){
    const entry = `${dirEntry.name}`;
    Deno.test(`${entry}`, () => {
        const [inputFile, outputFile] = getFiles(`${entry}`);
        assertEquals(
          schedule(JSON.parse(inputFile)),JSON.parse(outputFile));
      });
    }
    else {
      console.log('%cWARN Empty directory : {'+`${dirEntry.name}`+'} Or one of input.json & observed.json not exist ', 'background: #222; color: #bada55')
    }
  
  }
}

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