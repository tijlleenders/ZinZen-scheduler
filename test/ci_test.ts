import { loadAPI } from "../ts/api.ts";
//import { assertEquals as _assert } from "https://deno.land/std@0.140.0/testing/asserts.ts";

// Load the API
const API = loadAPI("https://github.com/tijlleenders/ZinZen-scheduler/raw/main/ts/scheduler.wasm");

// Load JSON
const json = Deno.readTextFile("test/goals.json");

Deno.test("API.processTaskCount", () => {
	Promise.all([API, json]).then(([API, json]) => {
		console.log("API:", API.generateSchedule)
		console.log("Output:", json)

		// TODO: Insert assertions here
	}).catch(console.error);
});
