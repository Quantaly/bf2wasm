import init, { compile_brainfuck } from "./wasm/wasm_module.js";
const initPromise = init();
function canonicalOptions(options) {
    if (options === undefined) {
        return { numCells: 32768, cellSize: 32 };
    }
    else {
        return {
            numCells: options.numCells === undefined ? 32768 : options.numCells,
            cellSize: options.cellSize === undefined ? 32 : options.cellSize,
        };
    }
}
export async function compileBrainfuck(program, options) {
    await initPromise;
    const canon = canonicalOptions(options);
    return compile_brainfuck(program, canon.numCells, canon.cellSize);
}
export async function compileBrainfuckToModule(program, options) {
    return WebAssembly.compile(await compileBrainfuck(program, options));
}
//# sourceMappingURL=compilation.js.map