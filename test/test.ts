import { loadAPI } from "../ts/api.ts";

// Load the API
const wasm = await Deno.readFile("test/scheduler.wasm");
const API = loadAPI(wasm);

// Load JSON
const json = Deno.readTextFile("test/test.json");

Promise.all([API, json]).then(([API, json]) => {
	const goals = JSON.parse(json);
	const tasks = API.processTaskCount(goals, BigInt(24 * 7 * 2));

	console.log(tasks);
});
