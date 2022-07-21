use std::time::Instant;

use crate::{Input, WindowImpl};
use anyhow::Result;
use minifb::{Key, WindowOptions};

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

pub struct Window {
    window: minifb::Window,
    buffer: Vec<u32>,
}

impl Window {
    pub fn new() -> Result<Window> {
        #[cfg(target_os = "windows")]
        unsafe {
            winapi::um::timeapi::timeBeginPeriod(1);
        }

        let buffer: Vec<u32> = vec![0; 320 * 240];

        let options = WindowOptions {
            scale: minifb::Scale::X2,
            scale_mode: minifb::ScaleMode::AspectRatioStretch,
            resize: true,
            ..Default::default()
        };
        let window = minifb::Window::new("MicroW8", 320, 240, options).unwrap();

        Ok(Window { window, buffer })
    }
}

impl WindowImpl for Window {
    fn begin_frame(&mut self) -> Input {
        let mut gamepads = [0u8; 4];
        for key in self.window.get_keys() {
            if let Some(index) = GAMEPAD_KEYS
                .iter()
                .enumerate()
                .find(|(_, &k)| k == key)
                .map(|(i, _)| i)
            {
                gamepads[0] |= 1 << index;
            }
        }

        Input {
            gamepads,
            reset: self.window.is_key_pressed(Key::R, minifb::KeyRepeat::No),
        }
    }

    fn end_frame(&mut self, framebuffer: &[u8], palette: &[u8], next_frame: Instant) {
        for (i, &color_index) in framebuffer.iter().enumerate() {
            let offset = color_index as usize * 4;
            self.buffer[i] = 0xff000000
                | ((palette[offset] as u32) << 16)
                | ((palette[offset + 1] as u32) << 8)
                | palette[offset + 2] as u32;
        }
        self.window
            .update_with_buffer(&self.buffer, 320, 240)
            .unwrap();
        if let Some(sleep) = next_frame.checked_duration_since(Instant::now()) {
            std::thread::sleep(sleep);
        }
    }

    fn is_open(&self) -> bool {
        self.window.is_open()
    }
}
