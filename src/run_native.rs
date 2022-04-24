use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;
use std::{thread, time::Instant};

use anyhow::{anyhow, Result};
use cpal::traits::*;
use minifb::{Key, Window, WindowOptions};
use wasmtime::{
    Engine, GlobalType, Memory, MemoryType, Module, Mutability, Store, TypedFunc, ValType,
};

static GAMEPAD_KEYS: &[Key] = &[
    Key::Up,
    Key::Down,
    Key::Left,
    Key::Right,
    Key::Z,
    Key::X,
    Key::A,
    Key::S,
];

pub struct MicroW8 {
    engine: Engine,
    loader_module: Module,
    window: Window,
    window_buffer: Vec<u32>,
    instance: Option<UW8Instance>,
    timeout: u32,
}

struct UW8Instance {
    store: Store<()>,
    memory: Memory,
    end_frame: TypedFunc<(), ()>,
    update: Option<TypedFunc<(), ()>>,
    start_time: Instant,
    module: Vec<u8>,
    watchdog: Arc<Mutex<UW8WatchDog>>,
    sound: Uw8Sound,
}

impl Drop for UW8Instance {
    fn drop(&mut self) {
        if let Ok(mut watchdog) = self.watchdog.lock() {
            watchdog.stop = true;
        }
    }
}

struct UW8WatchDog {
    interupt: wasmtime::InterruptHandle,
    timeout: u32,
    stop: bool,
}

impl MicroW8 {
    pub fn new() -> Result<MicroW8> {
        let engine = wasmtime::Engine::new(wasmtime::Config::new().interruptable(true))?;

        let loader_module =
            wasmtime::Module::new(&engine, include_bytes!("../platform/bin/loader.wasm"))?;

        let options = WindowOptions {
            scale: minifb::Scale::X2,
            scale_mode: minifb::ScaleMode::AspectRatioStretch,
            resize: true,
            ..Default::default()
        };
        let mut window = Window::new("MicroW8", 320, 240, options)?;
        window.limit_update_rate(Some(std::time::Duration::from_micros(16666)));

        Ok(MicroW8 {
            engine,
            loader_module,
            window,
            window_buffer: vec![0u32; 320 * 240],
            instance: None,
            timeout: 30,
        })
    }

    fn reset(&mut self) {
        self.instance = None;
        for v in &mut self.window_buffer {
            *v = 0;
        }
    }
}

struct Uw8Sound {
    stream: cpal::Stream,
    tx: mpsc::SyncSender<[u8; 32]>,
}

impl super::Runtime for MicroW8 {
    fn is_open(&self) -> bool {
        self.window.is_open() && !self.window.is_key_down(Key::Escape)
    }

    fn set_timeout(&mut self, timeout: u32) {
        self.timeout = timeout;
    }

