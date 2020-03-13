"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
importScripts("./js-lib/wasm/wasm_module.js");
const initPromise = wasm_bindgen("./js-lib/wasm/wasm_module_bg.wasm");
function quantifyEOF(eof) {
    switch (eof) {
        case "0":
            return 0;
        case "-1":
            return -1;
        default:
            return 1;
    }
}
function canonicalOptions(options) {
    var _a, _b, _c;
    if (options === undefined) {
        return { numCells: 32768, cellSize: 32, eof: "no-change" };
    }
    else {
        return {
            numCells: (_a = options.numCells, (_a !== null && _a !== void 0 ? _a : 32768)),
            cellSize: (_b = options.cellSize, (_b !== null && _b !== void 0 ? _b : 32)),
            eof: (_c = options.eof, (_c !== null && _c !== void 0 ? _c : "no-change")),
        };
    }
}
function compileBrainfuck(program, options) {
    return __awaiter(this, void 0, void 0, function* () {
        yield initPromise;
        const canon = canonicalOptions(options);
        return wasm_bindgen.compile_brainfuck(program, canon.numCells, canon.cellSize, quantifyEOF(canon.eof));
    });
}
function compileBrainfuckToModule(program, options) {
    return __awaiter(this, void 0, void 0, function* () {
        return WebAssembly.compile(yield compileBrainfuck(program, options));
    });
}
function runBrainfuckWithCallbacks(mod, readCallback, writeCallback) {
    return __awaiter(this, void 0, void 0, function* () {
        yield WebAssembly.instantiate(mod, { io: { read_value: readCallback, write_value: writeCallback } });
    });
}
function runBrainfuckWithBuffers(mod, readBuffer) {
    return __awaiter(this, void 0, void 0, function* () {
        let bufferPos = 0;
        const read_value = () => {
            if (bufferPos < readBuffer.length) {
                return readBuffer[bufferPos++];
            }
            else {
                return -1;
            }
        };
        const ret = [];
        const write_value = (i32) => ret.push(i32);
        yield WebAssembly.instantiate(mod, { io: { read_value, write_value } });
        return new Uint8Array(ret);
    });
}
function runBrainfuckWithStringBuffers(mod, readString) {
    return __awaiter(this, void 0, void 0, function* () {
        const readBuffer = new TextEncoder().encode(readString);
        const output = yield runBrainfuckWithBuffers(mod, readBuffer);
        return new TextDecoder("utf-8").decode(output);
    });
}
addEventListener("message", (event) => __awaiter(void 0, void 0, void 0, function* () {
    const msg = event.data;
    let mod;
    try {
        mod = yield compileIfNecessary(msg.program, msg.options);
    }
    catch (e) {
        postStatus({ status: 3, output: "" + e, });
        return;
    }
    postStatus({ status: 2, output: "", });
    let output;
    try {
        output = yield runBrainfuckWithStringBuffers(mod, msg.input);
    }
    catch (e) {
        postStatus({ status: 4, output: "" + e });
        return;
    }
    postStatus({ status: 5, output });
}));
postMessage("ready");
function compileIfNecessary(program, options) {
    return __awaiter(this, void 0, void 0, function* () {
        if (program instanceof WebAssembly.Module) {
            return program;
        }
        postStatus({ status: 0, output: "", });
        yield wasm_bindgen(program.bfMod);
        const mod = yield compileBrainfuckToModule(program.text, options);
        postStatus({ status: 1, output: "", mod });
        return mod;
    });
}
function postStatus(status) {
    postMessage(status);
}
//# sourceMappingURL=main.js.map