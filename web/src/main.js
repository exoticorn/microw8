import loaderUrl from "data-url:../../platform/bin/loader.wasm";
import platformUrl from "data-url:../../platform/bin/platform.uw8";

function setMessage(size, error) {
    let html = size ? `${size} bytes` : 'Insert cart';
    if (error) {
        html += ` - <span class="error">${error.replaceAll('<', '&lt;')}</span>`
    }
    document.getElementById('message').innerHTML = html;
}

let screen = document.getElementById('screen');
let canvasCtx = screen.getContext('2d');
let imageData = canvasCtx.createImageData(320, 240);

let cancelFunction;

let currentData;

let U8 = (d) => new Uint8Array(d);
let U32 = (d) => new Uint32Array(d);

let pad = 0;
let keyHandler = (e) => {
    let isKeyDown = e.type == 'keydown';
    let mask;
    switch(e.code) {
    case 'ArrowUp':
        mask = 1;
        break;
    case 'ArrowDown':
        mask = 2;
        break;
    case 'ArrowLeft':
        mask = 4;
        break;
    case 'ArrowRight':
        mask = 8;
        break;
    case 'KeyZ':
        mask = 16;
        break;
    case 'KeyX':
        mask = 32;
        break;
    case 'KeyA':
        mask = 64;
        break;
    case 'KeyS':
        mask = 128;
        break;
    case 'KeyR':
        if(isKeyDown)
        {
            runModule(currentData);
        }
        break;
    }

    if(isKeyDown) {
        pad |= mask;
    } else {
        pad &= ~mask;
    }
};
window.onkeydown = keyHandler;
window.onkeyup = keyHandler;

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

    currentData = data;

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

        let buffer = U32(imageData.data.buffer);

        let startTime = Date.now();

        let keepRunning = true;
        cancelFunction = () => keepRunning = false;

        const timePerFrame = 1000 / 60;
        let nextFrame = startTime;

        function mainloop() {
            if (!keepRunning) {
                return;
            }

            try {
                let now = Date.now();
                if(now >= nextFrame) {
                    let u32Mem = U32(memory.buffer);
                    u32Mem[16] = now - startTime;
                    u32Mem[17] = pad;
                    instance.exports.upd();

                    let palette = U32(memory.buffer.slice(0x13000, 0x13000 + 1024));
                    for (let i = 0; i < 320 * 240; ++i) {
                        buffer[i] = palette[memU8[i + 120]] | 0xff000000;
                    }
                    canvasCtx.putImageData(imageData, 0, 0);
                    nextFrame = Math.max(nextFrame + timePerFrame, now);
                }

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