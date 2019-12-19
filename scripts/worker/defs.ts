/// Represents a request made to a worker to compile and/or run a program.
interface WorkerRequest {
    readonly program: string | WebAssembly.Module,
    readonly input: string,
    readonly afterEmpty: number,
    readonly bfMod: WebAssembly.Module,
}

/// The possible statuses of a running program.
const enum ProgramStatus {
    compiling, compiled, running, compileError, runtimeError, done, terminated
}

/// Represents the full status of a running program.
interface WorkerStatus {
    readonly status: ProgramStatus,
    readonly mod?: WebAssembly.Module,
    readonly output: string,
}
