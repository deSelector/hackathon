use dob::DOB;
use tape::Tape;
use wasm_bindgen::prelude::*;

mod ctx2d;
mod dob;
mod grid;
mod tape;
mod utils;

#[macro_use]
extern crate more_asserts;
extern crate enum_iterator;
extern crate js_sys;

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

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

//#[macro_export]
macro_rules! _console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn paint_dob(dob: DOB, bids: &[f64], asks: &[f64]) {
    init_panic_hook();
    dob.paint(bids, asks);
}

#[wasm_bindgen]
pub fn paint_tape(tape: Tape, trades: &[f64]) {
    init_panic_hook();
    tape.paint(trades);
}
