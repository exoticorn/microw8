use anyhow::Result;
use gpu::scale_mode::ScaleMode;
use std::time::Instant;

mod cpu;
mod gpu;

pub struct Window {
    inner: Box<dyn WindowImpl>,
    fps_counter: Option<FpsCounter>,
}

struct FpsCounter {
    start: Instant,
    num_frames: u32,
}

impl Window {
    pub fn new(mut config: WindowConfig) -> Result<Window> {
        let fps_counter = if config.fps_counter {
            Some(FpsCounter {
                start: Instant::now(),
                num_frames: 0,
            })
        } else {
            None
        };
        config.scale = config.scale.max(1.).min(20.);
        if config.enable_gpu {
            match gpu::Window::new(config) {
                Ok(window) => {
                    return Ok(Window {
                        inner: Box::new(window),
                        fps_counter,
                    })
                }
                Err(err) => eprintln!(
                    "Failed to create gpu window: {}\nFalling back tp cpu window",
                    err
                ),
            }
        }
        cpu::Window::new().map(|window| Window {
            inner: Box::new(window),
            fps_counter,
        })
    }

    pub fn begin_frame(&mut self) -> Input {
        self.inner.begin_frame()
    }
    pub fn end_frame(&mut self, framebuffer: &[u8], palette: &[u8], next_frame: Instant) {
        self.inner.end_frame(framebuffer, palette, next_frame);
        if let Some(ref mut fps_counter) = self.fps_counter {
            fps_counter.num_frames += 1;
            let elapsed = fps_counter.start.elapsed().as_secs_f32();
            if elapsed >= 1.0 {
                println!("fps: {:.1}", fps_counter.num_frames as f32 / elapsed);
                fps_counter.num_frames = 0;
                fps_counter.start = Instant::now();
            }
        }
    }

    pub fn is_open(&self) -> bool {
        self.inner.is_open()
    }
}

#[derive(Debug)]
pub struct WindowConfig {
    enable_gpu: bool,
    filter: u32,
    fullscreen: bool,
    fps_counter: bool,
    scale: f32,
    scale_mode: ScaleMode,
}

impl Default for WindowConfig {
    fn default() -> WindowConfig {
        WindowConfig {
            enable_gpu: true,
            filter: 5,
            fullscreen: false,
            fps_counter: false,
            scale: 2.,
            scale_mode: ScaleMode::Fit,
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
        self.fps_counter = args.contains("--fps");
        self.scale = args
            .opt_value_from_str("--scale")
            .unwrap()
            .unwrap_or(self.scale);
        if args.contains("--scale-fill") {
            self.scale_mode = ScaleMode::Fill;
        }
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
