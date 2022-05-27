import { loadAPI } from "../ts/api.ts";
//import { assertEquals as _assert } from "https://deno.land/std@0.140.0/testing/asserts.ts";

Deno.test("API.processTaskCount", async () => {
	
	// Load the API
	const API = await loadAPI("https://github.com/tijlleenders/ZinZen-scheduler/raw/main/ts/scheduler.wasm");
	
	// Load JSON
	const json = Deno.readTextFile("test/goals.json");
	
	Promise.all([API, json]).then(([API, json]) => {
		console.log("API:", API.generateSchedule)
		console.log("Output:", json)

		// TODO: Insert assertions here
	}).catch(console.error);
});
