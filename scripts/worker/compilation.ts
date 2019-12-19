importScripts("./js-lib/wasm/wasm_module.js");

const initPromise = wasm_bindgen("./js-lib/wasm/wasm_module_bg.wasm");

interface CompilerOptions {
    numCells?: number,
    cellSize?: number,
}

interface CanonicalOptions {
    numCells: number,
    cellSize: number,
}

function canonicalOptions(options: CompilerOptions | undefined): CanonicalOptions {
    if (options === undefined) {
        return { numCells: 32768, cellSize: 32 };
    } else {
        return {
            numCells: options.numCells ?? 32768,
            cellSize: options.cellSize ?? 32,
        }
    }
}

/**
 * Compiles the provided Brainfuck program to a binary WebAssembly file.
 * 
 * The resolution of the returned `Promise` can be passed directly to `WebAssembly.compile` or `WebAssembly.instantiate`.
 */
function compileBrainfuck(program: string, options?: CompilerOptions): Uint8Array {
    const canon = canonicalOptions(options);
    return wasm_bindgen.compile_brainfuck(program, canon.numCells, canon.cellSize);
}

/**
 * Compiles the provided Brainfuck program to a `WebAssembly.Module`.
 */
function compileBrainfuckToModule(program: string, options?: CompilerOptions): Promise<WebAssembly.Module> {
    return WebAssembly.compile(compileBrainfuck(program, options));
}
