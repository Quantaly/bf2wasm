My Wasm compiler compiles to Wasm.

# bf2wasm4wasm

For all of your compiling-Brainfuck-to-WebAssembly-on-the-go needs.

## Building

Install [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/), then run (from this directory, not the repository root):

```
wasm-pack build -t web
```

## Usage

`wasm-pack` will create a `pkg` directory with the Wasm module and some JS glue code. [`compilation.ts`](https://github.com/Quantaly/bf2wasm/blob/master/wasm-module/compilation.ts) exports a nicer wrapper around that wrapper; leave it one directory below `pkg`.

Modules emitted by the `compileBrainfuckToModule` function can be directly consumed by functions exported by [`invocation.ts`](https://github.com/Quantaly/bf2wasm/blob/master/wasm-module/invocation.ts), while the `Uint8Array` returned by `compileBrainfuck` can be manually passed to [`WebAssembly.compile`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WebAssembly/compile) for use with the invocation helpers.