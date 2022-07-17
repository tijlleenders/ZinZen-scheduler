import init, {schedule} from "../pkg/scheduler.js"

await init(Deno.readFile('./pkg/scheduler_bg.wasm'));
console.log("hi!");
schedule({a: "1"});