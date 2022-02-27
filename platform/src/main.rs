use std::io::prelude::*;
use std::{fs::File, path::Path};

use anyhow::Result;

fn main() -> Result<()> {
    println!("Generating compressed base module");
    uw8_tool::BaseModule::create_binary(&Path::new("target/base.upk"))?;

    println!("Converting font");
    convert_font()?;

    println!("Compiling loader module");
    let loader = curlywas::compile_file("src/loader.cwa", curlywas::Options::default())?;
    File::create("bin/loader.wasm")?.write_all(&loader.wasm)?;

    println!("Loader (including base module): {} bytes", loader.wasm.len());

    println!("Compiling platform module");
    let platform = curlywas::compile_file("src/platform.cwa", curlywas::Options::default())?;
    println!("Compressing platform module");
    let platform = uw8_tool::pack(
        &platform.wasm,
        &uw8_tool::PackConfig::default().with_compression_level(4),
    )?;
    File::create("bin/platform.uw8")?.write_all(&platform)?;
    println!("Platform module: {} bytes", platform.len());

    Ok(())
}

fn convert_font() -> Result<()> {
    let image = lodepng::decode32_file("src/font.png")?;

    assert!(image.width == 128 && image.height == 128);

    let mut font = vec![];
    for char in 0..256 {
        for y in 0..8 {
            let mut byte = 0u8;
            let base = (char % 16 * 8) + (char / 16 * 8 + y) * 128;
            for x in 0..8 {
                byte += byte;
                if image.buffer[base + x].r > 128 {
                    byte |= 1;
                }
            }
            font.push(byte);
        }
    }

    File::create("target/font.bin")?.write_all(&font)?;

    Ok(())
}