export async function runBrainfuckWithCallbacks(mod: WebAssembly.Module, readCallback: () => number, writeCallback: (i32: number) => void): Promise<void> {
    await WebAssembly.instantiate(mod, { io: { read_value: readCallback, write_value: writeCallback } });
}

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

export async function runBrainfuckWithStringBuffers(mod: WebAssembly.Module, readBuffer: string, afterEmpty: number): Promise<string> {
    let bufferPos = 0;
    const read_value = () => {
        if (bufferPos < readBuffer.length) {
            return readBuffer.charCodeAt(bufferPos++)
        } else {
            return afterEmpty;
        }
    }

    const outBuf: number[] = [];
    const write_value = (i32: number) => outBuf.push(i32);

    await WebAssembly.instantiate(mod, { io: { read_value, write_value } });

    let ret = "";
    outBuf.forEach(code => ret += String.fromCharCode(code));
    return ret;
}
