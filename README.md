# bf2wasm
A compiler from [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck) to [WebAssembly](https://webassembly.org/).

Generated modules import two functions, `io.read_value` and `io.write_value`, that provide the functionality of the `,` and `.` Brainfuck commands, respectively. The Brainfuck program is compiled into the module's "start function" and so is run immediately when the module is instantiated. There is also a [TypeScript module](https://github.com/Quantaly/bf2wasm/blob/master/wasm-module/js-lib/invocation.ts) that abstracts away all of that with nice wrapper functions.

It is heavily recommended that generated modules be passed through an external optimizer (such as `wasm-opt` from [WebAssembly/binaryen](https://github.com/WebAssembly/binaryen/)) before deployment to production. It is even heavierly recommended that you find a better language than Brainfuck to use in production.

## Implementation details
To the fullest extent possible, this compiler complies with Brian Raiter's [guidelines for "nice" Brainfuck implementations](https://www.muppetlabs.com/~breadbox/bf/standards.html).

By default, the cell array can hold 32,768 32-bit integers. This can be changed using the `--num-cells` and `--cell-size` command-line options.

Incrementing a cell's value above the maximum or decrementing below the minimum causes the value to wrap.

No matter the cell size, the `.` command will only output the least significant 8 bits of the cell value, zero-padded to an `i32`. Likewise, the `,` command will only accept 8 bits of input; if any bit higher than the 8th is set (i.e. greater than 255 unsigned), it is interpreted as an EOF. By default, an EOF leaves the value in the cell unchanged; this behavior can be customized at compile time with the `--eof` option.

It is also technically possible, when using `--cell-size=64` and a sufficiently low `--num-cells`, to end up with a cell array smaller than Raiter's minimum length of 9999. Because it takes a very specific conjunction of compiler options to reach this state, it can be considered intentional. Consider this your warning, if you must; you reap what you sow.