use std::fs::File;
use std::sync::mpsc;
use std::time::Duration;
use std::{
    path::{Path, PathBuf},
    process::exit,
};
use std::io::prelude::*;

use anyhow::{bail, Result};
use uw8::MicroW8;
use notify::{DebouncedEvent, Watcher};
use pico_args::Arguments;

fn main() -> Result<()> {
    let mut args = Arguments::from_env();

    let watch_mode = args.contains(["-w", "--watch"]);

    let filename = args.free_from_os_str::<PathBuf, bool>(|s| Ok(s.into()))?;

    let mut uw8 = MicroW8::new()?;

    let (tx, rx) = mpsc::channel();
    let mut watcher = notify::watcher(tx, Duration::from_millis(100))?;

    if watch_mode {
        watcher.watch(&filename, notify::RecursiveMode::NonRecursive)?;
    }

    if let Err(err) = load_cart(&filename, &mut uw8) {
        eprintln!("Load error: {}", err);
        if !watch_mode {
            exit(1);
        }
    }

    while uw8.is_open() {
        match rx.try_recv() {
            Ok(DebouncedEvent::Create(_) | DebouncedEvent::Write(_)) => {
                if let Err(err) = load_cart(&filename, &mut uw8) {
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

fn load_cart(filename: &Path, uw8: &mut MicroW8) -> Result<()> {
    let mut cart = vec![];
    File::open(filename)?.read_to_end(&mut cart)?;

    if cart[0] >= 10 {
        let src = String::from_utf8(cart)?;
        cart = curlywas::compile_str(&src)?;
    }

    if let Err(err) = uw8.load_from_memory(&cart) {
        eprintln!("Load error: {}", err);
        Err(err)
    } else {
        Ok(())
    }
}
