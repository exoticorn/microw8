mod base_module;

use base_module::BaseModule;
use anyhow::Result;

fn main() -> Result<()> {
    BaseModule::for_format_version(1)?.write_to_file("base.wasm")?;
    Ok(())
}
