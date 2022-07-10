use std::time::Instant;

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

    let mut fps_start = Instant::now();
    let mut fps_counter = 0;

    uw8_window::run(true, move |gpu_framebuffer, _gamepads, _reset| {
        for _ in 0..1 {
            draw_frame(&mut framebuffer, start_time.elapsed().as_secs_f32());
        }
        gpu_framebuffer.update(&framebuffer, bytemuck::cast_slice(&palette));
        fps_counter += 1;
        let elapsed = fps_start.elapsed().as_secs_f32();
        if elapsed >= 1.0 {
            println!("{:.1} fps", fps_counter as f32 / elapsed);
            fps_start = Instant::now();
            fps_counter = 0;
        }
        Instant::now()
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
