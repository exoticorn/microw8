use std::path::PathBuf;

use anyhow::Result;
use pico_args::Arguments;
use uw8_tool::BaseModule;

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
            "base-cwa" => {
                let path: PathBuf = args.free_from_str()?;
                BaseModule::for_format_version(1)?.write_as_cwa(path)?;
            }
            "base-wat" => {
                let path: PathBuf = args.free_from_str()?;
                BaseModule::for_format_version(1)?.write_as_wat(path)?;
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
