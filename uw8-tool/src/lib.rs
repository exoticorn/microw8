mod base_module;
mod filter_exports;
mod pack;

pub use base_module::BaseModule;
pub use filter_exports::filter_exports;
pub use pack::{pack, pack_file, unpack, unpack_file, PackConfig};

pub fn compressed_size(cart: &[u8]) -> f32 {
    if cart[0] != 2 {
        cart.len() as f32
    } else {
        upkr::compressed_size(&cart[1..]) + 1.
    }
}
