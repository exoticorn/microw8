use std::io::prelude::*;
use std::{fs::File, path::Path};

use anyhow::Result;

fn main() -> Result<()> {
    println!("Generating compressed base module");
    uw8_tool::BaseModule::create_binary(&Path::new("target/base.upk"))?;

    println!("Compiling loader module");
    let loader = curlywas::compile_file("src/loader.cwa")?;
    File::create("bin/loader.wasm")?.write_all(&loader)?;

    println!("Compiling platform module");
    let platform = curlywas::compile_file("src/platform.cwa")?;
    println!("Compressing platform module");
    let platform = uw8_tool::pack(
        &platform,
        uw8_tool::PackConfig::default().with_compression(),
    )?;
    File::create("bin/platform.uw8")?.write_all(&platform)?;
    println!("All done!");

    Ok(())
}
