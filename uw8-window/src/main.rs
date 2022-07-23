use std::time::Instant;
use uw8_window::WindowConfig;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let mut args = pico_args::Arguments::from_env();

    let mut framebuffer = vec![0u8; 320 * 240];
    let mut start_time = Instant::now();

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

    let mut window_config = WindowConfig::default();
    window_config.parse_arguments(&mut args);

    let mut window = uw8_window::Window::new(window_config).unwrap();

    while window.is_open() {
        let input = window.begin_frame();
        if input.reset {
            start_time = Instant::now();
        }
        draw_frame(&mut framebuffer, start_time.elapsed().as_secs_f32());
        window.end_frame(&framebuffer, bytemuck::cast_slice(&palette), Instant::now());

        fps_counter += 1;
        let elapsed = fps_start.elapsed().as_secs_f32();
        if elapsed >= 1.0 {
            println!("{:.1} fps", fps_counter as f32 / elapsed);
            fps_start = Instant::now();
            fps_counter = 0;
        }
    }
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
