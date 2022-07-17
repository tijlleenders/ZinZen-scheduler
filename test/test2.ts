// // THESE TESTS ARE MEANT TO BE RUN LOCALLY ON A USER'S MACHINE  //
//
// // deno-lint-ignore-file no-explicit-any
// import { assertEquals, assert } from "https://deno.land/std@0.141.0/testing/asserts.ts";
// import { loadAPI, Goal, jsDateToDateTime, durationFromHours } from "../ts/api.ts";
//
// // Load the API and the JSON
// const [API, json] = await Promise.all([loadAPI(`file:///${Deno.cwd()}/ts/scheduler.wasm`), Deno.readTextFile("test/goals.json")]);
//
// // Process serialized Goals into API compatible Goals
// const goalsRaw = JSON.parse(json);
// const goals: Goal[] = goalsRaw.map((goal: any) => {
// 	goal.task_duration = durationFromHours(goal.task_duration);
// 	goal.start = jsDateToDateTime(new Date(goal.start));
// 	goal.deadline = jsDateToDateTime(new Date(goal.deadline));
//
// 	return goal
// });
//
// // Define schedule boundaries
// const start = new Date('June 1, 2022 00:00:00');
// const finish = new Date('June 2, 2022 00:00:00');
//
// Deno.test("Test ProcessTaskCount", () => {
// 	const taskCounts = API.processTaskCount(goals, start, finish);
// 	console.log(taskCounts);
//
// 	taskCounts.forEach((task) => {
// 		assertEquals(task, 1);
// 	});
// })
//
// Deno.test("Test API.GenerateSchedule", () => {
// 	const schedule = API.generateSchedule(goals, start, finish).sort((a, b) => a.flexibility - b.flexibility);
// 	console.log(schedule);
//
// 	schedule.forEach(task => {
// 		// Minimum flexibility is 1
// 		assert(task.flexibility >= 1);
// 	});
// })