use std::fs::File;
use std::io::prelude::*;
use std::process;
use std::sync::mpsc;
use std::time::Duration;
use std::{
    path::{Path, PathBuf},
    process::exit,
};

use anyhow::{bail, Result};
use notify::{DebouncedEvent, Watcher};
use pico_args::Arguments;
use uw8::MicroW8;

fn main() -> Result<()> {
    let mut args = Arguments::from_env();

    match args.subcommand()?.as_ref().map(|s| s.as_str()) {
        Some("run") => run(args),
        Some("pack") => pack(args),
        Some("filter-exports") => filter_exports(args),
        Some("help") | None => {
            println!("Usage:");
            println!("  uw8 run [-w/--watch] [-p/--pack] [-u/--uncompressed] [-l/--level] [-o/--output <out-file>] <file>");
            println!("  uw8 pack [-u/--uncompressed] [-l/--level] <in-file> <out-file>");
            println!("  uw8 filter-exports <in-wasm> <out-wasm>");
            Ok(())
        }
        Some(other) => {
            eprintln!("Unknown command '{}'", other);
            process::exit(1);
        }
    }
}

fn run(mut args: Arguments) -> Result<()> {
    let watch_mode = args.contains(["-w", "--watch"]);

    let mut config = Config::default();
    if args.contains(["-p", "--pack"]) {
        let mut pack = uw8_tool::PackConfig::default();
        if args.contains(["-u", "--uncompressed"]) {
            pack = pack.uncompressed();
        }

        if let Some(level) = args.opt_value_from_str(["-l", "--level"])? {
            pack = pack.with_compression_level(level);
        }

        config.pack = Some(pack);
    }

    if let Some(path) =
        args.opt_value_from_os_str::<_, _, bool>(["-o", "--output"], |s| Ok(s.into()))?
    {
        config.output_path = Some(path);
    }

    let filename = args.free_from_os_str::<PathBuf, bool>(|s| Ok(s.into()))?;

    let mut uw8 = MicroW8::new()?;

    let (tx, rx) = mpsc::channel();
    let mut watcher = notify::watcher(tx, Duration::from_millis(100))?;

    if watch_mode {
        watcher.watch(&filename, notify::RecursiveMode::NonRecursive)?;
    }

    if let Err(err) = start_cart(&filename, &mut uw8, &config) {
        eprintln!("Load error: {}", err);
        if !watch_mode {
            exit(1);
        }
    }

    while uw8.is_open() {
        match rx.try_recv() {
            Ok(DebouncedEvent::Create(_) | DebouncedEvent::Write(_)) => {
                if let Err(err) = start_cart(&filename, &mut uw8, &config) {
                    eprintln!("Load error: {}", err);
                }
            }
            Err(mpsc::TryRecvError::Disconnected) => bail!("File watcher disconnected"),
            _ => (),
        }

        uw8.run_frame()?;
    }

    Ok(())
}

#[derive(Default)]
struct Config {
    pack: Option<uw8_tool::PackConfig>,
    output_path: Option<PathBuf>,
}

fn load_cart(filename: &Path, pack: &Option<uw8_tool::PackConfig>) -> Result<Vec<u8>> {
    let mut cart = vec![];
    File::open(filename)?.read_to_end(&mut cart)?;

    if cart[0] >= 10 {
        let src = String::from_utf8(cart)?;
        cart = if src.chars().find(|c| !c.is_whitespace()) == Some('(') {
            wat::parse_str(src)?
        } else {
            curlywas::compile_str(&src, filename, curlywas::Options::default())?
        };
    }

    if let Some(pack_config) = pack {
        cart = uw8_tool::pack(&cart, pack_config)?;
        println!("packed size: {} bytes", cart.len());
    }

    Ok(cart)
}

fn start_cart(filename: &Path, uw8: &mut MicroW8, config: &Config) -> Result<()> {
    let cart = load_cart(filename, &config.pack)?;

    if let Some(ref path) = config.output_path {
        File::create(path)?.write_all(&cart)?;
    }

    if let Err(err) = uw8.load_from_memory(&cart) {
        eprintln!("Load error: {}", err);
        Err(err)
    } else {
        Ok(())
    }
}

fn pack(mut args: Arguments) -> Result<()> {
    let mut pack_config = uw8_tool::PackConfig::default();

    if args.contains(["-u", "--uncompressed"]) {
        pack_config = pack_config.uncompressed();
    }

    if let Some(level) = args.opt_value_from_str(["-l", "--level"])? {
        pack_config = pack_config.with_compression_level(level);
    }

    let in_file = args.free_from_os_str::<PathBuf, bool>(|s| Ok(s.into()))?;

    let out_file = args.free_from_os_str::<PathBuf, bool>(|s| Ok(s.into()))?;

    let cart = load_cart(&in_file, &Some(pack_config))?;

    File::create(out_file)?.write_all(&cart)?;

    Ok(())
}

fn filter_exports(mut args: Arguments) -> Result<()> {
    let in_file = args.free_from_os_str::<PathBuf, bool>(|s| Ok(s.into()))?;
    let out_file = args.free_from_os_str::<PathBuf, bool>(|s| Ok(s.into()))?;

    uw8_tool::filter_exports(&in_file, &out_file)?;

    Ok(())
}
