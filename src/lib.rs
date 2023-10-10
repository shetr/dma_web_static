mod utils;

use utils::set_panic_hook;
use wasm_bindgen::prelude::*;
use dma;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn lib_init() {
    set_panic_hook();
}

#[wasm_bindgen]
pub fn gcd(a: i32, b: i32) -> i32
{
    dma::gcd(a as i64, b as i64) as i32
}

#[wasm_bindgen]
pub fn test_panic() {
    panic!("panic test!");
}