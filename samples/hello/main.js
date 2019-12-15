window.addEventListener("load", async _ => {
    const charbuf = [];
    const read_value = () => -1;
    const write_value = (i32) => charbuf.push(i32);

    const mod = await WebAssembly.compileStreaming(fetch("hello.wasm"));
    await WebAssembly.instantiate(mod, { io: { read_value, write_value } });
    console.log(charbuf);
    let result = "";
    charbuf.forEach(code => result += String.fromCharCode(code));
    console.log(result);
    console.log("done");
});