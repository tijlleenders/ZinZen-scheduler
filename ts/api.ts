// A span of time with nanosecond precision. Each Duration is composed of a whole number of seconds and a fractional part represented in nanoseconds.
type Duration = [number, number];

// Combined date and time
// Contains Year, Ordinal Day, Hour, Minute, Second, Nanosecond
type DateTime = [number, number, number, number, number, number];

// The Goal interface
interface Goal {
	id: number,
	description: string,
	task_duration: Duration,
	interval: Duration,
	time_constraint: null | DateTime,
	location_constraint: null | number
}

// A goal id is just a number
type GoalID = number;

// A simple API class
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

	public getTaskCount(goals: [Goal], durationInHours: BigInt): [[number, GoalID]] {
		// Encode data
		const string = JSON.stringify(goals);
		const data = this.textEncoder.encode(string);

		// Send data
		const target = this.getIPCView(data.length);
		target.set(data);

		// Process
		const offset = (this.instance.exports.taskCount as CallableFunction)(json.length, durationInHours) as number;
		const buffer = this.getIPCView(offset);
		const readString = this.textDecoder.decode(buffer);

		return JSON.parse(readString)
	}

	public getIPCView(offset: number): Uint8Array {
		return new Uint8Array(this.wasmMemory.buffer, this.ipcStart, offset);
	}
}

export async function loadAPI(path: string): Promise<API> {
	// Build instance
	const { instance } = await WebAssembly.instantiateStreaming(await fetch(path), {
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

	// Build API
	const _wasmMemory: WebAssembly.Memory = instance.exports.memory as WebAssembly.Memory;
	const _ipcStart = (instance.exports.IPC_BUFFER as WebAssembly.Global).value as number;
	const api = new API(instance, _ipcStart, _wasmMemory);

	return api
}
