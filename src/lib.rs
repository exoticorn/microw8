mod filewatcher;
#[cfg(feature = "native")]
mod run_native;
#[cfg(feature = "browser")]
mod run_web;

pub use filewatcher::FileWatcher;
#[cfg(feature = "native")]
pub use run_native::MicroW8;
#[cfg(feature = "browser")]
pub use run_web::RunWebServer;

use anyhow::Result;

pub trait Runtime {
    fn is_open(&self) -> bool;
    fn load(&mut self, module_data: &[u8]) -> Result<()>;
    fn run_frame(&mut self) -> Result<()>;
}
