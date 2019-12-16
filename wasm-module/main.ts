import * as bfc from "./compilation.js";
import * as bfi from "./invocation.js";

(async function () {
    const mod = await bfc.compileBrainfuckToModule(",.>+++++[.-]<.");
    await bfi.runBrainfuckWithCallbacks(mod, () => 42, (i32) => console.log(i32));
    console.log("done");
})();