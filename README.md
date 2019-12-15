# bf2wasm
A compiler from [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck) to [WebAssembly](https://webassembly.org/).

Generated modules import two functions, `io.read_value` and `io.write_value`, that provide the functionality of the `,` and `.` Brainfuck commands, respectively. The Brainfuck program is compiled into the module's "start function" and so is run immediately when the module is instantiated.

The cell array can hold 16,384 32-bit integers (implemented as `i32`s in a single-page Wasm memory). Incrementing a cell's value above the maximum or decrementing below the minimum causes the value to wrap as with standard Wasm integer arithmetic.

It is heavily recommended that generated modules be passed through an external optimizer (such as `wasm-opt` from [WebAssembly/binaryen](https://github.com/WebAssembly/binaryen/)) before deployment to production. It is even heavierly recommended that you find a better language than Brainfuck to use in production.

## "Nice Brainfuck" compliance
To the fullest extent possible, this compiler complies with Brian Raiter's [guidelines for "nice" Brainfuck implementations](https://www.muppetlabs.com/~breadbox/bf/standards.html). The exception is that there is no defined behavior on EOF. The `,` command directly calls the imported `io.read_value` function and places its return value in the cell at the pointer. Therefore, any EOF behavior must be defined by the external implementation of `io.read_value`.