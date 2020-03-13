/** 
 * Runs the provided module, using the provided callbacks as the imports expected by a Brainfuck program.
 */
async function runBrainfuckWithCallbacks(mod: WebAssembly.Module, readCallback: () => number, writeCallback: (i32: number) => void): Promise<void> {
    await WebAssembly.instantiate(mod, { io: { read_value: readCallback, write_value: writeCallback } });
}

/** 
 * Runs the provided module, using `readBuffer` as a source for input and reading EOFs after it is exhausted.
 * 
 * Output is collected into a `Uint8Array`.
 */
async function runBrainfuckWithBuffers(mod: WebAssembly.Module, readBuffer: Uint8Array | number[]): Promise<Uint8Array> {
    let bufferPos = 0;
    const read_value = () => {
        if (bufferPos < readBuffer.length) {
            return readBuffer[bufferPos++];
        } else {
            return -1;
        }
    };

    const ret: number[] = [];
    const write_value = (i32: number) => ret.push(i32);

    await WebAssembly.instantiate(mod, { io: { read_value, write_value } });

    return new Uint8Array(ret);
}

/**
 * Runs the provided module, using `readString`'s UTF-8 representation as input and reading EOFs after it is exhausted.
 * 
 * Output is encoded into a `String` using UTF-8.
 */
async function runBrainfuckWithStringBuffers(mod: WebAssembly.Module, readString: string): Promise<string> {
    const readBuffer = new TextEncoder().encode(readString);
    const output = await runBrainfuckWithBuffers(mod, readBuffer);
    return new TextDecoder("utf-8").decode(output);
}
