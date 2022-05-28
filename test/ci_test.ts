import { loadAPI } from "../ts/api.ts";
//import { assertEquals as _assert } from "https://deno.land/std@0.140.0/testing/asserts.ts";

Deno.test("API.processTaskCount", async () => {
	
	// Load the API
	const API = loadAPI("https://github.com/tijlleenders/ZinZen-scheduler/raw/main/ts/scheduler.wasm");
	
	// Load JSON
	const jsonString = Deno.readTextFile("test/goals.json");
	
	await Promise.all([API, jsonString]).then(([API, jsonString]) => {
		console.log("input:", jsonString)
		console.log("API:", API.generateSchedule(JSON.parse(jsonString), new Date(), new Date() ))

		// TODO: Insert assertions here
	}).catch(console.error);
});
