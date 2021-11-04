mod base_module;
mod pack;

use std::path::PathBuf;

use anyhow::Result;
use base_module::BaseModule;
use pico_args::Arguments;

fn main() -> Result<()> {
    let mut args = Arguments::from_env();

    if let Some(cmd) = args.subcommand()? {
        match cmd.as_str() {
            "make-base" => {
                let version: u32 = args.free_from_str()?;
                BaseModule::for_format_version(version)?
                    .write_to_file(format!("base{}.wasm", version))?;
            }
            "pack" => {
                let version: u32 = args.opt_value_from_str(["-v", "--version"])?.unwrap_or(1);
                let source: PathBuf = args.free_from_str()?;
                let dest: PathBuf = args.free_from_str()?;
                pack::pack(&source, &dest, version)?;
            }
            _ => {
                eprintln!("Unknown subcommand '{}'", cmd);
                print_help();
            }
        }
    } else {
        print_help();
    }

    BaseModule::for_format_version(1)?.write_to_file("base.wasm")?;
    Ok(())
}

fn print_help() {
    println!(
        "Usage:
    uw8-tool make-base <version>
    uw8-tool pack <wasm file> <uw8 file>"
    );
}
