window.addEventListener("load", async _ => {
    const read_value = () => 42;
    const write_value = (i32) => console.log(i32);

    await WebAssembly.instantiateStreaming(fetch("simple.wasm"), { io: { read_value, write_value } });
    console.log("done");
});