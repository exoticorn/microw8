let U8 = (...a) => new Uint8Array(...a);
class APU extends AudioWorkletProcessor {
    constructor() {
        super();
        this.sampleIndex = 0;
        this.currentTime = 0;
        this.isFirstMessage = true;
        this.pendingUpdates = [];
        this.port.onmessage = (ev) => {
            if(this.memory) {
                if(this.isFirstMessage)
                {
                    this.currentTime += (ev.data.t - this.currentTime) / 8;
                    this.isFirstMessage = false;
                }
                this.pendingUpdates.push(ev.data);
            } else {
                this.load(ev.data[0], ev.data[1]);
            }
        };
    }

    async load(platform_data, data) {
        let memory = new WebAssembly.Memory({ initial: 4, maximum: 4 });

        let importObject = {
            env: {
                memory
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

        let instantiate = async (data) => (await WebAssembly.instantiate(data, importObject)).instance;

        let platform_instance = await instantiate(platform_data);

        for (let name in platform_instance.exports) {
            importObject.env[name] = platform_instance.exports[name]
        }

        let instance = await instantiate(data);

        this.memory = memory;

        this.snd = instance.exports.snd || platform_instance.exports.gesSnd;

        this.port.postMessage(2);
    }

    process(inputs, outputs, parameters) {
        this.isFirstMessage = true;
        if(this.snd) {
            while(this.pendingUpdates.length > 0 && this.pendingUpdates[0].t <= this.currentTime) {
                U8(this.memory.buffer, 80, 32).set(U8(this.pendingUpdates.shift().r));
            }
            let u32Mem = new Uint32Array(this.memory.buffer);
            u32Mem[16] = this.currentTime;
            let channels = outputs[0];
            let index = this.sampleIndex;
            let numSamples = channels[0].length;
            for(let i = 0; i < numSamples; ++i) {
                channels[0][i] = this.snd(index++);
                channels[1][i] = this.snd(index++);
            }
            this.sampleIndex = index & 0xffffffff;
            this.currentTime += numSamples / 44.1;
        }

        return true;
    }
}

registerProcessor('apu', APU);