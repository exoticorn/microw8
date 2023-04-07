use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;
use std::{thread, time::Instant};

use anyhow::{anyhow, Result};
use cpal::traits::*;
use rubato::Resampler;
use uw8_window::{Window, WindowConfig};
use wasmtime::{
    Engine, GlobalType, Memory, MemoryType, Module, Mutability, Store, TypedFunc, ValType,
};

pub struct MicroW8 {
    window: Window,
    stream: Option<cpal::Stream>,
    engine: Engine,
    loader_module: Module,
    disable_audio: bool,
    module_data: Option<Vec<u8>>,
    timeout: u32,
    instance: Option<UW8Instance>,
}

struct UW8Instance {
    store: Store<()>,
    memory: Memory,
    end_frame: TypedFunc<(), ()>,
    update: Option<TypedFunc<(), ()>>,
    start_time: Instant,
    watchdog: Arc<Mutex<UW8WatchDog>>,
    sound_tx: Option<mpsc::SyncSender<RegisterUpdate>>,
}

impl Drop for UW8Instance {
    fn drop(&mut self) {
        if let Ok(mut watchdog) = self.watchdog.lock() {
            watchdog.stop = true;
        }
    }
}

struct UW8WatchDog {
    engine: Engine,
    stop: bool,
}

impl MicroW8 {
    pub fn new(timeout: Option<u32>, window_config: WindowConfig) -> Result<MicroW8> {
        let mut config = wasmtime::Config::new();
        config.cranelift_opt_level(wasmtime::OptLevel::Speed);
        if timeout.is_some() {
            config.epoch_interruption(true);
        }
        let engine = wasmtime::Engine::new(&config)?;

        let loader_module =
            wasmtime::Module::new(&engine, include_bytes!("../platform/bin/loader.wasm"))?;

        let window = Window::new(window_config)?;

        Ok(MicroW8 {
            window,
            stream: None,
            engine,
            loader_module,
            disable_audio: false,
            module_data: None,
            timeout: timeout.unwrap_or(0),
            instance: None,
        })
    }

    pub fn disable_audio(&mut self) {
        self.disable_audio = true;
    }
}

impl super::Runtime for MicroW8 {
    fn is_open(&self) -> bool {
        self.window.is_open()
    }

    fn load(&mut self, module_data: &[u8]) -> Result<()> {
        self.stream = None;
        self.instance = None;

        let mut store = wasmtime::Store::new(&self.engine, ());
        store.set_epoch_deadline(60);

        let memory = wasmtime::Memory::new(&mut store, MemoryType::new(4, Some(4)))?;

        let mut linker = wasmtime::Linker::new(&self.engine);
        linker.define("env", "memory", memory)?;

        let loader_instance = linker.instantiate(&mut store, &self.loader_module)?;
        let load_uw8 = loader_instance.get_typed_func::<i32, i32>(&mut store, "load_uw8")?;

        let platform_data = include_bytes!("../platform/bin/platform.uw8");
        memory.data_mut(&mut store)[..platform_data.len()].copy_from_slice(platform_data);
        let platform_length =
            load_uw8.call(&mut store, platform_data.len() as i32)? as u32 as usize;
        let platform_module =
            wasmtime::Module::new(&self.engine, &memory.data(&store)[..platform_length])?;

        memory.data_mut(&mut store)[..module_data.len()].copy_from_slice(module_data);
        let module_length = load_uw8.call(&mut store, module_data.len() as i32)? as u32 as usize;
        let module = wasmtime::Module::new(&self.engine, &memory.data(&store)[..module_length])?;

        add_native_functions(&mut linker, &mut store)?;

        let platform_instance = instantiate_platform(&mut linker, &mut store, &platform_module)?;

        let watchdog = Arc::new(Mutex::new(UW8WatchDog {
            engine: self.engine.clone(),
            stop: false,
        }));

        {
            let watchdog = watchdog.clone();
            thread::spawn(move || loop {
                thread::sleep(Duration::from_millis(17));
                if let Ok(watchdog) = watchdog.lock() {
                    if watchdog.stop {
                        break;
                    }
                    watchdog.engine.increment_epoch();
                } else {
                    break;
                }
            });
        }

        let instance = linker.instantiate(&mut store, &module)?;
        let end_frame = platform_instance.get_typed_func::<(), ()>(&mut store, "endFrame")?;
        let update = instance.get_typed_func::<(), ()>(&mut store, "upd").ok();

        if let Some(start) = instance.get_typed_func::<(), ()>(&mut store, "start").ok() {
            start.call(&mut store, ())?;
        }

        let (sound_tx, stream) = if self.disable_audio {
            (None, None)
        } else {
            match init_sound(&self.engine, &platform_module, &module) {
                Ok(sound) => {
                    sound.stream.play()?;
                    (Some(sound.tx), Some(sound.stream))
                }
                Err(err) => {
                    eprintln!("Failed to init sound: {}", err);
                    (None, None)
                }
            }
        };

        self.instance = Some(UW8Instance {
            store,
            memory,
            end_frame,
            update,
            start_time: Instant::now(),
            watchdog,
            sound_tx,
        });
        self.stream = stream;
        self.module_data = Some(module_data.into());
        Ok(())
    }

