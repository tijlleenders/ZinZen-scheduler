import fs from "fs";

// RAW wasm source
const buffer = fs.readFileSync("output.wasm");

// Compiled wasm module
const module = await WebAssembly.compile(buffer);

// The current WASM instance
const instance = await WebAssembly.instantiate(module, {
	env: {
		console_log(isString, dataOffset) {
			if (isString) {
				let readResult = new Uint8Array(wasmMemory.buffer, dataStart, dataOffset);
				let decoder = new TextDecoder();
				let string = decoder.decode(readResult);
				console.log(string);
			} else {
				let readResult = new Uint8Array(wasmMemory.buffer, dataStart, dataOffset);
				console.log(readResult);
			}
		},
		exit(error_code) {
			if (error_code != 0) {
				throw new Error(`[WASM_ERROR; ErrorCode:${error_code}]`);
			} else {
				console.info("Webassembly has prematurely finished execution, without errors")
			}
		}
	},
});

// Where the IPC buffer pointer starts at
const dataStart = instance.exports.getDataPointer();

// Load JSON
const json = new Uint8Array(fs.readFileSync("test.json"));

// The wasm memory
const wasmMemory = instance.exports.memory;
const jsonTarget = new Uint8Array(wasmMemory.buffer, dataStart, json.length);

jsonTarget.set(json);

// A simple entry point for debugging
instance.exports.processGoals(json.length, 24 * 7 * 2);
