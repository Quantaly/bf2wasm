window.addEventListener("load", async _ => {
    const charbuf = [];
    const read_value = () => -1;
    const write_value = (i32) => charbuf.push(i32);

    await WebAssembly.instantiateStreaming(fetch("hello.wasm"), { io: { read_value, write_value } });
    console.log(charbuf);
    const decoder = new TextDecoder("utf-8");
    const result = decoder.decode(new Uint8Array(charbuf));
    console.log(result);
    console.log("done");
});