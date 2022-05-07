const buffer = await Deno.readFile("test/scheduler.wasm");

// Compiled wasm module
const module = await WebAssembly.compile(buffer);

// The current WASM instance
const instance = await WebAssembly.instantiate(module, {
	env: {
		console_log(isString, ipcOffset) {
			if (isString) {
				const readResult = new Uint8Array(wasmMemory.buffer, ipcStart, ipcOffset);
				const decoder = new TextDecoder();
				const string = decoder.decode(readResult);

				console.log(string);
			} else {
				const readResult = new Uint8Array(wasmMemory.buffer, ipcStart, ipcOffset);
				console.log(readResult);
			}
		},
		exit(error_code, ipcOffset) {
			if (error_code != 0) {
				const readResult = new Uint8Array(wasmMemory.buffer, ipcStart, ipcOffset);
				const decoder = new TextDecoder();

				throw new Error(`[WASM_ERROR; ErrorCode:${error_code}] ${decoder.decode(readResult)}`);
			} else {
				console.info("Webassembly has prematurely finished execution, without errors")
			}
		}
	},
});

// Where the IPC buffer pointer starts at
const ipcStart = instance.exports.getDataPointer();

// Load JSON
const json = new Uint8Array(await Deno.readFile("test/test.json"));

// The wasm memory
const wasmMemory = instance.exports.memory;
const jsonTarget = new Uint8Array(wasmMemory.buffer, ipcStart, json.length);

jsonTarget.set(json);

// A simple entry point for debugging
instance.exports.preProcessGoals(json.length, BigInt(24 * 7 * 2));
