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
                let mut config = uw8_tool::PackConfig::default();
                if args.contains(["-u", "--uncompressed"]) {
                    config = config.uncompressed();
                }
                let source: PathBuf = args.free_from_str()?;
                let dest: PathBuf = args.free_from_str()?;
                uw8_tool::pack_file(&source, &dest, &config)?;
            }
            "unpack" => {
                let source: PathBuf = args.free_from_str()?;
                let dest: PathBuf = args.free_from_str()?;
                uw8_tool::unpack_file(&source, &dest)?;
            }
            "filter-exports" => {
                let source: PathBuf = args.free_from_str()?;
                let dest: PathBuf = args.free_from_str()?;
                uw8_tool::filter_exports(&source, &dest)?;
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
    uw8-tool unpack <uw8 file> <wasm file>
    uw8-tool filter-exports <wasm file> <wasm file>"
    );
}
