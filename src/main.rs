use std::io::prelude::*;
use std::{fs::File, time::Instant};

use anyhow::{anyhow, Result};
use minifb::{Key, Window, WindowOptions};
use wasmtime::{GlobalType, MemoryType, Mutability, ValType};

fn main() -> Result<()> {
    let filename = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow!("Missing .uw8 file path"))?;
    let mut uw8_module = vec![];
    File::open(filename)?.read_to_end(&mut uw8_module)?;

    let engine = wasmtime::Engine::default();
    let module = wasmtime::Module::new(&engine, uw8_module)?;

    let mut store = wasmtime::Store::new(&engine, ());
    let time_global = wasmtime::Global::new(
        &mut store,
        GlobalType::new(ValType::I32, Mutability::Var),
        0.into(),
    )?;
    let memory = wasmtime::Memory::new(&mut store, MemoryType::new(2, Some(2)))?;

    let mut linker = wasmtime::Linker::new(&engine);
    linker.define("uw8", "time", time_global.clone())?;
    linker.define("uw8", "ram", memory.clone())?;

    let instance = linker.instantiate(&mut store, &module)?;
    let tic = instance.get_typed_func::<(), (), _>(&mut store, "tic")?;

    let mut buffer = vec![0u32; 320 * 256];
    let mut window = Window::new("MicroW8", 320, 256, WindowOptions::default())?;
    window.limit_update_rate(Some(std::time::Duration::from_micros(16666)));

    let start_time = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        time_global.set(
            &mut store,
            wasmtime::Val::I32(start_time.elapsed().as_millis() as i32),
        )?;

        tic.call(&mut store, ())?;

        let framebuffer = &memory.data(&store)[120..];
        for i in 0..320 * 256 {
            let c = framebuffer[i];
            buffer[i] = (c as u32) * 0x01010101;
        }

        window.update_with_buffer(&buffer, 320, 256)?;
    }

    Ok(())
}
