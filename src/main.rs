use std::path::PathBuf;
use std::sync::mpsc;
use std::time::Duration;

use anyhow::{bail, Result};
use microw8::MicroW8;
use notify::{DebouncedEvent, Watcher};
use pico_args::Arguments;

mod microw8;

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

    if let Err(err) = uw8.load_from_file(&filename) {
        if !watch_mode {
            return Err(err);
        }
        eprintln!("Load error: {}", err);
    }

    while uw8.is_open() {
        match rx.try_recv() {
            Ok(DebouncedEvent::Create(_) | DebouncedEvent::Write(_)) => {
                if let Err(err) = uw8.load_from_file(&filename) {
                    eprintln!("Load error: {}", err)
                }
            }
            Err(mpsc::TryRecvError::Disconnected) => bail!("File watcher disconnected"),
            _ => (),
        }

        uw8.run_frame()?;
    }

    Ok(())
}
