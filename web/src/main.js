import wasmDataUrl from "data-url:./trainride.uw8";

async function main() {
    let wasm_module = await (await fetch(wasmDataUrl)).arrayBuffer();
    let compiled_module = await WebAssembly.compile(wasm_module);

    let import_object = {
        uw8: {
            ram: new WebAssembly.Memory({ initial: 8, maximum: 8 }),
            time: new WebAssembly.Global({value: 'i32', mutable: true}, 0),
        }
    };

    let instance = new WebAssembly.Instance(compiled_module, import_object);

    let canvasCtx = document.getElementById('screen').getContext('2d');
    let imageData = canvasCtx.createImageData(320, 256);

    let buffer = imageData.data;
    for(let i = 0; i < 320*256; ++i) {
        buffer[i * 4 + 3] = 255;
    }

    let startTime = Date.now();

    function mainloop() {
        import_object.uw8.time.value = Date.now() - startTime;

        instance.exports.tic();

        let framebuffer = new Uint8Array(import_object.uw8.ram.buffer.slice(120, 120 + 320*256));
        for(let i = 0; i < 320*256; ++i) {
            buffer[i * 4] = framebuffer[i];
            buffer[i * 4 + 1] = framebuffer[i];
            buffer[i * 4 + 2] = framebuffer[i];
        }
        canvasCtx.putImageData(imageData, 0, 0);

        window.requestAnimationFrame(mainloop);
    }

    mainloop();
}

main();