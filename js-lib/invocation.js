export async function runBrainfuckWithCallbacks(mod, readCallback, writeCallback) {
    await WebAssembly.instantiate(mod, { io: { read_value: readCallback, write_value: writeCallback } });
}
export async function runBrainfuckWithBuffers(mod, readBuffer, afterEmpty) {
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
export async function runBrainfuckWithStringBuffers(mod, readBuffer, afterEmpty) {
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
//# sourceMappingURL=invocation.js.map