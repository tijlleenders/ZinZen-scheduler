// A span of time with nanosecond precision. Each Duration is composed of a whole number of seconds and a fractional part represented in nanoseconds.
export type Duration = [number, number];

export function durationFromHours(timeInHours: number): Duration {
	return [timeInHours * 3600, 0]
}

// Combined date and time
// Contains Year, Ordinal Day, Hour, Minute, Second, Nanosecond
export type DateTime = [number, number, number, number, number, number];

// Checks is a year is a leap year
function isLeapYear(year: number): boolean {
	return year % 4 == 0 && (year % 25 != 0 || year % 16 == 0)
}

export function jsDateToDateTime(date: Date): DateTime {
	// Simple substitutes
	const year = date.getFullYear();
	const hour = date.getHours();
	const minutes = date.getMinutes();
	const seconds = date.getSeconds();

	// Ordinal math is hard
	const DAYS_CUMULATIVE_COMMON_LEAP = [
		[0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334],
		[0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335],
	];

	const day = date.getDate();
	const ordinal = DAYS_CUMULATIVE_COMMON_LEAP[isLeapYear(year) ? 1 : 0][date.getMonth()] + day;

	return [year, ordinal, hour, minutes, seconds, 0]
}

// A schedule is just an array of tasks
type Schedule = [Task];

// The Goal interface
export interface Goal {
	id: number,
	description: string,
	task_duration: Duration,

	start: null | DateTime,
	deadline: null | DateTime,

	interval: null | Duration,
	location_constraint: null | number
}

// An interface that describes a Task
export interface Task {
	goal_id: number,
	start: DateTime,
	deadline: DateTime,
	flexibility: number,
}

// A goal id is just a number
export type GoalID = number;

// Contains data required to generate a schedule
export interface Plan {
	goals: [Goal],
	start: DateTime,
	deadline: DateTime
}

// The wrapper API class
export class API {
	private instance: WebAssembly.Instance;
	private textDecoder: TextDecoder;
	private textEncoder: TextEncoder;
	private ipcStart: number;
	private wasmMemory: WebAssembly.Memory;

	constructor(instance: WebAssembly.Instance, ipcStart: number, wasmMemory: WebAssembly.Memory) {
		this.instance = instance;
		this.textDecoder = new TextDecoder;
		this.textEncoder = new TextEncoder;
		this.ipcStart = ipcStart;
		this.wasmMemory = wasmMemory;
	}

	public processTaskCount(goals: [Goal], start: Date, deadline: Date): Map<GoalID, number> {
		// Encode data
		const string = JSON.stringify([goals, [jsDateToDateTime(start), jsDateToDateTime(deadline)]]);
		const data = this.textEncoder.encode(string);

		// Send data
		const target = this.getIPCView(data.length);
		target.set(data);

		// Process
		const offset = (this.instance.exports.processTaskCount as CallableFunction)(data.length) as number;
		const buffer = this.getIPCView(offset);
		const readString = this.textDecoder.decode(buffer);
		const iterator = (JSON.parse(readString) as [[number, GoalID]]).map(a => ([a[1], a[0]] as [number, number]));

		return new Map(iterator)
	}

	public generateSchedule(goals: [Goal], start: Date, deadline: Date): Schedule {
		// Serialize data
		const plan = { goals, start: jsDateToDateTime(start), deadline: jsDateToDateTime(deadline) };
		const string = JSON.stringify(plan);
		const bytes = this.textEncoder.encode(string);

		// Send data
		let view = this.getIPCView(bytes.length);
		view.set(bytes);

		// Call wasm function
		const result = (this.instance.exports.generateSchedule as CallableFunction)(view.length) as number;

		// Read result
		view = this.getIPCView(result);
		const resultString = this.textDecoder.decode(view);
		return JSON.parse(resultString)
	}

	public getIPCView(offset: number): Uint8Array {
		return new Uint8Array(this.wasmMemory.buffer, this.ipcStart, offset);
	}
}

// loads the api
export async function loadAPI(path: string): Promise<API> {
	// Build instance
	const response = await fetch(path);
	const buffer = await response.arrayBuffer();
	const obj = await WebAssembly.instantiate(buffer, {
		env: {
			console_log(isString: boolean, ipcOffset: number) {
				if (isString) {
					const readResult = new Uint8Array(_wasmMemory.buffer, _ipcStart, ipcOffset);
					const decoder = new TextDecoder();
					const string = decoder.decode(readResult);

					console.log(string);
				} else {
					const readResult = new Uint8Array(_wasmMemory.buffer, _ipcStart, ipcOffset);
					console.log(readResult);
				}
			},
			exit(error_code: number, ipcOffset: number) {
				if (error_code != 0) {
					const readResult = new Uint8Array(_wasmMemory.buffer, _ipcStart, ipcOffset);
					const decoder = new TextDecoder();

					throw new Error(`[WASM_ERROR; ErrorCode:${error_code}] ${decoder.decode(readResult)}`);
				} else {
					console.info("Webassembly has prematurely finished execution, without errors")
				}
			}
		},
	});
	const instance = obj.instance;

	// Build API
	const _wasmMemory: WebAssembly.Memory = instance.exports.memory as WebAssembly.Memory;
	const _ipcStart = (instance.exports.IPC_BUFFER as WebAssembly.Global).value as number;
	const api = new API(instance, _ipcStart, _wasmMemory);

	return api
}
