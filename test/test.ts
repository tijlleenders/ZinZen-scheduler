import { loadAPI } from "../ts/api.ts";

// Load the API
const API = loadAPI("file:///home/sokoro/Documents/dev/ZinZen-scheduler/test/scheduler.wasm");

// Load JSON
const json = Deno.readTextFile("test/test.json");

Promise.all([API, json]).then(([API, json]) => {
	const goals = JSON.parse(json);

	const tasks = API.processTaskCount(goals, BigInt(24 * 7 * 2));
	console.log(tasks);

	const schedule = API.generateSchedule(goals, [2019, 120, 0, 0, 0, 0], [2019, 130, 0, 0, 0, 0]);
	console.log(schedule);
});
