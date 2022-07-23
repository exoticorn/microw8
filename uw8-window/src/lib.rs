use anyhow::Result;
use std::time::Instant;

mod cpu;
mod gpu;

pub struct Window(Box<dyn WindowImpl>);

impl Window {
    pub fn new(config: WindowConfig) -> Result<Window> {
        if config.enable_gpu {
            match gpu::Window::new(config) {
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

#[derive(Debug)]
pub struct WindowConfig {
    enable_gpu: bool,
    filter: u32,
    fullscreen: bool,
}

impl Default for WindowConfig {
    fn default() -> WindowConfig {
        WindowConfig {
            enable_gpu: true,
            filter: 5,
            fullscreen: false,
        }
    }
}

impl WindowConfig {
    pub fn parse_arguments(&mut self, args: &mut pico_args::Arguments) {
        self.enable_gpu = !args.contains("--no-gpu");
        if let Some(filter) = args.opt_value_from_str::<_, String>("--filter").unwrap() {
            self.filter = match filter.as_str() {
                "1" | "nearest" => 1,
                "2" | "fast_crt" => 2,
                "3" | "ss_crt" => 3,
                "4" | "chromatic" => 4,
                "5" | "auto_crt" => 5,
                o => {
                    println!("Unknown --filter '{}'", o);
                    std::process::exit(1);
                }
            }
        }
        self.fullscreen = args.contains("--fullscreen");
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