    fn load(&mut self, module_data: &[u8]) -> Result<()> {
        self.reset();

        let mut store = wasmtime::Store::new(&self.engine, ());

        let memory = wasmtime::Memory::new(&mut store, MemoryType::new(4, Some(4)))?;

        let mut linker = wasmtime::Linker::new(&self.engine);
        linker.define("env", "memory", memory)?;

        let loader_instance = linker.instantiate(&mut store, &self.loader_module)?;
        let load_uw8 = loader_instance.get_typed_func::<i32, i32, _>(&mut store, "load_uw8")?;

        let platform_data = include_bytes!("../platform/bin/platform.uw8");
        memory.data_mut(&mut store)[..platform_data.len()].copy_from_slice(platform_data);
        let platform_length =
            load_uw8.call(&mut store, platform_data.len() as i32)? as u32 as usize;
        let platform_module =
            wasmtime::Module::new(&self.engine, &memory.data(&store)[..platform_length])?;

        memory.data_mut(&mut store)[..module_data.len()].copy_from_slice(module_data);
        let module_length = load_uw8.call(&mut store, module_data.len() as i32)? as u32 as usize;
        let module = wasmtime::Module::new(&self.engine, &memory.data(&store)[..module_length])?;

        fn add_native_functions(
            linker: &mut wasmtime::Linker<()>,
            store: &mut wasmtime::Store<()>,
        ) -> Result<()> {
            linker.func_wrap("env", "acos", |v: f32| v.acos())?;
            linker.func_wrap("env", "asin", |v: f32| v.asin())?;
            linker.func_wrap("env", "atan", |v: f32| v.atan())?;
            linker.func_wrap("env", "atan2", |x: f32, y: f32| x.atan2(y))?;
            linker.func_wrap("env", "cos", |v: f32| v.cos())?;
            linker.func_wrap("env", "exp", |v: f32| v.exp())?;
            linker.func_wrap("env", "log", |v: f32| v.ln())?;
            linker.func_wrap("env", "sin", |v: f32| v.sin())?;
            linker.func_wrap("env", "tan", |v: f32| v.tan())?;
            linker.func_wrap("env", "pow", |a: f32, b: f32| a.powf(b))?;
            for i in 10..64 {
                linker.func_wrap("env", &format!("reserved{}", i), || {})?;
            }
            for i in 0..16 {
                linker.define(
                    "env",
                    &format!("g_reserved{}", i),
                    wasmtime::Global::new(
                        &mut *store,
                        GlobalType::new(ValType::I32, Mutability::Const),
                        0.into(),
                    )?,
                )?;
            }

            Ok(())
        }

        add_native_functions(&mut linker, &mut store)?;

        fn instantiate_platform(
            linker: &mut wasmtime::Linker<()>,
            store: &mut wasmtime::Store<()>,
            platform_module: &wasmtime::Module,
        ) -> Result<wasmtime::Instance> {
            let platform_instance = linker.instantiate(&mut *store, &platform_module)?;

            for export in platform_instance.exports(&mut *store) {
                linker.define(
                    "env",
                    export.name(),
                    export
                        .into_func()
                        .expect("platform surely only exports functions"),
                )?;
            }

            Ok(platform_instance)
        }

        let platform_instance = instantiate_platform(&mut linker, &mut store, &platform_module)?;

        let watchdog = Arc::new(Mutex::new(UW8WatchDog {
            interupt: store.interrupt_handle()?,
            timeout: self.timeout,
            stop: false,
        }));

        {
            let watchdog = watchdog.clone();
            thread::spawn(move || loop {
                thread::sleep(Duration::from_millis(17));
                if let Ok(mut watchdog) = watchdog.lock() {
                    if watchdog.stop {
                        break;
                    }
                    if watchdog.timeout > 0 {
                        watchdog.timeout -= 1;
                        if watchdog.timeout == 0 {
                            watchdog.interupt.interrupt();
                        }
                    }
                } else {
                    break;
                }
            });
        }

        let instance = linker.instantiate(&mut store, &module)?;
        if let Ok(mut watchdog) = watchdog.lock() {
            watchdog.timeout = 0;
        }
        let end_frame = platform_instance.get_typed_func::<(), (), _>(&mut store, "endFrame")?;
        let update = instance.get_typed_func::<(), (), _>(&mut store, "upd").ok();

        let sound = {
            let mut store = wasmtime::Store::new(&self.engine, ());

            let memory = wasmtime::Memory::new(&mut store, MemoryType::new(4, Some(4)))?;

            let mut linker = wasmtime::Linker::new(&self.engine);
            linker.define("env", "memory", memory)?;
            add_native_functions(&mut linker, &mut store)?;

            let platform_instance =
                instantiate_platform(&mut linker, &mut store, &platform_module)?;
            let instance = linker.instantiate(&mut store, &module)?;

            let snd = instance
                .get_typed_func::<(i32,), f32, _>(&mut store, "snd")
                .or_else(|_| {
                    platform_instance.get_typed_func::<(i32,), f32, _>(&mut store, "gesSnd")
                })?;

            let host = cpal::default_host();
            let device = host
                .default_output_device()
                .ok_or_else(|| anyhow!("No audio output device available"))?;
            let config = device
                .supported_output_configs()?
                .find(|config| {
                    config.min_sample_rate().0 <= 44100
                        && config.max_sample_rate().0 >= 44100
                        && config.channels() == 2
                        && config.sample_format() == cpal::SampleFormat::F32
                })
                .ok_or_else(|| anyhow!("Could not find 44.1kHz float config"))?;
            let config = config.with_sample_rate(cpal::SampleRate(44100));
            let buffer_size = match *config.buffer_size() {
                cpal::SupportedBufferSize::Unknown => cpal::BufferSize::Default,
                cpal::SupportedBufferSize::Range { min, max } => {
                    cpal::BufferSize::Fixed(256.max(min).min(max))
                }
            };
            let config = cpal::StreamConfig {
                buffer_size,
                ..config.config()
            };

            let (tx, rx) = mpsc::sync_channel::<[u8; 32]>(1);

            let start_time = Instant::now();

            let mut sample_index = 0;
            let stream = {
                device.build_output_stream(
                    &config,
                    move |buffer: &mut [f32], _| {
                        if let Ok(regs) = rx.try_recv() {
                            memory.write(&mut store, 80, &regs).unwrap();
                        }

                        {
                            let time = start_time.elapsed().as_millis() as i32;
                            let mem = memory.data_mut(&mut store);
                            mem[64..68].copy_from_slice(&time.to_le_bytes());
                        }

                        for v in buffer {
                            *v = snd.call(&mut store, (sample_index,)).unwrap_or(0.0);
                            sample_index = sample_index.wrapping_add(1);
                        }
                    },
                    move |err| {
                        dbg!(err);
                    },
                )?
            };

            Uw8Sound { stream, tx }
        };

        sound.stream.play()?;

        self.instance = Some(UW8Instance {
            store,
            memory,
            end_frame,
            update,
            start_time: Instant::now(),
            module: module_data.into(),
            watchdog,
            sound,
        });

        Ok(())
    }

