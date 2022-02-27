use anyhow::{anyhow, bail, Result};
use notify::{DebouncedEvent, RecommendedWatcher, Watcher};
use std::{collections::BTreeSet, path::PathBuf, sync::mpsc, time::Duration};

pub struct FileWatcher {
    watcher: RecommendedWatcher,
    watched_files: BTreeSet<PathBuf>,
    directories: BTreeSet<PathBuf>,
    rx: mpsc::Receiver<DebouncedEvent>,
}

impl FileWatcher {
    pub fn new() -> Result<FileWatcher> {
        let (tx, rx) = mpsc::channel();
        let watcher = notify::watcher(tx, Duration::from_millis(100))?;
        Ok(FileWatcher {
            watcher,
            watched_files: BTreeSet::new(),
            directories: BTreeSet::new(),
            rx,
        })
    }

    pub fn add_file<P: Into<PathBuf>>(&mut self, path: P) -> Result<()> {
        let path = path.into();
        let parent = path.parent().ok_or_else(|| anyhow!("File has no parent"))?;

        if !self.directories.contains(parent) {
            self.watcher
                .watch(parent, notify::RecursiveMode::NonRecursive)?;
            self.directories.insert(parent.to_path_buf());
        }

        self.watched_files.insert(path);
        Ok(())
    }

    pub fn poll_changed_file(&self) -> Result<Option<PathBuf>> {
        let event = self.rx.try_recv();
        match event {
            Ok(DebouncedEvent::Create(path) | DebouncedEvent::Write(path)) => {
                let handle = same_file::Handle::from_path(&path)?;
                for file in &self.watched_files {
                    if handle == same_file::Handle::from_path(file)? {
                        return Ok(Some(path));
                    }
                }
            }
            Err(mpsc::TryRecvError::Disconnected) => bail!("File watcher disconnected"),
            _ => (),
        }

        Ok(None)
    }
}
