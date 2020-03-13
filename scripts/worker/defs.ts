/// Represents a request made to a worker to compile and/or run a program.
interface WorkerRequest {
    readonly program: WorkerProgram,
    readonly input: string,
    readonly options: CompilerOptions,
}

/// Represents a program and/or the means to compile it.
type WorkerProgram = string | WebAssembly.Module;

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
