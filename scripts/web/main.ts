import * as defs from "../lib/defs.js";

const programArea = document.querySelector("#program") as HTMLTextAreaElement;
const inputArea = document.querySelector("#input") as HTMLTextAreaElement;
const outputArea = document.querySelector("#output") as HTMLTextAreaElement;

const runButton = document.querySelector("#run") as HTMLButtonElement;
const stopButton = document.querySelector("#stop") as HTMLButtonElement;

const cellSize = document.querySelector("#cell-size") as HTMLSelectElement;
const cellArraySize = document.querySelector("#array-size") as HTMLInputElement;

const noChangeEof = document.querySelector("#no-change") as HTMLInputElement;
const zeroEof = document.querySelector("#zero") as HTMLInputElement;
const negOneEof = document.querySelector("#neg-one") as HTMLInputElement;

const statusDiv = document.querySelector(".controls-status") as HTMLDivElement;

let moduleCache: WebAssembly.Module;
let cached = false;

let currentWorker: defs.WorkerWrapper;

programArea.addEventListener("input", _ => {
    cached = false;
    statusDiv.textContent = "Ready";
});

cellSize.addEventListener("input", _ => {
    cached = false;
    statusDiv.textContent = "Ready";
});

cellArraySize.addEventListener("input", _ => {
    cached = false;
    statusDiv.textContent = "Ready";
});

inputArea.addEventListener("input", _ => {
    statusDiv.textContent = "Ready";
});

noChangeEof.addEventListener("input", _ => {
    cached = false;
    statusDiv.textContent = "Ready";
});

zeroEof.addEventListener("input", _ => {
    cached = false;
    statusDiv.textContent = "Ready";
});

negOneEof.addEventListener("input", _ => {
    cached = false;
    statusDiv.textContent = "Ready";
});

function disableEverything() {
    programArea.disabled = true;
    inputArea.disabled = true;
    runButton.disabled = true;
    stopButton.disabled = false;
    cellSize.disabled = true;
    cellArraySize.disabled = true;
    noChangeEof.disabled = true;
    zeroEof.disabled = true;
    negOneEof.disabled = true;
}

function enableEverything() {
    programArea.disabled = false;
    inputArea.disabled = false;
    runButton.disabled = false;
    stopButton.disabled = true;
    cellSize.disabled = false;
    cellArraySize.disabled = false;
    noChangeEof.disabled = false;
    zeroEof.disabled = false;
    negOneEof.disabled = false;
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
        {
            cellSize: Number.parseInt(cellSize.value),
            numCells: Number.parseInt(cellArraySize.value),
            eof: noChangeEof.checked ? "no-change" : negOneEof.checked ? "-1" : zeroEof.checked ? "0" : "no-change",
        },
        {
            updateUi: (status, output) => {
                statusDiv.textContent = (s => {
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
}));

stopButton.addEventListener("click", _ => currentWorker?.terminate());
