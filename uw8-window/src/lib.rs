use std::time::Instant;

mod cpu;
mod gpu;

pub fn run<F: 'static + FnMut(&mut dyn Framebuffer, u32, bool) -> Instant>(update: F) -> ! {
    match gpu::Window::new() {
        Ok(window) => window.run(Box::new(update)),
        Err(err) => eprintln!(
            "Failed to create gpu window: {}\nFalling back to cpu window",
            err
        ),
    }
    cpu::run(Box::new(update));
}

pub trait Framebuffer {
    fn update(&mut self, pixels: &[u8], palette: &[u8]);
}
