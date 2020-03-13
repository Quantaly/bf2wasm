importScripts("./js-lib/wasm/wasm_module.js");

const initPromise = wasm_bindgen("./js-lib/wasm/wasm_module_bg.wasm");

type EOFBehavior = "no-change" | "0" | "-1";

interface CompilerOptions {
    numCells?: number,
    cellSize?: number,
    eof?: EOFBehavior,
}

interface CanonicalOptions {
    numCells: number,
    cellSize: number,
    eof: EOFBehavior,
}

function quantifyEOF(eof: EOFBehavior): number {
    switch (eof) {
        case "0":
            return 0;
        case "-1":
            return -1;
        default:
            return 1;
    }
}

function canonicalOptions(options: CompilerOptions | undefined): CanonicalOptions {
    if (options === undefined) {
        return { numCells: 32768, cellSize: 32, eof: "no-change" };
    } else {
        return {
            numCells: options.numCells ?? 32768,
            cellSize: options.cellSize ?? 32,
            eof: options.eof ?? "no-change",
        };
    }
}

/**
 * Compiles the provided Brainfuck program to a binary WebAssembly file.
 * 
 * The resolution of the returned `Promise` can be passed directly to `WebAssembly.compile` or `WebAssembly.instantiate`.
 */
async function compileBrainfuck(program: string, options?: CompilerOptions): Promise<Uint8Array> {
    await initPromise;
    const canon = canonicalOptions(options);
    return wasm_bindgen.compile_brainfuck(program, canon.numCells, canon.cellSize, quantifyEOF(canon.eof));
}

/**
 * Compiles the provided Brainfuck program to a `WebAssembly.Module`.
 */
async function compileBrainfuckToModule(program: string, options?: CompilerOptions): Promise<WebAssembly.Module> {
    return WebAssembly.compile(await compileBrainfuck(program, options));
}
