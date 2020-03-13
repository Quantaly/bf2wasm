window.addEventListener("load", async _ => {
    const read_value = () => -1; // will have bits higher than 8th set
    const write_value = (i32) => console.log(i32);

    await WebAssembly.instantiateStreaming(fetch("eof.wasm"), { io: { read_value, write_value } });
    console.log("done");
});