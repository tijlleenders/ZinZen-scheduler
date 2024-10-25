

let WASM_VECTOR_LEN = 0;

let cachedUint8Memory0 = null;

function getUint8Memory0() {
    if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
}

const cachedTextEncoder = (typeof TextEncoder !== 'undefined' ? new TextEncoder('utf-8') : { encode: () => { throw Error('TextEncoder not available') } } );

const encodeString = function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
};

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8Memory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachedInt32Memory0 = null;

function getInt32Memory0() {
    if (cachedInt32Memory0 === null || cachedInt32Memory0.byteLength === 0) {
        cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32Memory0;
}

const cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );

if (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); };

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

let cachedFloat64Memory0 = null;

function getFloat64Memory0() {
    if (cachedFloat64Memory0 === null || cachedFloat64Memory0.byteLength === 0) {
        cachedFloat64Memory0 = new Float64Array(wasm.memory.buffer);
    }
    return cachedFloat64Memory0;
}

let cachedBigInt64Memory0 = null;

function getBigInt64Memory0() {
    if (cachedBigInt64Memory0 === null || cachedBigInt64Memory0.byteLength === 0) {
        cachedBigInt64Memory0 = new BigInt64Array(wasm.memory.buffer);
    }
    return cachedBigInt64Memory0;
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

function takeFromExternrefTable0(idx) {
    const value = wasm.__wbindgen_export_2.get(idx);
    wasm.__externref_table_dealloc(idx);
    return value;
}
/**
* The main wasm function to call
* @param {any} input
* @returns {any}
*/
export function schedule(input) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.schedule(retptr, input);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        var r2 = getInt32Memory0()[retptr / 4 + 2];
        if (r2) {
            throw takeFromExternrefTable0(r1);
        }
        return takeFromExternrefTable0(r0);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_export_2.set(idx, obj);
    return idx;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
}

