import { loadAPI } from "../ts/api.ts";

// Load the API
const API = loadAPI("file:///home/sokoro/Documents/dev/ZinZen-scheduler/ts/scheduler.wasm");

// Load JSON
const json = Deno.readTextFile("test/goals.json");

Promise.all([API, json]).then(([API, json]) => {
	const goals = JSON.parse(json);

	const taskCounts = API.processTaskCount(goals, BigInt(24 * 7 * 2));
	console.log(taskCounts);

	const schedule = API.generateSchedule(goals, [2019, 150, 0, 0, 0, 0], [2019, 170, 0, 0, 0, 0]).sort((a, b) => a.flexibility - b.flexibility);

	for (const task of schedule) {
		console.log(task);
	}
});
