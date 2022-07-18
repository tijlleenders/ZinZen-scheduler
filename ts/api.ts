// interface Task {
//     id: number,
//     title: string,
//     /**
//      * How long a given goal will take, in hours.
//      */
//     duration: number,
//     /**
//      * The start of this task, as an IS0 8601 string.
//      */
//     start: string,
//     /**
//      * The deadline of this task, as an IS0 8601 string.
//      */
//     deadline: string,
// }
//
// /**
//  * Input to the scheduler.
//  *
//  * The scheduler, given tasks and a start/end time, will return these tasks scheduled for you.
//  *
//  * Dates must be formatted as ISO 8601. We recommend using the `Date.toISOString()` method:
//  *
//  * ```ts
//  * const dt = new Date('05 October 2011 14:48 UTC'):
//  * const isoString = dt.toISOString();
//  * console.log(isoString); // "2011-10-05T14:48:00.000Z"
//  * ```
//  *
//  * Expected input should look like this:
//  *
//  * ```ts
//  * {
//  *     "startDate": "2022-01-01T00:00:00Z",
//  *     "endDate": "2022-01-02T00:00:00Z",
//  *     // ...
//  * }
//  * ```
//  */
// export interface Input {
//     /** The start date, formatted as an ISO8601 date. */
//     startDate: string;
//     /** The end date, formatted as an ISO8601 date. */
//     endDate: string;
//     tasks: Task[]
// }
//
// /**
//  * Span of time that a Task can fit into. A Task can have multiple slots.
//  */
// interface Slot {
//     task_id: number,
//     /**
//      * The start of this slot, as the number of hours since the startDate.
//      */
//     start: number,
//     /**
//      * The end of this slot, as the number of hours since the startDate.
//      */
//     end: number,
// }
//
// /**
//  * Result of the scheduler.
//  */
// export interface Result {
//     tasks: Task[],
//     slots: Slot[],
// }
//
//
// /**
//  * The wrapper API class
//  */
// export class API {
//     private instance: WebAssembly.Instance;
//     private textDecoder: TextDecoder;
//     private textEncoder: TextEncoder;
//     private ipcStart: number;
//     private wasmMemory: WebAssembly.Memory;
//
//     // Internal constructor for API class
//     constructor(instance: WebAssembly.Instance, ipcStart: number, wasmMemory: WebAssembly.Memory) {
//         this.instance = instance;
//         this.textDecoder = new TextDecoder;
//         this.textEncoder = new TextEncoder;
//         this.ipcStart = ipcStart;
//         this.wasmMemory = wasmMemory;
//     }
//
//     /**
//      * Given an array of Goals, it will process the total number of tasks per Goal that can fit within the timeline
//      * @param goals An array of Goals to be fitted into a timeline
//      * @param start The start of the timeline
//      * @param finish The end of the timeline
//      * @returns A Map from a Goal's ID to how many tasks can fit within the timeline
//      */
//     public processTaskCount(goals: Goal[], start: Date, finish: Date): Map<GoalID, number> {
//         // Encode data
//         const string = JSON.stringify([goals, [jsDateToDateTime(start), jsDateToDateTime(finish)]]);
//         const data = this.textEncoder.encode(string);
//
//         // Send data
//         const target = this.getIPCView(data.length);
//         target.set(data);
//
//         // Process
//         const offset = (this.instance.exports.processTaskCount as CallableFunction)(data.length) as number;
//         const buffer = this.getIPCView(offset);
//         const readString = this.textDecoder.decode(buffer);
//         const iterator = (JSON.parse(readString) as [[number, GoalID]]).map(a => ([a[1], a[0]] as [number, number]));
//
//         return new Map(iterator)
//     }
//
//     /**
//      * A fundamental function, that parses Goals and produces a Schedule, which is just an array of tasks
//      * @param goals The array of Goals to be processed
//      * @param start The start of the timeline
//      * @param finish The end of the timeline
//      * @returns A schedule
//      */
//     public generateSchedule(input: Input): Result {
//         // Serialize data
//         const plan = {goals, start: jsDateToDateTime(start), finish: jsDateToDateTime(finish)};
//         const string = JSON.stringify(plan);
//         const bytes = this.textEncoder.encode(string);
//
//         // Send data
//         let view = this.getIPCView(bytes.length);
//         view.set(bytes);
//
//         // Call wasm function
//         const result = (this.instance.exports.generateSchedule as CallableFunction)(view.length) as number;
//
//         // Read result
//         view = this.getIPCView(result);
//         const resultString = this.textDecoder.decode(view);
//         return JSON.parse(resultString)
//     }
//
//     /**
//      * @returns A reference to the IPC buffer used for communication with WebAssembly
//      */
//     public getIPCView(offset: number): Uint8Array {
//         return new Uint8Array(this.wasmMemory.buffer, this.ipcStart, offset);
//     }
// }
//
// /**
//  * Loads the WebAssembly api
//  * @param path A valid path that is passed to `fetch` and is used to fetch the webassembly data
//  * @returns A Promise that yields an API to the underlying wasm
//  */
// export async function loadAPI(path: string): Promise<API> {
//     // Build instance
//     const {instance} = await WebAssembly.instantiateStreaming(fetch(path), {
//         env: {
//             console_log(isString: boolean, ipcOffset: number) {
//                 if (isString) {
//                     const readResult = new Uint8Array(_wasmMemory.buffer, _ipcStart, ipcOffset);
//                     const decoder = new TextDecoder();
//                     const string = decoder.decode(readResult);
//
//                     console.log(string);
//                 } else {
//                     const readResult = new Uint8Array(_wasmMemory.buffer, _ipcStart, ipcOffset);
//                     console.log(readResult);
//                 }
//             },
//             exit(error_code: number, ipcOffset: number) {
//                 if (error_code != 0) {
//                     const readResult = new Uint8Array(_wasmMemory.buffer, _ipcStart, ipcOffset);
//                     const decoder = new TextDecoder();
//
//                     console.error(`[WASM_ERROR; ErrorCode:${error_code}] ${decoder.decode(readResult)}`);
//                 } else {
//                     console.info("Webassembly has prematurely finished execution, without errors")
//                 }
//             }
//         },
//     });
//
//     // Build API
//     const _wasmMemory: WebAssembly.Memory = instance.exports.memory as WebAssembly.Memory;
//     const _ipcStart = (instance.exports.IPC_BUFFER as WebAssembly.Global).value as number;
//     const api = new API(instance, _ipcStart, _wasmMemory);
//
//     return api
// }
