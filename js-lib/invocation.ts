/** 
 * Runs the provided module, using the provided callbacks as the imports expected by a Brainfuck program.
 */
export async function runBrainfuckWithCallbacks(mod: WebAssembly.Module, readCallback: () => number, writeCallback: (i32: number) => void): Promise<void> {
    await WebAssembly.instantiate(mod, { io: { read_value: readCallback, write_value: writeCallback } });
}

/** 
 * Runs the provided module, using `readBuffer` as a source for input. Once all of the values are read, `afterEmpty` is used as input.
 * 
 * Output is collected into an array of `Number`s.
 */
export async function runBrainfuckWithBuffers(mod: WebAssembly.Module, readBuffer: number[], afterEmpty: number): Promise<number[]> {
    let bufferPos = 0;
    const read_value = () => {
        if (bufferPos < readBuffer.length) {
            return readBuffer[bufferPos++];
        } else {
            return afterEmpty;
        }
    }

    const ret: number[] = [];
    const write_value = (i32: number) => ret.push(i32);

    await WebAssembly.instantiate(mod, { io: { read_value, write_value } });

    return ret;
}

/**
 * Runs the provided module, using `readBuffer` as a source for input. Once all of the characters are read, `afterEmpty` is used as input.
 * 
 * Output is collected into a `String`.
 */
export async function runBrainfuckWithStringBuffers(mod: WebAssembly.Module, readBuffer: string, afterEmpty: number): Promise<string> {
    let bufferPos = 0;
    const read_value = () => {
        if (bufferPos < readBuffer.length) {
            return readBuffer.charCodeAt(bufferPos++)
        } else {
            return afterEmpty;
        }
    }

    let ret = "";
    const write_value = (i32: number) => ret += String.fromCharCode(i32);

    await WebAssembly.instantiate(mod, { io: { read_value, write_value } });

    return ret;
}
