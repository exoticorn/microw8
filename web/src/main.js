import loaderUrl from "data-url:../../platform/loader.wasm";
import platformUrl from "data-url:../../platform/platform.wasm";

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

let U8 = (d) => new Uint8Array(d);

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

    let newURL = window.location.pathname;
    if (cartridgeSize <= 1024) {
        let dataString = '';
        for (let byte of U8(data)) {
            dataString += String.fromCharCode(byte);
        }
        newURL += '#' + btoa(dataString);
    }
    if (newURL != window.location.pathname + window.location.hash) {
        history.pushState(null, null, newURL);
    }

    screen.width = screen.width;

    try {
        let memory = new WebAssembly.Memory({ initial: 4, maximum: 4 });
        let memU8 = U8(memory.buffer);

        let importObject = {
            env: {
                memory
            },
        };

        let loader;

        let loadModuleData = (data) => {
            if (U8(data)[0] != 0) {
                memU8.set(U8(data));
                let length = loader.exports.load_uw8(data.byteLength);
                data = new ArrayBuffer(length);
                U8(data).set(memU8.slice(0, length));
            }
            return data;
        }

        let instantiate = async (data) => new WebAssembly.Instance(await WebAssembly.compile(data), importObject);

        let loadModuleURL = async (url) => instantiate(loadModuleData(await (await fetch(url)).arrayBuffer()));

        loader = await loadModuleURL(loaderUrl);

        for (let n of ['acos', 'asin', 'atan', 'atan2', 'cos', 'exp', 'log', 'sin', 'tan', 'pow']) {
            importObject.env[n] = Math[n];
        }

        for (let i = 9; i < 64; ++i) {
            importObject.env['reserved' + i] = () => { };
        }

        for (let i = 0; i < 16; ++i) {
            importObject.env['g_reserved' + i] = 0;
        }

        data = loadModuleData(data);

        let platform_instance = await loadModuleURL(platformUrl);

        for(let name in platform_instance.exports) {
            importObject.env[name] = platform_instance.exports[name]
        }

        let instance = await instantiate(data);

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

                let palette = new Uint32Array(memory.buffer.slice(76920, 76920 + 1024));
                for (let i = 0; i < 320 * 240; ++i) {
                    buffer[i] = palette[memU8[i + 120]] | 0xff000000;
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