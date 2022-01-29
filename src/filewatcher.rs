use anyhow::{bail, Result};
use notify::{DebouncedEvent, Watcher, RecommendedWatcher};
use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
    sync::mpsc,
    time::Duration,
};

pub struct FileWatcher {
    _watcher: RecommendedWatcher,
    watched_files: BTreeSet<PathBuf>,
    rx: mpsc::Receiver<DebouncedEvent>,
}

pub struct FileWatcherBuilder(BTreeSet<PathBuf>);

impl FileWatcher {
    pub fn builder() -> FileWatcherBuilder {
        FileWatcherBuilder(BTreeSet::new())
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

impl FileWatcherBuilder {
    pub fn add_file<P: Into<PathBuf>>(&mut self, path: P) -> &mut Self {
        self.0.insert(path.into());
        self
    }

    pub fn build(self) -> Result<FileWatcher> {
        let mut directories: BTreeSet<&Path> = BTreeSet::new();

        for file in &self.0 {
            if let Some(directory) = file.parent() {
                directories.insert(directory);
            }
        }

        let (tx, rx) = mpsc::channel();
        let mut watcher = notify::watcher(tx, Duration::from_millis(100))?;

        for directory in directories {
            watcher.watch(directory, notify::RecursiveMode::NonRecursive)?;
        }

        Ok(FileWatcher {
            _watcher: watcher,
            watched_files: self.0,
            rx,
        })
    }
}
