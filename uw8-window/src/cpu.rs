use std::time::Instant;

use crate::Framebuffer;
use minifb::{Key, Window, WindowOptions};

static GAMEPAD_KEYS: &[Key] = &[
    Key::Up,
    Key::Down,
    Key::Left,
    Key::Right,
    Key::Z,
    Key::X,
    Key::A,
    Key::S,
];

pub fn run(mut update: Box<dyn FnMut(&mut dyn Framebuffer, u32, bool) -> Instant + 'static>) -> ! {
    #[cfg(target_os = "windows")]
    unsafe {
        winapi::um::timeapi::timeBeginPeriod(1);
    }

    let mut buffer: Vec<u32> = vec![0; 320 * 240];

    let options = WindowOptions {
        scale: minifb::Scale::X2,
        scale_mode: minifb::ScaleMode::AspectRatioStretch,
        resize: true,
        ..Default::default()
    };
    let mut window = Window::new("MicroW8", 320, 240, options).unwrap();

    let mut next_frame = Instant::now();
    while window.is_open() && !window.is_key_down(Key::Escape) {
        if let Some(sleep) = next_frame.checked_duration_since(Instant::now()) {
            std::thread::sleep(sleep);
        }
        next_frame = update(
            &mut CpuFramebuffer {
                buffer: &mut buffer,
            },
            0,
            false,
        );
        window.update_with_buffer(&buffer, 320, 240).unwrap();
    }
    std::process::exit(0);
}

struct CpuFramebuffer<'a> {
    buffer: &'a mut Vec<u32>,
}

impl<'a> Framebuffer for CpuFramebuffer<'a> {
    fn update(&mut self, framebuffer: &[u8], palette: &[u8]) {
        for (i, &color_index) in framebuffer.iter().enumerate() {
            let offset = color_index as usize * 4;
            self.buffer[i] = 0xff000000
                | ((palette[offset] as u32) << 16)
                | ((palette[offset + 1] as u32) << 8)
                | palette[offset + 2] as u32;
        }
    }
}
