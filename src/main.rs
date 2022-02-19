use std::fs::File;
use std::io::prelude::*;
use std::process;
use std::time::Duration;
use std::{
    path::{Path, PathBuf},
    process::exit,
};

use anyhow::Result;
use pico_args::Arguments;
use uw8::{FileWatcher, MicroW8, RunWebServer};

fn main() -> Result<()> {
    let mut args = Arguments::from_env();

    match args.subcommand()?.as_ref().map(|s| s.as_str()) {
        Some("version") => {
            println!("{}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
        Some("run") => run(args),
        Some("pack") => pack(args),
        Some("unpack") => unpack(args),
        Some("compile") => compile(args),
        Some("filter-exports") => filter_exports(args),
        Some("help") | None => {
            println!("uw8 {}", env!("CARGO_PKG_VERSION"));
            println!();
            println!("Usage:");
            println!("  uw8 run [-t/--timeout <frames>] [-w/--watch] [-p/--pack] [-u/--uncompressed] [-l/--level] [-o/--output <out-file>] <file>");
            println!("  uw8 pack [-u/--uncompressed] [-l/--level] <in-file> <out-file>");
            println!("  uw8 unpack <in-file> <out-file>");
            println!("  uw8 compile [-d/--debug] <in-file> <out-file>");
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
    let timeout: Option<u32> = args.opt_value_from_str(["-t", "--timeout"])?;

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

    let run_browser = args.contains(["-b", "--browser"]);

    let filename = args.free_from_os_str::<PathBuf, bool>(|s| Ok(s.into()))?;

    if !run_browser {
        let mut uw8 = MicroW8::new()?;

        if let Some(timeout) = timeout {
            uw8.set_timeout(timeout);
        }

        let mut watcher = FileWatcher::builder();

        if watch_mode {
            watcher.add_file(&filename);
        }

        let watcher = watcher.build()?;

        if let Err(err) = start_cart(&filename, &mut uw8, &config) {
            eprintln!("Load error: {}", err);
            if !watch_mode {
                exit(1);
            }
        }

        while uw8.is_open() {
            if watcher.poll_changed_file()?.is_some() {
                if let Err(err) = start_cart(&filename, &mut uw8, &config) {
                    eprintln!("Load error: {}", err);
                }
            }

            if let Err(err) = uw8.run_frame() {
                eprintln!("Runtime error: {}", err);
                if !watch_mode {
                    exit(1);
                }
            }
        }
    } else {
        let mut server = RunWebServer::new();
        match load_cart(&filename, &config) {
            Ok(cart) => server.load_module(&cart)?,
            Err(err) => {
                eprintln!("Load error: {}", err);
                if !watch_mode {
                    exit(1);
                }
            }
        }

        loop {
            std::thread::sleep(Duration::from_millis(100));
        }
    }

    Ok(())
}

#[derive(Default)]
struct Config {
    pack: Option<uw8_tool::PackConfig>,
    output_path: Option<PathBuf>,
}

fn load_cart(filename: &Path, config: &Config) -> Result<Vec<u8>> {
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

    if let Some(ref pack_config) = config.pack {
        cart = uw8_tool::pack(&cart, pack_config)?;
        println!("packed size: {} bytes", cart.len());
    }

    if let Some(ref path) = config.output_path {
        File::create(path)?.write_all(&cart)?;
    }

    Ok(cart)
}

fn start_cart(filename: &Path, uw8: &mut MicroW8, config: &Config) -> Result<()> {
    let cart = load_cart(filename, config)?;

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

    let cart = load_cart(
        &in_file,
        &Config {
            pack: Some(pack_config),
            output_path: None,
        },
    )?;

    File::create(out_file)?.write_all(&cart)?;

    Ok(())
}

fn unpack(mut args: Arguments) -> Result<()> {
    let in_file = args.free_from_os_str::<PathBuf, bool>(|s| Ok(s.into()))?;
    let out_file = args.free_from_os_str::<PathBuf, bool>(|s| Ok(s.into()))?;

    uw8_tool::unpack_file(&in_file, &out_file).into()
}

fn compile(mut args: Arguments) -> Result<()> {
    let mut options = curlywas::Options::default();
    if args.contains(["-d", "--debug"]) {
        options = options.with_debug();
    }

    let in_file = args.free_from_os_str::<PathBuf, bool>(|s| Ok(s.into()))?;
    let out_file = args.free_from_os_str::<PathBuf, bool>(|s| Ok(s.into()))?;

    let module = curlywas::compile_file(in_file, options)?;
    File::create(out_file)?.write_all(&module)?;

    Ok(())
}

fn filter_exports(mut args: Arguments) -> Result<()> {
    let in_file = args.free_from_os_str::<PathBuf, bool>(|s| Ok(s.into()))?;
    let out_file = args.free_from_os_str::<PathBuf, bool>(|s| Ok(s.into()))?;

    uw8_tool::filter_exports(&in_file, &out_file)?;

    Ok(())
}
