addEventListener("message", async event => {
    const msg: WorkerRequest = event.data;
    let mod: WebAssembly.Module;
    try {
        mod = await compileIfNecessary(msg.program, msg.bfMod);
    } catch (e) {
        postStatus({ status: ProgramStatus.compileError, output: "" + e, });
        return;
    }

    postStatus({ status: ProgramStatus.running, output: "", });
    let output: string;
    try {
        output = await runBrainfuckWithStringBuffers(mod, msg.input, msg.afterEmpty);
    } catch (e) {
        postStatus({ status: ProgramStatus.runtimeError, output: "" + e });
        return;
    }
    postStatus({ status: ProgramStatus.done, output });
});

postMessage("ready");

async function compileIfNecessary(program: string | WebAssembly.Module, bfMod: WebAssembly.Module): Promise<WebAssembly.Module> {
    if (program instanceof WebAssembly.Module) {
        return program;
    }
    postStatus({ status: ProgramStatus.compiling, output: "", });
    await wasm_bindgen(bfMod);
    const mod = await compileBrainfuckToModule(program);
    postStatus({ status: ProgramStatus.compiled, output: "", mod })
    return mod;
}

function postStatus(status: WorkerStatus) {
    postMessage(status);
}