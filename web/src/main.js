import loaderUrl from "data-url:../../platform/loader.wasm";
import baseUrl from "data-url:../../platform/base.wasm";

async function loadWasm(url, imports) {
    let wasm_module = await (await fetch(url)).arrayBuffer();
    let compiled_module = await WebAssembly.compile(wasm_module);

    return new WebAssembly.Instance(compiled_module, imports);
}

function setMessage(size, error) {
    let html = size ? `${size} bytes` : 'Insert cart';
    if(error) {
        html += ` - <span class="error">${error.replaceAll('<', '&lt;')}</span>`
    }
    document.getElementById('message').innerHTML = html;
}

let framebufferCanvas = document.createElement("canvas");
framebufferCanvas.width = 320;
framebufferCanvas.height = 256;
let framebufferCanvasCtx = framebufferCanvas.getContext("2d");
let imageData = framebufferCanvasCtx.createImageData(320, 256);
let screen = document.getElementById('screen');
let canvasCtx = screen.getContext('2d');

let cancelFunction;

async function runModule(data) {
    if(cancelFunction) {
        cancelFunction();
        cancelFunction = null;
    }

    let cartridgeSize = data.byteLength;

    setMessage(cartridgeSize);
    if(cartridgeSize == 0) {
        return;
    }

    let dataU8Array = new Uint8Array(data);

    let newURL = window.location.pathname;
    if(cartridgeSize <= 1024) {
        let dataString = '';
        for(let byte of dataU8Array) {
            dataString += String.fromCharCode(byte);
        }
        newURL += '#' + btoa(dataString);
    }
    if(newURL != window.location.pathname + window.location.hash) {
        history.pushState(null, null, newURL);
    }

    screen.width = screen.width;

    try {

        let loaderImport = {
            env: {
                memory: new WebAssembly.Memory({ initial: 8 })
            }
        };
        let loadMem = loaderImport.env.memory.buffer;
        let loader = await loadWasm(loaderUrl, loaderImport);
    
        let baseModule = await (await fetch(baseUrl)).arrayBuffer();
    
        if(dataU8Array[0] != 0) {
            new Uint8Array(loadMem).set(dataU8Array);
            new Uint8Array(loadMem).set(new Uint8Array(baseModule), data.byteLength);
    
            let destOffset = data.byteLength + baseModule.byteLength;
            let endOffset = loader.exports.load_uw8(0, data.byteLength, data.byteLength, destOffset);
    
            data = new ArrayBuffer(endOffset - destOffset);
            new Uint8Array(data).set(new Uint8Array(loadMem).slice(destOffset, endOffset));
        }
    
        let importObject = {
            env: {
                memory: new WebAssembly.Memory({ initial: 8, maximum: 8 }),
            },
            math: {
                sin: Math.sin,
                cos: Math.cos
            }
        };
    
        let instance = new WebAssembly.Instance(await WebAssembly.compile(data), importObject);
    
        let buffer = imageData.data;
    
        let startTime = Date.now();
    
        let keepRunning = true;
        cancelFunction = () => keepRunning = false;

        function mainloop() {
            if(!keepRunning) {
                return;
            }

            try {
                instance.exports.tic(Date.now() - startTime);
    
                let framebuffer = new Uint8Array(importObject.env.memory.buffer.slice(120, 120 + 320*256));
                for(let i = 0; i < 320*256; ++i) {
                    buffer[i * 4] = framebuffer[i];
                    buffer[i * 4 + 1] = framebuffer[i];
                    buffer[i * 4 + 2] = framebuffer[i];
                    buffer[i * 4 + 3] = 255;
                }
                framebufferCanvasCtx.putImageData(imageData, 0, 0);
                canvasCtx.imageSmoothingEnabled = false;
                canvasCtx.drawImage(framebufferCanvas, 0, 0, 640, 512);
        
                window.requestAnimationFrame(mainloop);
            } catch(err) {
                setMessage(cartridgeSize, err.toString());
            }
        }
    
        mainloop();
    } catch(err) {
        setMessage(cartridgeSize, err.toString());
    }
}

async function runModuleFromURL(url) {
    runModule(await (await fetch(url)).arrayBuffer());
}

function runModuleFromHash() {
    let base64Data = window.location.hash.slice(1);
    if(base64Data.length > 0) {
        runModuleFromURL('data:;base64,' + base64Data);
    } else {
        runModule(new ArrayBuffer(0));
    }
}

let fileInput = document.getElementById('cart');
fileInput.onchange = () => {
    if(fileInput.files.length > 0) {
        runModuleFromURL(URL.createObjectURL(fileInput.files[0]));
    }
};

window.onhashchange = runModuleFromHash;
runModuleFromHash();