    fn run_frame(&mut self) -> Result<()> {
        let input = self.window.begin_frame();

        if input.reset {
            if let Some(module_data) = self.module_data.take() {
                self.load(&module_data)?;
            }
        }

        let now = Instant::now();
        let mut result = Ok(());
        if let Some(mut instance) = self.instance.take() {
            let time = (now - instance.start_time).as_millis() as i32;
            let next_frame = {
                let frame = (time as u32 as u64 * 6 / 100) as u32;
                let cur_offset = (time as u32).wrapping_sub((frame as u64 * 100 / 6) as u32);
                let next_time =
                    ((frame as u64 + 1) * 100 / 6 + cur_offset.max(1).min(4) as u64) as u32;
                let offset = next_time.wrapping_sub(time as u32);
                now + Duration::from_millis(offset as u64)
            };

            {
                let mem = instance.memory.data_mut(&mut instance.store);
                mem[64..68].copy_from_slice(&time.to_le_bytes());
                mem[68..72].copy_from_slice(&input.gamepads);
            }

            instance.store.set_epoch_deadline(self.timeout as u64);
            if let Some(ref update) = instance.update {
                if let Err(err) = update.call(&mut instance.store, ()) {
                    result = Err(err);
                }
            }
            instance.end_frame.call(&mut instance.store, ())?;

            let memory = instance.memory.data(&instance.store);

            let mut sound_regs = [0u8; 32];
            sound_regs.copy_from_slice(&memory[80..112]);
            if let Some(ref sound_tx) = instance.sound_tx {
                let _ = sound_tx.send(RegisterUpdate {
                    time,
                    data: sound_regs,
                });
            }

            let framebuffer_mem = &memory[120..(120 + 320 * 240)];
            let palette_mem = &memory[0x13000..];
            self.window
                .end_frame(framebuffer_mem, palette_mem, next_frame);

            if result.is_ok() {
                self.instance = Some(instance);
            }
        }

        result?;
        Ok(())
    }
}

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
    let log_line = std::sync::Mutex::new(String::new());
    linker.func_wrap("env", "logChar", move |c: i32| {
        let mut log_line = log_line.lock().unwrap();
        if c == 10 {
            println!("{}", log_line);
            log_line.clear();
        } else {
            log_line.push(c as u8 as char);
        }
    })?;
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

struct RegisterUpdate {
    time: i32,
    data: [u8; 32],
}

struct Uw8Sound {
    stream: cpal::Stream,
    tx: mpsc::SyncSender<RegisterUpdate>,
}

