import * as defs from "../lib/defs.js";
const programArea = document.querySelector("#program");
const inputArea = document.querySelector("#input");
const outputArea = document.querySelector("#output");
const runButton = document.querySelector("#run");
const stopButton = document.querySelector("#stop");
const zeroEof = document.querySelector("#zero");
const negOneEof = document.querySelector("#neg-one");
const statusDiv = document.querySelector(".controls-status");
let moduleCache;
let cached = false;
let currentWorker;
programArea.addEventListener("input", _ => {
    cached = false;
    statusDiv.innerText = "Ready";
});
inputArea.addEventListener("input", _ => {
    statusDiv.innerText = "Ready";
});
zeroEof.addEventListener("input", _ => {
    statusDiv.innerText = "Ready";
});
negOneEof.addEventListener("input", _ => {
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
async function withEverythingDisabled(run) {
    disableEverything();
    try {
        const ret = await run();
        return ret;
    }
    finally {
        enableEverything();
    }
}
runButton.addEventListener("click", _ => withEverythingDisabled(async () => {
    currentWorker = new defs.WorkerWrapper(cached ? moduleCache : programArea.value, inputArea.value, zeroEof.checked ? 0 : -1, {
        updateUi: (status, output) => {
            statusDiv.innerText = (s => {
                switch (s) {
                    case 0:
                        return "Compiling...";
                    case 1:
                        return "Caching...";
                    case 2:
                        return "Running...";
                    case 3:
                        return "Compilation error";
                    case 4:
                        return "Runtime error";
                    case 5:
                        return "Done";
                    case 6:
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
stopButton.addEventListener("click", _ => { var _a; return (_a = currentWorker) === null || _a === void 0 ? void 0 : _a.terminate(); });
//# sourceMappingURL=main.js.map