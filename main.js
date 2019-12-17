import * as bfc from "./js-lib/compilation.js";
import * as bfi from "./js-lib/invocation.js";
const program = document.querySelector("#program");
const input = document.querySelector("#input");
const output = document.querySelector("#output");
const runButton = document.querySelector("#run");
const zeroEof = document.querySelector("#zero");
const status = document.querySelector(".controls-status");
let moduleCache;
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
runButton.addEventListener("click", async (_) => {
    disableEverything();
    if (!cached) {
        status.innerText = "Compiling...";
        try {
            moduleCache = await bfc.compileBrainfuckToModule(program.value);
            cached = true;
        }
        catch (e) {
            status.innerText = "Error compiling";
            output.value = "" + e;
            enableEverything();
            return;
        }
    }
    status.innerText = "Running...";
    output.value = await bfi.runBrainfuckWithStringBuffers(moduleCache, input.value, zeroEof.checked ? 0 : -1);
    enableEverything();
    status.innerText = "Done";
});
//# sourceMappingURL=main.js.map