const imports = {
    __wbindgen_placeholder__: {
        __wbindgen_is_undefined: function(arg0) {
            const ret = arg0 === undefined;
            return ret;
        },
        __wbindgen_in: function(arg0, arg1) {
            const ret = arg0 in arg1;
            return ret;
        },
        __wbindgen_is_bigint: function(arg0) {
            const ret = typeof(arg0) === 'bigint';
            return ret;
        },
        __wbindgen_bigint_from_u64: function(arg0) {
            const ret = BigInt.asUintN(64, arg0);
            return ret;
        },
        __wbindgen_jsval_eq: function(arg0, arg1) {
            const ret = arg0 === arg1;
            return ret;
        },
        __wbindgen_string_get: function(arg0, arg1) {
            const obj = arg1;
            const ret = typeof(obj) === 'string' ? obj : undefined;
            var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len1;
            getInt32Memory0()[arg0 / 4 + 0] = ptr1;
        },
        __wbindgen_is_object: function(arg0) {
            const val = arg0;
            const ret = typeof(val) === 'object' && val !== null;
            return ret;
        },
        __wbindgen_error_new: function(arg0, arg1) {
            const ret = new Error(getStringFromWasm0(arg0, arg1));
            return ret;
        },
        __wbg_new_abda76e883ba8a5f: function() {
            const ret = new Error();
            return ret;
        },
        __wbg_stack_658279fe44541cf6: function(arg0, arg1) {
            const ret = arg1.stack;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len1;
            getInt32Memory0()[arg0 / 4 + 0] = ptr1;
        },
        __wbg_error_f851667af71bcfc6: function(arg0, arg1) {
            let deferred0_0;
            let deferred0_1;
            try {
                deferred0_0 = arg0;
                deferred0_1 = arg1;
                console.error(getStringFromWasm0(arg0, arg1));
            } finally {
                wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
            }
        },
        __wbindgen_jsval_loose_eq: function(arg0, arg1) {
            const ret = arg0 == arg1;
            return ret;
        },
        __wbindgen_boolean_get: function(arg0) {
            const v = arg0;
            const ret = typeof(v) === 'boolean' ? (v ? 1 : 0) : 2;
            return ret;
        },
        __wbindgen_number_get: function(arg0, arg1) {
            const obj = arg1;
            const ret = typeof(obj) === 'number' ? obj : undefined;
            getFloat64Memory0()[arg0 / 8 + 1] = isLikeNone(ret) ? 0 : ret;
            getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);
        },
        __wbg_String_88810dfeb4021902: function(arg0, arg1) {
            const ret = String(arg1);
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len1;
            getInt32Memory0()[arg0 / 4 + 0] = ptr1;
        },
        __wbindgen_number_new: function(arg0) {
            const ret = arg0;
            return ret;
        },
        __wbindgen_string_new: function(arg0, arg1) {
            const ret = getStringFromWasm0(arg0, arg1);
            return ret;
        },
        __wbg_getwithrefkey_5e6d9547403deab8: function(arg0, arg1) {
            const ret = arg0[arg1];
            return ret;
        },
        __wbg_set_841ac57cff3d672b: function(arg0, arg1, arg2) {
            arg0[arg1] = arg2;
        },
        __wbg_get_4a9aa5157afeb382: function(arg0, arg1) {
            const ret = arg0[arg1 >>> 0];
            return ret;
        },
        __wbg_length_cace2e0b3ddc0502: function(arg0) {
            const ret = arg0.length;
            return ret;
        },
        __wbg_new_08236689f0afb357: function() {
            const ret = new Array();
            return ret;
        },
        __wbindgen_is_function: function(arg0) {
            const ret = typeof(arg0) === 'function';
            return ret;
        },
        __wbg_next_15da6a3df9290720: function(arg0) {
            const ret = arg0.next;
            return ret;
        },
        __wbg_next_1989a20442400aaa: function() { return handleError(function (arg0) {
            const ret = arg0.next();
            return ret;
        }, arguments) },
        __wbg_done_bc26bf4ada718266: function(arg0) {
            const ret = arg0.done;
            return ret;
        },
        __wbg_value_0570714ff7d75f35: function(arg0) {
            const ret = arg0.value;
            return ret;
        },
        __wbg_iterator_7ee1a391d310f8e4: function() {
            const ret = Symbol.iterator;
            return ret;
        },
        __wbg_get_2aff440840bb6202: function() { return handleError(function (arg0, arg1) {
            const ret = Reflect.get(arg0, arg1);
            return ret;
        }, arguments) },
        __wbg_call_669127b9d730c650: function() { return handleError(function (arg0, arg1) {
            const ret = arg0.call(arg1);
            return ret;
        }, arguments) },
        __wbg_new_c728d68b8b34487e: function() {
            const ret = new Object();
            return ret;
        },
        __wbg_set_0ac78a2bc07da03c: function(arg0, arg1, arg2) {
            arg0[arg1 >>> 0] = arg2;
        },
        __wbg_isArray_38525be7442aa21e: function(arg0) {
            const ret = Array.isArray(arg0);
            return ret;
        },
        __wbg_instanceof_ArrayBuffer_c7cc317e5c29cc0d: function(arg0) {
            let result;
            try {
                result = arg0 instanceof ArrayBuffer;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_isSafeInteger_c38b0a16d0c7cef7: function(arg0) {
            const ret = Number.isSafeInteger(arg0);
            return ret;
        },
        __wbg_buffer_344d9b41efe96da7: function(arg0) {
            const ret = arg0.buffer;
            return ret;
        },
        __wbg_new_d8a000788389a31e: function(arg0) {
            const ret = new Uint8Array(arg0);
            return ret;
        },
        __wbg_set_dcfd613a3420f908: function(arg0, arg1, arg2) {
            arg0.set(arg1, arg2 >>> 0);
        },
        __wbg_length_a5587d6cd79ab197: function(arg0) {
            const ret = arg0.length;
            return ret;
        },
        __wbg_instanceof_Uint8Array_19e6f142a5e7e1e1: function(arg0) {
            let result;
            try {
                result = arg0 instanceof Uint8Array;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbindgen_bigint_get_as_i64: function(arg0, arg1) {
            const v = arg1;
            const ret = typeof(v) === 'bigint' ? v : undefined;
            getBigInt64Memory0()[arg0 / 8 + 1] = isLikeNone(ret) ? BigInt(0) : ret;
            getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);
        },
        __wbindgen_debug_string: function(arg0, arg1) {
            const ret = debugString(arg1);
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len1;
            getInt32Memory0()[arg0 / 4 + 0] = ptr1;
        },
        __wbindgen_throw: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
        __wbindgen_memory: function() {
            const ret = wasm.memory;
            return ret;
        },
        __wbindgen_init_externref_table: function() {
            const table = wasm.__wbindgen_export_2;
            const offset = table.grow(4);
            table.set(0, undefined);
            table.set(offset + 0, undefined);
            table.set(offset + 1, null);
            table.set(offset + 2, true);
            table.set(offset + 3, false);
            ;
        },
    },

};

const wasm_url = new URL('scheduler_bg.wasm', import.meta.url);
let wasmCode = '';
switch (wasm_url.protocol) {
    case 'file:':
    wasmCode = await Deno.readFile(wasm_url);
    break
    case 'https:':
    case 'http:':
    wasmCode = await (await fetch(wasm_url)).arrayBuffer();
    break
    default:
    throw new Error(`Unsupported protocol: ${wasm_url.protocol}`);
}

const wasmInstance = (await WebAssembly.instantiate(wasmCode, imports)).instance;
const wasm = wasmInstance.exports;

wasm.__wbindgen_start();

