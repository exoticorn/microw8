use std::time::{Duration, Instant};

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let mut framebuffer = vec![0u8; 320 * 240];
    let start_time = Instant::now();

    let mut palette = vec![0u32; 256];
    for i in 0..256 {
        let v = i & 15;
        let r = ((i >> 2) & 12) * v;
        let g = ((i >> 3) & 12) * v;
        let b = ((i >> 4) & 12) * v;
        palette[i as usize] = r + (g << 8) + (b << 16);
    }

    let mut prev_frame = Instant::now();

    uw8_window::run(move |gpu_framebuffer, _gamepads, _reset| {
        draw_frame(&mut framebuffer, start_time.elapsed().as_secs_f32());
        gpu_framebuffer.update(&framebuffer, bytemuck::cast_slice(&palette));
        prev_frame += Duration::from_secs_f32(1.0 / 60.0);
        prev_frame
    });
}

fn draw_frame(framebuffer: &mut [u8], time: f32) {
    for x in 0..320 {
        let xr = x as f32 - 160.0;
        for y in 0..240 {
            let yr = y as f32 - 120.0;
            let f = 8192.0 / (xr * xr + yr * yr);
            let u = xr * f + 512.0 + time * 32.0;
            let v = yr * f + time * 29.0;
            let c = (u.floor() as i32 ^ v.floor() as i32) as u32;
            framebuffer[x + y * 320] = c as u8;
        }
    }
}
