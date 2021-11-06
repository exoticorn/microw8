#![no_std]
#![feature(core_intrinsics)]

#[link(wasm_import_module = "math")]
extern "C" {
    fn atan2(x: f32, y: f32) -> f32;
}

fn sqrt(v: f32) -> f32 {
    unsafe { core::intrinsics::sqrtf32(v) }
}

#[no_mangle]
pub fn tic(time: i32) {
    unsafe {
        for i in 0..320 * 256 {
            let t = time as f32 / 10 as f32;
            let x = (i % 320 - 160) as f32;
            let y = (i / 320 - 128) as f32;
            let d = 20000 as f32 / sqrt(x * x + y * y + 1 as f32);
            let u = atan2(x, y) * 512f32 / 3.141;
            let c = ((d + t).to_int_unchecked::<i32>() ^ (u + t).to_int_unchecked::<i32>()) as u8;
            *((120 + i) as *mut u8) = c;
        }
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
