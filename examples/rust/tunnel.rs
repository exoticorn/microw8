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

    extern "C" {
        pub fn time() -> f32;
    }
}

fn atan2(x: f32, y: f32) -> f32 {
    unsafe { env::atan2(x, y) }
}

fn time() -> f32 {
    unsafe { env::time() }
}

fn sqrt(v: f32) -> f32 {
    unsafe { core::intrinsics::sqrtf32(v) }
}

#[no_mangle]
pub fn upd() {
    let mut i: i32 = 0;
    loop {
        let t = time() * 63.;
        let x = (i % 320 - 160) as f32;
        let y = (i / 320 - 120) as f32;
        let d = 40000 as f32 / sqrt(x * x + y * y);
        let u = atan2(x, y) * 512. / 3.141;
        let c = ((d + t * 2.) as i32 ^ (u + t) as i32) as u8 >> 4;
        unsafe {
            *((120 + i) as *mut u8) = c;
        }
        i += 1;
        if i >= 320*240 {
            break;
        }
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
