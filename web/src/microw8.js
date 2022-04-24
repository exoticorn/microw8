import loaderUrl from "data-url:../../platform/bin/loader.wasm";
import platformUrl from "data-url:../../platform/bin/platform.uw8";
import audioWorkletUrl from "data-url:./audiolet.js";

class AudioNode extends AudioWorkletNode {
    constructor(context) {
        super(context, 'apu', {outputChannelCount: [2]});
    }
}

let U8 = (...a) => new Uint8Array(...a);
let U32 = (...a) => new Uint32Array(...a);

export default function MicroW8(screen, config = {}) {
    if(!config.setMessage) {
        config.setMessage = (s, e) => {
            if(e) {
                console.log('error: ' + e);
            }
        }
    }
    let canvasCtx = screen.getContext('2d');
    let imageData = canvasCtx.createImageData(320, 240);
    
    let devkitMode = config.devkitMode;
    
    let cancelFunction;
    
    let currentData;
    
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
    
        let audioContext = new AudioContext({sampleRate: 44100});
        let keepRunning = true;
        let abortController = new AbortController();
        cancelFunction = () => {
            audioContext.close();
            keepRunning = false;
            abortController.abort();
        }

        let cartridgeSize = data.byteLength;
    
        config.setMessage(cartridgeSize);
        if (cartridgeSize == 0) {
            return;
        }
    
        await audioContext.audioWorklet.addModule(audioWorkletUrl);
        let audioNode = new AudioNode(audioContext);

        let audioReadyFlags = 0;
        let audioReadyResolve;
        let audioReadyPromise = new Promise(resolve => audioReadyResolve = resolve);
        let updateAudioReady = (f) => {
            audioReadyFlags |= f;
            if(audioReadyFlags == 3 && audioReadyResolve) {
                audioReadyResolve(true);
                audioReadyResolve = null;
            }
        };
        let audioStateChange = () => {
            if(audioContext.state == 'suspended') {
                if(config.startButton) {
                    config.startButton.style = '';
                    screen.style = 'display:none';
                }
                (config.startButton || screen).onclick = () => {
                    audioContext.resume();
                };
            } else {
                if(config.startButton) {
                    config.startButton.style = 'display:none';
                    screen.style = '';
                }
                updateAudioReady(1);
            }
        };
        audioContext.onstatechange = audioStateChange;
        audioStateChange();

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
            let memory = new WebAssembly.Memory(memSize);
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
    
            let loadModuleURL = async (url) => loadModuleData(await (await fetch(url)).arrayBuffer());
    
            loader = await instantiate(await loadModuleURL(loaderUrl));
    
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
    
            let platform_data = await loadModuleURL(platformUrl);

            audioNode.port.onmessage = (e) => updateAudioReady(e.data);
            audioNode.port.postMessage([platform_data, data]);

            let platform_instance = await instantiate(platform_data);
    
            for (let name in platform_instance.exports) {
                importObject.env[name] = platform_instance.exports[name]
            }
    
            let instance = await instantiate(data);
    
            let buffer = U32(imageData.data.buffer);

            await audioReadyPromise;
    
            let startTime = Date.now();
    
            const timePerFrame = 1000 / 60;
            let nextFrame = startTime;

            audioNode.connect(audioContext.destination);

            let isPaused = false;
            let pauseTime = startTime;
            let updateVisibility = isVisible => {
                let now = Date.now();
                if(isVisible) {
                    isPaused = false;
                    audioContext.resume();
                    startTime += now - pauseTime;
                    audioNode.port.postMessage(startTime);
                } else {
                    isPaused = true;
                    audioContext.suspend();
                    pauseTime = now;
                    audioNode.port.postMessage(0);
                }
            };
            window.addEventListener('focus', () => updateVisibility(true), { signal: abortController.signal });
            window.addEventListener('blur', () => updateVisibility(false), { signal: abortController.signal });
            updateVisibility(document.hasFocus());
    
            function mainloop() {
                if (!keepRunning) {
                    return;
                }
    
                try {
                    let restart = false;
                    if (!isPaused) {
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
                        u32Mem[16] = Date.now() - startTime;
                        u32Mem[17] = pad | gamepad;
                        if(instance.exports.upd) {
                            instance.exports.upd();
                        }
                        platform_instance.exports.endFrame();

                        let soundRegisters = new ArrayBuffer(32);
                        U8(soundRegisters).set(U8(memory.buffer, 80, 32));
                        audioNode.port.postMessage(soundRegisters, [soundRegisters]);
    
                        let palette = U32(memory.buffer, 0x13000, 1024);
                        for (let i = 0; i < 320 * 240; ++i) {
                            buffer[i] = palette[memU8[i + 120]] | 0xff000000;
                        }
                        canvasCtx.putImageData(imageData, 0, 0);
                    }
                    let now = Date.now();
                    nextFrame = Math.max(nextFrame + timePerFrame, now);
    
                    if (restart) {
                        runModule(currentData);
                    } else {
                        window.setTimeout(mainloop, Math.round(nextFrame - now))
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
        if((type && type.includes('html')) || response.status != 200) {
            return false;
        }
        runModule(await response.arrayBuffer(), keepUrl || devkitMode);
        return true;
    }

    return {
        runModule,
        runModuleFromURL,
        setDevkitMode: (m) => devkitMode = m,
    };
}
