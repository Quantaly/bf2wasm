import * as bfc from "./js-lib/compilation.js";
import * as bfi from "./js-lib/invocation.js";

(async function () {
    const mod = await bfc.compileBrainfuckToModule(",.>+++++[.-]<.");
    await bfi.runBrainfuckWithCallbacks(mod, () => 42, (i32) => console.log(i32));
    console.log("done");
})();