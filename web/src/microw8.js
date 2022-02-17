import loaderUrl from "data-url:../../platform/bin/loader.wasm";
import platformUrl from "data-url:../../platform/bin/platform.uw8";

export default function MicroW8(screen, config) {
    let canvasCtx = screen.getContext('2d');
    let imageData = canvasCtx.createImageData(320, 240);
    
    let devkitMode = config.devkitMode;
    
    let cancelFunction;
    
    let currentData;
    
    let U8 = (d) => new Uint8Array(d);
    let U32 = (d) => new Uint32Array(d);
    
    let pad = 0;
    let keyboardElement = config.keyboardElement == undefined ? screen : config.keyboardElement;
    if(keyboardElement) {
        let keyHandler = (e) => {
            let isKeyDown = e.type == 'keydown';
            let mask;
            switch (e.code) {
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
                    if (isKeyDown) {
                        runModule(currentData, true);
                    }
                    break;
                case 'F9':
                    if(isKeyDown) {
                        screen.toBlob(blob => {
                            downloadBlob(blob, '.png');
                        });
                    }
                    e.preventDefault();
                    break;
                case 'F10':
                    if(isKeyDown) {
                        recordVideo();
                    }
                    e.preventDefault();
                    break;
            }
        
            if (isKeyDown) {
                pad |= mask;
            } else {
                pad &= ~mask;
            }
        };

        keyboardElement.onkeydown = keyHandler;
        keyboardElement.onkeyup = keyHandler;
    }
    
    async function runModule(data, keepUrl) {
        if (cancelFunction) {
            cancelFunction();
            cancelFunction = null;
        }
    
        let cartridgeSize = data.byteLength;
    
        config.setMessage(cartridgeSize);
        if (cartridgeSize == 0) {
            return;
        }
    
        currentData = data;
    
        let newURL = window.location.pathname;
        if (cartridgeSize <= 1024 && !keepUrl) {
            let dataString = '';
            for (let byte of U8(data)) {
                dataString += String.fromCharCode(byte);
            }
            newURL += '#' + btoa(dataString);
    
            if (newURL != window.location.pathname + window.location.hash) {
                history.pushState(null, null, newURL);
            }
        }
    
        screen.width = screen.width;
    
        try {
            let memSize = { initial: 4 };
            if(!devkitMode) {
                memSize.maximum = 4;
            }
            let memory = new WebAssembly.Memory({ initial: 4, maximum: devkitMode ? 16 : 4 });
            let memU8 = U8(memory.buffer);
    
            let importObject = {
                env: {
                    memory
                },
            };
    
            let loader;
    
            let loadModuleData = (data) => {
                if (loader && (!devkitMode || U8(data)[0] != 0)) {
                    memU8.set(U8(data));
                    let length = loader.exports.load_uw8(data.byteLength);
                    data = new ArrayBuffer(length);
                    U8(data).set(memU8.slice(0, length));
                }
                return data;
            }
    
            let instantiate = async (data) => (await WebAssembly.instantiate(data, importObject)).instance;
    
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
    
            for (let name in platform_instance.exports) {
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
                    let restart = false;
                    if (now >= nextFrame) {
                        let gamepads = navigator.getGamepads();
                        let gamepad = 0;
                        for (let i = 0; i < 4; ++i) {
                            let pad = gamepads[i];
                            if (!pad) {
                                continue;
                            }
                            for (let j = 0; j < 8; ++j) {
                                let buttonIdx = (j + 12) % 16;
                                if (pad.buttons.length > buttonIdx && pad.buttons[buttonIdx].pressed) {
                                    gamepad |= 1 << (i * 8 + j);
                                }
                            }
                            if (pad.axes.length > 1) {
                                for (let j = 0; j < 4; ++j) {
                                    let v = pad.axes[1 - (j >> 1)];
                                    if (((j & 1) ? v : -v) > 0.5) {
                                        gamepad |= 1 << (i * 8 + j);
                                    }
                                }
                            }
                            if (pad.buttons.length > 9 && pad.buttons[9].pressed) {
                                restart = true;
                            }
                        }
    
                        let u32Mem = U32(memory.buffer);
                        u32Mem[16] = now - startTime;
                        u32Mem[17] = pad | gamepad;
                        instance.exports.upd();
                        platform_instance.exports.endFrame();
    
                        let palette = U32(memory.buffer.slice(0x13000, 0x13000 + 1024));
                        for (let i = 0; i < 320 * 240; ++i) {
                            buffer[i] = palette[memU8[i + 120]] | 0xff000000;
                        }
                        canvasCtx.putImageData(imageData, 0, 0);
                        nextFrame = Math.max(nextFrame + timePerFrame, now);
                    }
    
                    if (restart) {
                        runModule(currentData);
                    } else {
                        window.requestAnimationFrame(mainloop);
                    }
                } catch (err) {
                    config.setMessage(cartridgeSize, err.toString());
                }
            }
    
            mainloop();
        } catch (err) {
            config.setMessage(cartridgeSize, err.toString());
        }
    }
    
    function downloadBlob(blob, ext) {
        let a = document.createElement('a');
        a.href = URL.createObjectURL(blob);
        a.download = 'microw8_' + new Date().toISOString() + ext;
        a.click();
        URL.revokeObjectURL(a.href);
    }
    
    let videoRecorder;
    let videoStartTime;
    function recordVideo() {
        if(videoRecorder) {
            videoRecorder.stop();
            videoRecorder = null;
            return;
        }
    
        videoRecorder = new MediaRecorder(screen.captureStream(), {
            mimeType: 'video/webm',
            videoBitsPerSecond: 25000000
        });
    
        let chunks = [];
        videoRecorder.ondataavailable = e => {
            chunks.push(e.data);
        };
    
        let timer = config.timerElement;
        if(timer) {
            timer.hidden = false;
            timer.innerText = "00:00";
        }
    
        videoRecorder.onstop = () => {
            if(timer) {
                timer.hidden = true;
            }
            downloadBlob(new Blob(chunks, {type: 'video/webm'}), '.webm');
        };
    
        videoRecorder.start();
        videoStartTime = Date.now();
    
        function updateTimer() {
            if(!videoStartTime) {
                return;
            }
    
            if(timer) {
                let duration = Math.floor((Date.now() - videoStartTime) / 1000);
                timer.innerText = Math.floor(duration / 60).toString().padStart(2, '0') + ':' + (duration % 60).toString().padStart(2, '0');
            }
    
            setTimeout(updateTimer, 1000);
        }
    
        setTimeout(updateTimer, 1000);
    }
    
    async function runModuleFromURL(url, keepUrl) {
        let response = await fetch(url);
        let type = response.headers.get('Content-Type');
        if(type && type.includes('html')) {
            throw false;
        }
        runModule(await response.arrayBuffer(), keepUrl || devkitMode);
    }

    return {
        runModule,
        runModuleFromURL,
        setDevkitMode: (m) => devkitMode = m,
    };
}

this.uw8 = MicroW8;