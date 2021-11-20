use std::path::PathBuf;

use anyhow::Result;
use uw8_tool::BaseModule;
use pico_args::Arguments;

fn main() -> Result<()> {
    let mut args = Arguments::from_env();

    if let Some(cmd) = args.subcommand()? {
        match cmd.as_str() {
            "make-base" => {
                let path: PathBuf = args.free_from_str()?;
                BaseModule::create_binary(&path)?;
            }
            "pack" => {
                let version: u8 = args.opt_value_from_str(["-v", "--version"])?.unwrap_or(1);
                let source: PathBuf = args.free_from_str()?;
                let dest: PathBuf = args.free_from_str()?;
                uw8_tool::pack_file(&source, &dest, version)?;
            }
            "unpack" => {
                let source: PathBuf = args.free_from_str()?;
                let dest: PathBuf = args.free_from_str()?;
                uw8_tool::unpack_file(&source, &dest)?;
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
