# bf2wasm
A compiler from [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck) to [WebAssembly](https://webassembly.org/).

Generated modules import two functions, `io.read_value` and `io.write_value`, that provide the functionality of the `,` and `.` Brainfuck commands, respectively. The Brainfuck program is compiled into the module's "start function" and so is run immediately when the module is instantiated.

By default, the cell array can hold 32,768 32-bit integers. This can be changed using the `--num-cells` and `--cell-size` command-line options.

(Note that JavaScript embedders, i.e. the browser, currently cannot use modules compiled with `--cell-size=64`. When this option is set, the imported `io` functions take and return `i64`s, which cannot be faithfully converted to and from JS `Number`s. With all other supported cell sizes, the `io` functions use `i32`s, which can be converted.)

Incrementing a cell's value above the maximum or decrementing below the minimum causes the value to wrap.

It is heavily recommended that generated modules be passed through an external optimizer (such as `wasm-opt` from [WebAssembly/binaryen](https://github.com/WebAssembly/binaryen/)) before deployment to production. It is even heavierly recommended that you find a better language than Brainfuck to use in production.

## "Nice Brainfuck" compliance
To the fullest extent possible, this compiler complies with Brian Raiter's [guidelines for "nice" Brainfuck implementations](https://www.muppetlabs.com/~breadbox/bf/standards.html). The exception is that there is no defined behavior on EOF. The `,` command directly calls the imported `io.read_value` function and places its return value in the cell at the pointer. Therefore, any EOF behavior must be defined by the external implementation of `io.read_value`.