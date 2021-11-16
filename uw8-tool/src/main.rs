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
                let version: u8 = args.free_from_str()?;
                BaseModule::for_format_version(version)?
                    .write_to_file(format!("base{}.wasm", version))?;
            }
            "pack" => {
                let version: u8 = args.opt_value_from_str(["-v", "--version"])?.unwrap_or(1);
                let source: PathBuf = args.free_from_str()?;
                let dest: PathBuf = args.free_from_str()?;
                pack::pack_file(&source, &dest, version)?;
            }
            "unpack" => {
                let source: PathBuf = args.free_from_str()?;
                let dest: PathBuf = args.free_from_str()?;
                pack::unpack_file(&source, &dest)?;
            }
            _ => {
                eprintln!("Unknown subcommand '{}'", cmd);
                print_help();
            }
        }
    } else {
        print_help();
    }

    Ok(())
}

fn print_help() {
    println!(
        "Usage:
    uw8-tool make-base <version>
    uw8-tool pack <wasm file> <uw8 file>
    uw8-tool unpack <uw8 file> <wasm file>"
    );
}
