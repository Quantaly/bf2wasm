import init, { compile_brainfuck } from "./pkg/wasm_module.js";

const initPromise = init();

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
            numCells: options.numCells === undefined ? 32768 : options.numCells,
            cellSize: options.cellSize === undefined ? 32 : options.cellSize,
        }
    }
}

export async function compileBrainfuck(program: string, options?: CompilerOptions): Promise<Uint8Array> {
    await initPromise;
    const canon = canonicalOptions(options);
    return compile_brainfuck(program, canon.numCells, canon.cellSize);
}

export async function compileBrainfuckToModule(program: string, options?: CompilerOptions): Promise<WebAssembly.Module> {
    return WebAssembly.compile(await compileBrainfuck(program, options));
}
