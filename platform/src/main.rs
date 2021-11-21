use std::{fs::File, path::Path};
use std::io::prelude::*;

use anyhow::Result;

fn main() -> Result<()> {
    uw8_tool::BaseModule::create_binary(&Path::new("target/base.upk"))?;

    let loader = curlywas::compile_file("src/loader.cwa")?;
    File::create("bin/loader.wasm")?.write_all(&loader)?;

    let platform = curlywas::compile_file("src/platform.cwa")?;
    File::create("bin/platform.uw8")?.write_all(&platform)?;

    Ok(())
}
