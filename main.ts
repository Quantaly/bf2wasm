import * as bfc from "./js-lib/compilation.js";
import * as bfi from "./js-lib/invocation.js";

const program = document.querySelector("#program") as HTMLTextAreaElement;
const input = document.querySelector("#input") as HTMLTextAreaElement;
const output = document.querySelector("#output") as HTMLTextAreaElement;

const runButton = document.querySelector("#run") as HTMLButtonElement;
const zeroEof = document.querySelector("#zero") as HTMLInputElement;

const status = document.querySelector(".controls-status") as HTMLDivElement;

let moduleCache: WebAssembly.Module;
let cached = false;

program.addEventListener("input", _ => {
    cached = false;
    status.innerText = "Ready";
});

input.addEventListener("input", _ => {
    status.innerText = "Ready";
});

function disableEverything() {
    program.disabled = true;
    input.disabled = true;
    runButton.disabled = true;
}

function enableEverything() {
    program.disabled = false;
    input.disabled = false;
    runButton.disabled = false;
}

// TODO move all this work to a worker, allow user to terminate
runButton.addEventListener("click", async _ => {
    disableEverything();

    if (!cached) {
        status.innerText = "Compiling...";
        try {
            moduleCache = await bfc.compileBrainfuckToModule(program.value);
            cached = true;
        } catch (e) {
            status.innerText = "Error compiling";
            output.value = "" + e;
            return;
        }
    }

    status.innerText = "Running...";
    output.value = await bfi.runBrainfuckWithStringBuffers(moduleCache, input.value, zeroEof.checked ? 0 : -1);
    enableEverything();
    status.innerText = "Done";
});