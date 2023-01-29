use anyhow::{anyhow, bail, Result};
use notify_debouncer_mini::{
    new_debouncer,
    notify::{self, RecommendedWatcher},
    DebouncedEvent, DebouncedEventKind, Debouncer,
};
use std::{collections::BTreeSet, path::PathBuf, sync::mpsc, time::Duration};

pub struct FileWatcher {
    debouncer: Debouncer<RecommendedWatcher>,
    watched_files: BTreeSet<PathBuf>,
    directories: BTreeSet<PathBuf>,
    rx: mpsc::Receiver<DebouncedEvent>,
}

impl FileWatcher {
    pub fn new() -> Result<FileWatcher> {
        let (tx, rx) = mpsc::channel();
        let debouncer = new_debouncer(Duration::from_millis(100), None, move |res| match res {
            Ok(events) => {
                for event in events {
                    let _ = tx.send(event);
                }
            }
            Err(errs) => {
                for err in errs {
                    eprintln!("Error watching for file changes: {err}");
                }
            }
        })?;
        Ok(FileWatcher {
            debouncer,
            watched_files: BTreeSet::new(),
            directories: BTreeSet::new(),
            rx,
        })
    }

    pub fn add_file<P: Into<PathBuf>>(&mut self, path: P) -> Result<()> {
        let path = path.into();
        let parent = path.parent().ok_or_else(|| anyhow!("File has no parent"))?;

        if !self.directories.contains(parent) {
            self.debouncer
                .watcher()
                .watch(parent, notify::RecursiveMode::NonRecursive)?;
            self.directories.insert(parent.to_path_buf());
        }

        self.watched_files.insert(path);
        Ok(())
    }

    pub fn poll_changed_file(&self) -> Result<Option<PathBuf>> {
        match self.rx.try_recv() {
            Ok(event) => match event.kind {
                DebouncedEventKind::Any => {
                    let handle = same_file::Handle::from_path(&event.path)?;
                    for file in &self.watched_files {
                        if handle == same_file::Handle::from_path(file)? {
                            return Ok(Some(event.path));
                        }
                    }
                }
                _ => (),
            },
            Err(mpsc::TryRecvError::Disconnected) => bail!("File watcher disconnected"),
            _ => (),
        }

        Ok(None)
    }
}