fn init_sound(
    engine: &wasmtime::Engine,
    platform_module: &wasmtime::Module,
    module: &wasmtime::Module,
) -> Result<Uw8Sound> {
    let mut store = wasmtime::Store::new(engine, ());
    store.set_epoch_deadline(60);

    let memory = wasmtime::Memory::new(&mut store, MemoryType::new(4, Some(4)))?;

    let mut linker = wasmtime::Linker::new(engine);
    linker.define("env", "memory", memory)?;
    add_native_functions(&mut linker, &mut store)?;

    let platform_instance = instantiate_platform(&mut linker, &mut store, platform_module)?;
    let instance = linker.instantiate(&mut store, module)?;

    let snd = instance
        .get_typed_func::<(i32,), f32>(&mut store, "snd")
        .or_else(|_| platform_instance.get_typed_func::<(i32,), f32>(&mut store, "sndGes"))?;

    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or_else(|| anyhow!("No audio output device available"))?;
    let mut configs: Vec<_> = device
        .supported_output_configs()?
        .filter(|config| {
            config.channels() == 2 && config.sample_format() == cpal::SampleFormat::F32
        })
        .collect();
    configs.sort_by_key(|config| {
        let rate = 44100
            .max(config.min_sample_rate().0)
            .min(config.max_sample_rate().0);
        if rate >= 44100 {
            rate - 44100
        } else {
            (44100 - rate) * 1000
        }
    });
    let config = configs
        .into_iter()
        .next()
        .ok_or_else(|| anyhow!("Could not find float output config"))?;
    let sample_rate = cpal::SampleRate(44100)
        .max(config.min_sample_rate())
        .min(config.max_sample_rate());
    let config = config.with_sample_rate(sample_rate);
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

    let sample_rate = config.sample_rate.0 as usize;

    let (tx, rx) = mpsc::sync_channel::<RegisterUpdate>(30);

    struct Resampler {
        resampler: rubato::FftFixedIn<f32>,
        input_buffers: Vec<Vec<f32>>,
        output_buffers: Vec<Vec<f32>>,
        output_index: usize,
    }
    let mut resampler: Option<Resampler> = if sample_rate == 44100 {
        None
    } else {
        let rs = rubato::FftFixedIn::new(44100, sample_rate, 128, 1, 2)?;
        let input_buffers = rs.input_buffer_allocate();
        let output_buffers = rs.output_buffer_allocate();
        Some(Resampler {
            resampler: rs,
            input_buffers,
            output_buffers,
            output_index: usize::MAX,
        })
    };

    let mut sample_index = 0;
    let mut pending_updates: Vec<RegisterUpdate> = Vec::with_capacity(30);
    let mut current_time = 0;
    let stream = device.build_output_stream(
        &config,
        move |mut outer_buffer: &mut [f32], _| {
            let mut first_update = true;
            while let Ok(update) = rx.try_recv() {
                if first_update {
                    current_time += update.time.wrapping_sub(current_time) / 8;
                    first_update = false;
                }
                pending_updates.push(update);
            }

            while !outer_buffer.is_empty() {
                store.set_epoch_deadline(30);
                while pending_updates
                    .first()
                    .into_iter()
                    .any(|u| u.time.wrapping_sub(current_time) <= 0)
                {
                    let update = pending_updates.remove(0);
                    memory.write(&mut store, 80, &update.data).unwrap();
                }

                let duration = if let Some(update) = pending_updates.first() {
                    ((update.time.wrapping_sub(current_time) as usize) * sample_rate + 999) / 1000
                } else {
                    outer_buffer.len()
                };
                let step_size = (duration.max(64) * 2).min(outer_buffer.len());

                let mut buffer = &mut outer_buffer[..step_size];

                {
                    let mem = memory.data_mut(&mut store);
                    mem[64..68].copy_from_slice(&current_time.to_le_bytes());
                }

                if let Some(ref mut resampler) = resampler {
                    while !buffer.is_empty() {
                        let copy_size = resampler.output_buffers[0]
                            .len()
                            .saturating_sub(resampler.output_index)
                            .min(buffer.len() / 2);
                        if copy_size == 0 {
                            resampler.input_buffers[0].clear();
                            resampler.input_buffers[1].clear();
                            for _ in 0..resampler.resampler.input_frames_next() {
                                resampler.input_buffers[0]
                                    .push(snd.call(&mut store, (sample_index,)).unwrap_or(0.0));
                                resampler.input_buffers[1]
                                    .push(snd.call(&mut store, (sample_index + 1,)).unwrap_or(0.0));
                                sample_index = sample_index.wrapping_add(2);
                            }

                            resampler
                                .resampler
                                .process_into_buffer(
                                    &resampler.input_buffers,
                                    &mut resampler.output_buffers,
                                    None,
                                )
                                .unwrap();
                            resampler.output_index = 0;
                        } else {
                            for i in 0..copy_size {
                                buffer[i * 2] =
                                    resampler.output_buffers[0][resampler.output_index + i];
                                buffer[i * 2 + 1] =
                                    resampler.output_buffers[1][resampler.output_index + i];
                            }
                            resampler.output_index += copy_size;
                            buffer = &mut buffer[copy_size * 2..];
                        }
                    }
                } else {
                    for v in buffer {
                        *v = snd.call(&mut store, (sample_index,)).unwrap_or(0.0);
                        sample_index = sample_index.wrapping_add(1);
                    }
                }

                outer_buffer = &mut outer_buffer[step_size..];
                current_time =
                    current_time.wrapping_add((step_size * 500 / sample_rate).max(1) as i32);
            }
        },
        move |err| {
            dbg!(err);
        },
    )?;

    Ok(Uw8Sound { stream, tx })
}
