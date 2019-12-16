My Wasm compiler compiles to Wasm.

# bf2wasm4wasm

For all of your compiling-Brainfuck-to-WebAssembly-on-the-go needs.

## Building

Install [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/), then run (from this directory, not the repository root):

```
wasm-pack build -t web
```

## Usage

The `pkg` directory created by `wasm-pack` is symlinked as `js-lib/wasm`. The `js-lib` directory itself contains a nicer wrapper around `wasm-pack`'s wrapper through [`compilation.ts`](https://github.com/Quantaly/bf2wasm/blob/master/wasm-module/js-lib/compilation.ts).

Modules emitted by the `compileBrainfuckToModule` function can be directly consumed by functions exported by [`invocation.ts`](https://github.com/Quantaly/bf2wasm/blob/master/wasm-module/js-lib/invocation.ts), while the `Uint8Array` returned by `compileBrainfuck` can be manually passed to [`WebAssembly.compile`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WebAssembly/compile) for use with the invocation helpers.