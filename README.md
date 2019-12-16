# bf2wasm
A compiler from [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck) to [WebAssembly](https://webassembly.org/).

Generated modules import two functions, `io.read_value` and `io.write_value`, that provide the functionality of the `,` and `.` Brainfuck commands, respectively. The Brainfuck program is compiled into the module's "start function" and so is run immediately when the module is instantiated. There is also a [TypeScript module](https://github.com/Quantaly/bf2wasm/blob/master/wasm-module/js-lib/invocation.ts) that abstracts away all of that with nice wrapper functions.

It is heavily recommended that generated modules be passed through an external optimizer (such as `wasm-opt` from [WebAssembly/binaryen](https://github.com/WebAssembly/binaryen/)) before deployment to production. It is even heavierly recommended that you find a better language than Brainfuck to use in production.

## Implementation details
To the fullest extent possible, this compiler complies with Brian Raiter's [guidelines for "nice" Brainfuck implementations](https://www.muppetlabs.com/~breadbox/bf/standards.html).

By default, the cell array can hold 32,768 32-bit integers. This can be changed using the `--num-cells` and `--cell-size` command-line options.

Incrementing a cell's value above the maximum or decrementing below the minimum causes the value to wrap.

No matter the cell size, the `.` command will only output up to the least significant 32 bits of the cell value and the `,` command will accept 32 bits of input, wrapped or zero-extended to the cell size if necessary.

This compiler does not define any behavior related to EOF conditions. The `,` command directly calls the imported `io.read_value` function and places its return value in the cell at the pointer. Therefore, any EOF behavior must be defined by the external implementation of `io.read_value`.

It is also technically possible, when using `--cell-size=64` and a sufficiently low `--num-cells`, to end up with a cell array smaller than Raiter's minimum length of 9999. Because it takes a very specific conjunction of compiler options to reach this state, it can be considered intentional. Consider this your warning, if you must; you reap what you sow.