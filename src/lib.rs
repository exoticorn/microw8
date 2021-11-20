use std::io::prelude::*;
use std::path::Path;
use std::{fs::File, time::Instant};

use anyhow::Result;
use minifb::{Key, Window, WindowOptions};
use wasmtime::{
    Engine, GlobalType, Memory, MemoryType, Module, Mutability, Store, TypedFunc, ValType,
};

pub struct MicroW8 {
    engine: Engine,
    loader: Loader,
    platform_module: Module,
    window: Window,
    window_buffer: Vec<u32>,
    instance: Option<UW8Instance>,
}

struct UW8Instance {
    store: Store<()>,
    memory: Memory,
    tic: TypedFunc<i32, ()>,
    start_time: Instant,
}

impl MicroW8 {
    pub fn new() -> Result<MicroW8> {
        let engine = wasmtime::Engine::default();

        let loader = Loader::new(&engine)?;

        let platform_module =
            wasmtime::Module::new(&engine, include_bytes!("../platform/platform.wasm"))?;

        let mut window = Window::new("MicroW8", 320, 240, WindowOptions::default())?;
        window.limit_update_rate(Some(std::time::Duration::from_micros(16666)));

        Ok(MicroW8 {
            engine,
            loader,
            platform_module,
            window,
            window_buffer: vec![0u32; 320 * 240],
            instance: None,
        })
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open() && !self.window.is_key_down(Key::Escape)
    }

    fn reset(&mut self) {
        self.instance = None;
        for v in &mut self.window_buffer {
            *v = 0;
        }
    }

    pub fn load_from_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        self.reset();

        let mut module = vec![];
        File::open(path)?.read_to_end(&mut module)?;
        self.load_from_memory(&module)
    }

    pub fn load_from_memory(&mut self, module: &[u8]) -> Result<()> {
        self.reset();

        let module = wasmtime::Module::new(&self.engine, self.loader.load(module)?)?;

        let mut store = wasmtime::Store::new(&self.engine, ());
        let memory = wasmtime::Memory::new(&mut store, MemoryType::new(4, Some(4)))?;

        let mut linker = wasmtime::Linker::new(&self.engine);
        linker.define("env", "memory", memory.clone())?;
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
        for i in 9..64 {
            linker.func_wrap("env", &format!("reserved{}", i), || {})?;
        }
        for i in 0..16 {
            linker.define(
                "env",
                &format!("g_reserved{}", i),
                wasmtime::Global::new(
                    &mut store,
                    GlobalType::new(ValType::I32, Mutability::Const),
                    0.into(),
                )?,
            )?;
        }

        let platform_instance = linker.instantiate(&mut store, &self.platform_module)?;

        for export in platform_instance.exports(&mut store) {
            linker.define(
                "env",
                export.name(),
                export
                    .into_func()
                    .expect("platform surely only exports functions"),
            )?;
        }

        let instance = linker.instantiate(&mut store, &module)?;
        let tic = instance.get_typed_func::<i32, (), _>(&mut store, "tic")?;

        self.instance = Some(UW8Instance {
            store,
            memory,
            tic,
            start_time: Instant::now(),
        });

        Ok(())
    }

    pub fn run_frame(&mut self) -> Result<()> {
        if let Some(mut instance) = self.instance.take() {
            instance.tic.call(
                &mut instance.store,
                instance.start_time.elapsed().as_millis() as i32,
            )?;

            let framebuffer = &instance.memory.data(&instance.store)[120..];
            let palette = &framebuffer[320 * 240..];
            for i in 0..320 * 240 {
                let offset = framebuffer[i] as usize * 4;
                self.window_buffer[i] = 0xff000000
                    | ((palette[offset + 0] as u32) << 16)
                    | ((palette[offset + 1] as u32) << 8)
                    | palette[offset + 2] as u32;
            }

            self.instance = Some(instance);
        }

        self.window
            .update_with_buffer(&self.window_buffer, 320, 240)?;

        Ok(())
    }
}

struct Loader {
    store: wasmtime::Store<()>,
    memory: wasmtime::Memory,
    instance: wasmtime::Instance,
}

impl Loader {
    fn new(engine: &wasmtime::Engine) -> Result<Loader> {
        let module = wasmtime::Module::new(engine, include_bytes!("../platform/loader.wasm"))?;
        let mut store = wasmtime::Store::new(engine, ());

        let mut linker = wasmtime::Linker::new(engine);
        let memory = wasmtime::Memory::new(&mut store, MemoryType::new(9, Some(9)))?;
        linker.define("env", "memory", memory.clone())?;

        let instance = linker.instantiate(&mut store, &module)?;
        Ok(Loader {
            store,
            memory,
            instance,
        })
    }

    fn load(&mut self, module_data: &[u8]) -> Result<Vec<u8>> {
        let memory = self.memory.data_mut(&mut self.store);

        let compressed_base_module = include_bytes!("../uw8-tool/base.upk");
        memory[..compressed_base_module.len()].copy_from_slice(compressed_base_module);

        let base_end = self.instance.get_typed_func::<(i32, i32), i32, _>(&mut self.store, "uncompress")?.call(&mut self.store, (0, 0x84000))? as u32 as usize;

        let memory = self.memory.data_mut(&mut self.store);
    
        let base_module = memory[0x84000..base_end].to_vec();

        let base_start = module_data.len();
        memory[..base_start].copy_from_slice(module_data);

        let base_end = base_start + base_module.len();
        memory[base_start..base_end].copy_from_slice(&base_module);

        let load_uw8 = self
            .instance
            .get_typed_func::<(i32, i32, i32, i32), i32, _>(&mut self.store, "load_uw8")?;
        let end_offset = load_uw8.call(
            &mut self.store,
            (0, base_start as i32, base_start as i32, base_end as i32),
        )? as u32 as usize;

        Ok(self.memory.data(&self.store)[base_end..end_offset].to_vec())
    }
}