    fn run_frame(&mut self) -> Result<()> {
        let mut result = Ok(());
        if let Some(mut instance) = self.instance.take() {
            {
                let time = instance.start_time.elapsed().as_millis() as i32;
                let mut gamepad: u32 = 0;
                for key in self.window.get_keys() {
                    if let Some(index) = GAMEPAD_KEYS
                        .iter()
                        .enumerate()
                        .find(|(_, &k)| k == key)
                        .map(|(i, _)| i)
                    {
                        gamepad |= 1 << index;
                    }
                }

                let mem = instance.memory.data_mut(&mut instance.store);
                mem[64..68].copy_from_slice(&time.to_le_bytes());
                mem[68..72].copy_from_slice(&gamepad.to_le_bytes());
            }

            if let Ok(mut watchdog) = instance.watchdog.lock() {
                watchdog.timeout = self.timeout;
            }
            if let Some(ref update) = instance.update {
                result = update.call(&mut instance.store, ());
            }
            if let Ok(mut watchdog) = instance.watchdog.lock() {
                watchdog.timeout = 0;
            }
            instance.end_frame.call(&mut instance.store, ())?;

            let memory = instance.memory.data(&instance.store);

            let mut sound_regs = [0u8; 32];
            sound_regs.copy_from_slice(&memory[80..112]);
            instance.sound.tx.send(sound_regs)?;

            let framebuffer = &memory[120..(120 + 320 * 240)];
            let palette = &memory[0x13000..];
            for (i, &color_index) in framebuffer.iter().enumerate() {
                let offset = color_index as usize * 4;
                self.window_buffer[i] = 0xff000000
                    | ((palette[offset] as u32) << 16)
                    | ((palette[offset + 1] as u32) << 8)
                    | palette[offset + 2] as u32;
            }

            if self.window.is_key_pressed(Key::R, minifb::KeyRepeat::No) {
                self.load(&instance.module)?;
            } else if result.is_ok() {
                self.instance = Some(instance);
            }
        }

        self.window
            .update_with_buffer(&self.window_buffer, 320, 240)?;

        result?;
        Ok(())
    }
}
