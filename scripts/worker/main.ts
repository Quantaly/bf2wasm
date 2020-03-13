addEventListener("message", async event => {
    const msg: WorkerRequest = event.data;
    let mod: WebAssembly.Module;
    try {
        mod = await compileIfNecessary(msg.program, msg.options);
    } catch (e) {
        postStatus({ status: ProgramStatus.compileError, output: "" + e, });
        return;
    }

    postStatus({ status: ProgramStatus.running, output: "", });
    let output: string;
    try {
        output = await runBrainfuckWithStringBuffers(mod, msg.input);
    } catch (e) {
        postStatus({ status: ProgramStatus.runtimeError, output: "" + e });
        return;
    }
    postStatus({ status: ProgramStatus.done, output });
});

postMessage("ready");

async function compileIfNecessary(program: WorkerProgram, options: CompilerOptions): Promise<WebAssembly.Module> {
    if (program instanceof WebAssembly.Module) {
        return program;
    } else {
        postStatus({ status: ProgramStatus.compiling, output: "", });
        const mod = await compileBrainfuckToModule(program, options);
        postStatus({ status: ProgramStatus.compiled, output: "", mod });
        return mod;
    }
}

function postStatus(status: WorkerStatus) {
    postMessage(status);
}