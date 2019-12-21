"use strict";
importScripts("./js-lib/wasm/wasm_module.js");
const initPromise = wasm_bindgen("./js-lib/wasm/wasm_module_bg.wasm");
function canonicalOptions(options) {
    var _a, _b;
    if (options === undefined) {
        return { numCells: 32768, cellSize: 32 };
    }
    else {
        return {
            numCells: (_a = options.numCells, (_a !== null && _a !== void 0 ? _a : 32768)),
            cellSize: (_b = options.cellSize, (_b !== null && _b !== void 0 ? _b : 32)),
        };
    }
}
function compileBrainfuck(program, options) {
    const canon = canonicalOptions(options);
    return wasm_bindgen.compile_brainfuck(program, canon.numCells, canon.cellSize);
}
function compileBrainfuckToModule(program, options) {
    return WebAssembly.compile(compileBrainfuck(program, options));
}
async function runBrainfuckWithCallbacks(mod, readCallback, writeCallback) {
    await WebAssembly.instantiate(mod, { io: { read_value: readCallback, write_value: writeCallback } });
}
async function runBrainfuckWithBuffers(mod, readBuffer, afterEmpty) {
    let bufferPos = 0;
    const read_value = () => {
        if (bufferPos < readBuffer.length) {
            return readBuffer[bufferPos++];
        }
        else {
            return afterEmpty;
        }
    };
    const ret = [];
    const write_value = (i32) => ret.push(i32);
    await WebAssembly.instantiate(mod, { io: { read_value, write_value } });
    return ret;
}
async function runBrainfuckWithStringBuffers(mod, readBuffer, afterEmpty) {
    let bufferPos = 0;
    const read_value = () => {
        if (bufferPos < readBuffer.length) {
            return readBuffer.charCodeAt(bufferPos++);
        }
        else {
            return afterEmpty;
        }
    };
    let ret = "";
    const write_value = (i32) => ret += String.fromCharCode(i32);
    await WebAssembly.instantiate(mod, { io: { read_value, write_value } });
    return ret;
}
addEventListener("message", async (event) => {
    const msg = event.data;
    let mod;
    try {
        mod = await compileIfNecessary(msg.program, msg.options);
    }
    catch (e) {
        postStatus({ status: 3, output: "" + e, });
        return;
    }
    postStatus({ status: 2, output: "", });
    let output;
    try {
        output = await runBrainfuckWithStringBuffers(mod, msg.input, msg.afterEmpty);
    }
    catch (e) {
        postStatus({ status: 4, output: "" + e });
        return;
    }
    postStatus({ status: 5, output });
});
postMessage("ready");
async function compileIfNecessary(program, options) {
    if (program instanceof WebAssembly.Module) {
        return program;
    }
    postStatus({ status: 0, output: "", });
    await wasm_bindgen(program.bfMod);
    const mod = await compileBrainfuckToModule(program.text, options);
    postStatus({ status: 1, output: "", mod });
    return mod;
}
function postStatus(status) {
    postMessage(status);
}
//# sourceMappingURL=main.js.map