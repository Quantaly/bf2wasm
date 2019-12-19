const bfModPromise = WebAssembly.compileStreaming(fetch("./scripts/worker/js-lib/wasm/wasm_module_bg.wasm"));
export class WorkerWrapper {
    constructor(program, input, afterEmpty, callbacks) {
        this.resolveEnded = () => { };
        this.worker = new Worker("scripts/worker/main.js");
        this.callbacks = callbacks;
        this._ended = new Promise((resolve, _) => this.resolveEnded = resolve);
        this.worker.addEventListener("message", _ => {
            this.worker.addEventListener("message", event => {
                const msg = event.data;
                callbacks.updateUi(msg.status, msg.output);
                if (msg.mod !== undefined) {
                    callbacks.cacheModule(msg.mod);
                }
                if (msg.status === 5 ||
                    msg.status === 3 ||
                    msg.status === 4) {
                    this.worker.terminate();
                    this.resolveEnded();
                }
            });
            bfModPromise.then(bfMod => this.worker.postMessage({ program, input, afterEmpty, bfMod }));
        }, { once: true });
    }
    get ended() {
        return this._ended;
    }
    terminate() {
        this.worker.terminate();
        this.callbacks.updateUi(6, "");
        this.resolveEnded();
    }
}
//# sourceMappingURL=defs.js.map