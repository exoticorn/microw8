use anyhow::{anyhow, Result};
use microw8::MicroW8;

mod microw8;

fn main() -> Result<()> {
    let filename = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow!("Missing .uw8 file path"))?;

    let mut uw8 = MicroW8::new()?;

    uw8.load_from_file(filename)?;

    while uw8.is_open() {
        uw8.run_frame()?;
    }

    Ok(())
}
