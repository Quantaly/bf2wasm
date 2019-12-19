import * as defs from "../lib/defs.js";

const programArea = document.querySelector("#program") as HTMLTextAreaElement;
const inputArea = document.querySelector("#input") as HTMLTextAreaElement;
const outputArea = document.querySelector("#output") as HTMLTextAreaElement;

const runButton = document.querySelector("#run") as HTMLButtonElement;
const stopButton = document.querySelector("#stop") as HTMLButtonElement;
const zeroEof = document.querySelector("#zero") as HTMLInputElement;

const statusDiv = document.querySelector(".controls-status") as HTMLDivElement;

let moduleCache: WebAssembly.Module;
let cached = false;

let currentWorker: defs.WorkerWrapper;

programArea.addEventListener("input", _ => {
    cached = false;
    statusDiv.innerText = "Ready";
});

inputArea.addEventListener("input", _ => {
    statusDiv.innerText = "Ready";
});

function disableEverything() {
    programArea.disabled = true;
    inputArea.disabled = true;
    runButton.disabled = true;
    stopButton.disabled = false;
}

function enableEverything() {
    programArea.disabled = false;
    inputArea.disabled = false;
    runButton.disabled = false;
    stopButton.disabled = true;
}

async function withEverythingDisabled<T>(run: () => Promise<T>): Promise<T> {
    disableEverything();
    try {
        const ret = await run();
        return ret;
    } finally {
        enableEverything();
    }
}

runButton.addEventListener("click", _ => withEverythingDisabled(async () => {
    currentWorker = new defs.WorkerWrapper(
        cached ? moduleCache : programArea.value,
        inputArea.value,
        zeroEof.checked ? 0 : -1,
        {
            updateUi: (status, output) => {
                statusDiv.innerText = (s => {
                    switch (s) {
                        case defs.ProgramStatus.compiling:
                            return "Compiling...";
                        case defs.ProgramStatus.compiled:
                            return "Caching...";
                        case defs.ProgramStatus.running:
                            return "Running...";
                        case defs.ProgramStatus.compileError:
                            return "Compilation error";
                        case defs.ProgramStatus.runtimeError:
                            return "Runtime error";
                        case defs.ProgramStatus.done:
                            return "Done";
                        case defs.ProgramStatus.terminated:
                            return "Terminated";
                    }
                })(status);
                outputArea.value = output;
            }, cacheModule: (mod) => {
                moduleCache = mod;
                cached = true;
            },
        });
    await currentWorker.ended;

    /*if (!cached) {
        statusDiv.innerText = "Compiling...";
        try {
            moduleCache = await bfc.compileBrainfuckToModule(program.value);
            cached = true;
        } catch (e) {
            statusDiv.innerText = "Error compiling";
            output.value = "" + e;
            return;
        }
    }

    statusDiv.innerText = "Running...";
    try {
        output.value = await bfi.runBrainfuckWithStringBuffers(moduleCache, input.value, zeroEof.checked ? 0 : -1);
    } catch (e) {
        statusDiv.innerText = "Runtime error";
        output.value = "" + e;
        return;
    }
    statusDiv.innerText = "Done";*/
}));

stopButton.addEventListener("click", _ => currentWorker?.terminate());
