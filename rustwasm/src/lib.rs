use wasm_bindgen::prelude::*;

mod dob;
pub mod grid;
mod utils;

#[macro_use]
extern crate more_asserts;
extern crate enum_iterator;
extern crate js_sys;
extern crate serde;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

//#[macro_export]
macro_rules! _console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}
