#![no_std]
#![feature(core_intrinsics)]

mod env {
    // "env" is the default module for imports, but it is still needed here
    // since there is a compiler builtin of the same name which is used
    // if we don't make it clear that this is a module import.
    #[link(wasm_import_module = "env")]
    extern "C" {
        pub fn atan2(x: f32, y: f32) -> f32;
    }
}

fn atan2(x: f32, y: f32) -> f32 {
    unsafe { env::atan2(x, y) }
}

fn sqrt(v: f32) -> f32 {
    unsafe { core::intrinsics::sqrtf32(v) }
}

fn ftoi(v: f32) -> i32 {
    // The compiler is allowed to do bad things to our code if this
    // ever results in a value that doesn't fit in an i32.
    // (the joy of undefined behavior)
    // But that would trap in wasm anyway, so we don't really
    // care.
    unsafe { v.to_int_unchecked() }
}

#[no_mangle]
pub fn tic(time: i32) {
    for i in 0..320 * 256 {
        let t = time as f32 / 10 as f32;
        let x = (i % 320 - 160) as f32;
        let y = (i / 320 - 128) as f32;
        let d = 20000 as f32 / sqrt(x * x + y * y + 1 as f32);
        let u = atan2(x, y) * 512f32 / 3.141;
        let c = (ftoi(d + t) ^ ftoi(u + t)) as u8;
        unsafe {
            *((120 + i) as *mut u8) = c;
        }
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
