/// Represents a request made to a worker to compile and/or run a program.
export interface WorkerRequest {
    readonly program: string | WebAssembly.Module,
    readonly input: string,
    readonly afterEmpty: number,
    readonly bfMod: WebAssembly.Module,
}

/// The possible statuses of a running program.
export const enum ProgramStatus {
    compiling, compiled, running, compileError, runtimeError, done, terminated
}

/// Represents the full status of a running program.
export interface WorkerStatus {
    readonly status: ProgramStatus,
    readonly mod?: WebAssembly.Module,
    readonly output: string,
}

/// Callbacks required by a WorkerWrapper
interface WorkerCallbacks {
    updateUi: (status: ProgramStatus, output: string) => void,
    cacheModule: (mod: WebAssembly.Module) => void,
}

const bfModPromise = WebAssembly.compileStreaming(fetch("./scripts/worker/js-lib/wasm/wasm_module_bg.wasm"));

/// Wraps the functionality of the worker, exposing events and handling cleanup.
export class WorkerWrapper {
    private worker: Worker;
    private callbacks: WorkerCallbacks;
    private _ended: Promise<void>;
    private resolveEnded: () => void = () => { };

    constructor(program: string | WebAssembly.Module, input: string, afterEmpty: number, callbacks: WorkerCallbacks) {
        this.worker = new Worker("scripts/worker/main.js");
        this.callbacks = callbacks;
        this._ended = new Promise((resolve, _) => this.resolveEnded = resolve);
        this.worker.addEventListener("message", _ => {
            this.worker.addEventListener("message", event => {
                const msg: WorkerStatus = event.data;
                callbacks.updateUi(msg.status, msg.output);
                if (msg.mod !== undefined) {
                    callbacks.cacheModule(msg.mod);
                }
                if (msg.status === ProgramStatus.done ||
                    msg.status === ProgramStatus.compileError ||
                    msg.status === ProgramStatus.runtimeError) {
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
        this.callbacks.updateUi(ProgramStatus.terminated, "");
        this.resolveEnded();
    }
}