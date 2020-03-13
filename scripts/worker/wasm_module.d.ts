declare const wasm_bindgen: ((module: URL | string | Request | WebAssembly.Module) => Promise<WebAssembly.Exports>) & {
    compile_brainfuck: (program: string, num_cells: number, cell_size: number, eof: number) => Uint8Array,
};