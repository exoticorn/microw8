use std::io::prelude::*;
use std::{fs::File, time::Instant};

use anyhow::{anyhow, Result};
use minifb::{Key, Window, WindowOptions};
use wasmtime::{GlobalType, MemoryType, Mutability, ValType};

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

        let base_start = module_data.len();
        memory[..base_start].copy_from_slice(module_data);

        let base_module = include_bytes!("../uw8-tool/base1.wasm");
        let base_end = base_start + base_module.len();
        memory[base_start..base_end].copy_from_slice(base_module);

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

fn main() -> Result<()> {
    let filename = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow!("Missing .uw8 file path"))?;
    let mut uw8_module = vec![];
    File::open(filename)?.read_to_end(&mut uw8_module)?;

    let engine = wasmtime::Engine::default();

    let mut loader = Loader::new(&engine)?;

    let platform_module = wasmtime::Module::new(&engine, include_bytes!("../platform/platform.wasm"))?;

    let module = wasmtime::Module::new(&engine, loader.load(&uw8_module)?)?;

    let mut store = wasmtime::Store::new(&engine, ());
    let memory = wasmtime::Memory::new(&mut store, MemoryType::new(4, Some(4)))?;

    let mut linker = wasmtime::Linker::new(&engine);
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

    let platform_instance = linker.instantiate(&mut store, &platform_module)?;

    for export in platform_instance.exports(&mut store) {
        linker.define("env", export.name(), export.into_func().expect("platform surely only exports functions"))?;
    }

    let instance = linker.instantiate(&mut store, &module)?;
    let tic = instance.get_typed_func::<i32, (), _>(&mut store, "tic")?;

    let mut buffer = vec![0u32; 320 * 256];
    let mut window = Window::new("MicroW8", 320, 256, WindowOptions::default())?;
    window.limit_update_rate(Some(std::time::Duration::from_micros(16666)));

    let start_time = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        tic.call(&mut store, start_time.elapsed().as_millis() as i32)?;

        let framebuffer = &memory.data(&store)[120..];
        for i in 0..320 * 256 {
            let c = framebuffer[i];
            buffer[i] = (c as u32) * 0x01010101;
        }

        window.update_with_buffer(&buffer, 320, 256)?;
    }

    Ok(())
}
