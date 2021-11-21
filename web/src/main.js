import loaderUrl from "data-url:../../platform/loader.wasm";
import platformUrl from "data-url:../../platform/platform.wasm";

async function loadWasm(url, imports) {
    let wasm_module = await (await fetch(url)).arrayBuffer();
    let compiled_module = await WebAssembly.compile(wasm_module);

    return new WebAssembly.Instance(compiled_module, imports);
}

function setMessage(size, error) {
    let html = size ? `${size} bytes` : 'Insert cart';
    if (error) {
        html += ` - <span class="error">${error.replaceAll('<', '&lt;')}</span>`
    }
    document.getElementById('message').innerHTML = html;
}

let framebufferCanvas = document.createElement("canvas");
framebufferCanvas.width = 320;
framebufferCanvas.height = 240;
let framebufferCanvasCtx = framebufferCanvas.getContext("2d");
let imageData = framebufferCanvasCtx.createImageData(320, 240);
let screen = document.getElementById('screen');
let canvasCtx = screen.getContext('2d');

let cancelFunction;

async function runModule(data) {
    if (cancelFunction) {
        cancelFunction();
        cancelFunction = null;
    }

    let cartridgeSize = data.byteLength;

    setMessage(cartridgeSize);
    if (cartridgeSize == 0) {
        return;
    }

    let dataU8Array = new Uint8Array(data);

    let newURL = window.location.pathname;
    if (cartridgeSize <= 1024) {
        let dataString = '';
        for (let byte of dataU8Array) {
            dataString += String.fromCharCode(byte);
        }
        newURL += '#' + btoa(dataString);
    }
    if (newURL != window.location.pathname + window.location.hash) {
        history.pushState(null, null, newURL);
    }

    screen.width = screen.width;

    try {

        let loaderImport = {
            env: {
                memory: new WebAssembly.Memory({ initial: 9 })
            }
        };
        let loadMem = loaderImport.env.memory.buffer;
        let loader = await loadWasm(loaderUrl, loaderImport);

        if (dataU8Array[0] != 0) {

            new Uint8Array(loadMem).set(dataU8Array);

            let length = loader.exports.load_uw8(data.byteLength);

            data = new ArrayBuffer(length);
            new Uint8Array(data).set(new Uint8Array(loadMem).slice(0, length));
        }

        let importObject = {
            env: {
                memory: new WebAssembly.Memory({ initial: 4, maximum: 4 }),
            },
        };

        for (let n of ['acos', 'asin', 'atan', 'atan2', 'cos', 'exp', 'log', 'sin', 'tan', 'pow']) {
            importObject.env[n] = Math[n];
        }

        for (let i = 9; i < 64; ++i) {
            importObject.env['reserved' + i] = () => { };
        }

        for (let i = 0; i < 16; ++i) {
            importObject.env['g_reserved' + i] = 0;
        }

        let platform_instance = await loadWasm(platformUrl, importObject);

        for(let name in platform_instance.exports) {
            importObject.env[name] = platform_instance.exports[name]
        }

        let instance = new WebAssembly.Instance(await WebAssembly.compile(data), importObject);

        let buffer = new Uint32Array(imageData.data.buffer);

        let startTime = Date.now();

        let keepRunning = true;
        cancelFunction = () => keepRunning = false;

        function mainloop() {
            if (!keepRunning) {
                return;
            }

            try {
                instance.exports.tic(Date.now() - startTime);

                let framebuffer = new Uint8Array(importObject.env.memory.buffer.slice(120, 120 + 320 * 240));
                let palette = new Uint32Array(importObject.env.memory.buffer.slice(76920, 76920 + 1024));
                for (let i = 0; i < 320 * 240; ++i) {
                    buffer[i] = palette[framebuffer[i]] | 0xff000000;
                }
                framebufferCanvasCtx.putImageData(imageData, 0, 0);
                canvasCtx.imageSmoothingEnabled = false;
                canvasCtx.drawImage(framebufferCanvas, 0, 0, 640, 480);

                window.requestAnimationFrame(mainloop);
            } catch (err) {
                setMessage(cartridgeSize, err.toString());
            }
        }

        mainloop();
    } catch (err) {
        setMessage(cartridgeSize, err.toString());
    }
}

async function runModuleFromURL(url) {
    runModule(await (await fetch(url)).arrayBuffer());
}

function runModuleFromHash() {
    let base64Data = window.location.hash.slice(1);
    if (base64Data.length > 0) {
        runModuleFromURL('data:;base64,' + base64Data);
    } else {
        runModule(new ArrayBuffer(0));
    }
}

window.onhashchange = runModuleFromHash;
runModuleFromHash();

document.getElementById('cartButton').onclick = () => {
    let fileInput = document.createElement('input');
    fileInput.type = 'file';
    fileInput.accept = '.wasm,.uw8,application/wasm';
    fileInput.onchange = () => {
        if (fileInput.files.length > 0) {
            runModuleFromURL(URL.createObjectURL(fileInput.files[0]));
        }
    };
    fileInput.click();
};