use anyhow::Result;
use std::time::Instant;

mod cpu;
mod gpu;

pub struct Window(Box<dyn WindowImpl>);

impl Window {
    pub fn new(gpu: bool) -> Result<Window> {
        if gpu {
            match gpu::Window::new() {
                Ok(window) => return Ok(Window(Box::new(window))),
                Err(err) => eprintln!(
                    "Failed to create gpu window: {}\nFalling back tp cpu window",
                    err
                ),
            }
        }
        cpu::Window::new().map(|window| Window(Box::new(window)))
    }

    pub fn begin_frame(&mut self) -> Input {
        self.0.begin_frame()
    }
    pub fn end_frame(&mut self, framebuffer: &[u8], palette: &[u8], next_frame: Instant) {
        self.0.end_frame(framebuffer, palette, next_frame)
    }

    pub fn is_open(&self) -> bool {
        self.0.is_open()
    }
}

pub struct Input {
    pub gamepads: [u8; 4],
    pub reset: bool,
}

trait WindowImpl {
    fn begin_frame(&mut self) -> Input;
    fn end_frame(&mut self, framebuffer: &[u8], palette: &[u8], next_frame: Instant);
    fn is_open(&self) -> bool;
}
