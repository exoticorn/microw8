import loaderUrl from "data-url:./uw8loader.wasm";
import baseUrl from "data-url:./base.wasm";

async function loadWasm(url, imports) {
    let wasm_module = await (await fetch(url)).arrayBuffer();
    let compiled_module = await WebAssembly.compile(wasm_module);

    return new WebAssembly.Instance(compiled_module, imports);
}

let cancelFunction;

async function runModule(data) {
    if(cancelFunction) {
        cancelFunction();
        cancelFunction = null;
    }

    document.getElementById('message').innerText = '' + data.byteLength + ' bytes';

    let loaderImport = {
        uw8: {
            ram: new WebAssembly.Memory({ initial: 8 })
        }
    };
    let loadMem = loaderImport.uw8.ram.buffer;
    let loader = await loadWasm(loaderUrl, loaderImport);

    new Uint8Array(loadMem).set(new Uint8Array(data));

    let baseModule = await (await fetch(baseUrl)).arrayBuffer();
    new Uint8Array(loadMem).set(new Uint8Array(baseModule), data.byteLength);

    let destOffset = data.byteLength + baseModule.byteLength;
    let endOffset = loader.exports.load_uw8(0, data.byteLength, data.byteLength, destOffset);

    data = new ArrayBuffer(endOffset - destOffset);
    new Uint8Array(data).set(new Uint8Array(loadMem).slice(destOffset, endOffset));

    let importObject = {
        uw8: {
            ram: new WebAssembly.Memory({ initial: 8, maximum: 8 }),
            time: new WebAssembly.Global({value: 'i32', mutable: true}, 0),
        }
    };

    let instance = new WebAssembly.Instance(await WebAssembly.compile(data), importObject);

    let canvasCtx = document.getElementById('screen').getContext('2d');
    let imageData = canvasCtx.createImageData(320, 256);

    let buffer = imageData.data;
    for(let i = 0; i < 320*256; ++i) {
        buffer[i * 4 + 3] = 255;
    }

    let startTime = Date.now();

    let keepRunning = true;
    cancelFunction = () => keepRunning = false;

    function mainloop() {
        if(!keepRunning) {
            return;
        }
        importObject.uw8.time.value = Date.now() - startTime;

        instance.exports.tic();

        let framebuffer = new Uint8Array(importObject.uw8.ram.buffer.slice(120, 120 + 320*256));
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

async function runModuleFromURL(url) {
    runModule(await (await fetch(url)).arrayBuffer());
}

function runModuleFromHash() {
    runModuleFromURL('data:;base64,' + window.location.hash.slice(1));
}

window.onhashchange = runModuleFromHash;
runModuleFromHash();