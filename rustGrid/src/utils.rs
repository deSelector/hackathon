use getrandom;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    CanvasRenderingContext2d, Document, HtmlCanvasElement, HtmlElement, Performance, Window,
};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! _console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[allow(dead_code)]
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn _body() -> HtmlElement {
    document().body().expect("document should have a body")
}

pub fn window() -> Window {
    web_sys::window().expect("no global `window` exists")
}

pub fn document() -> Document {
    window()
        .document()
        .expect("should have a document on window")
}

pub fn canvas(id: &str) -> HtmlCanvasElement {
    document()
        .get_element_by_id(id)
        .expect("should have a canvas element")
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap()
}

pub fn ctx(id: &str) -> CanvasRenderingContext2d {
    canvas(id)
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap()
}

pub fn _performance() -> Performance {
    window()
        .performance()
        .expect("performance should be available")
}

pub fn get_random_255() -> Result<[u8; 255], getrandom::Error> {
    let mut buf = [0u8; 255];
    getrandom::getrandom(&mut buf)?;
    Ok(buf)
}

pub fn _perf_loop(title: &str, count: u32, cb: &dyn Fn()) -> f64 {
    let p1 = _performance().now();
    for _ in 0..count {
        cb();
    }
    let msec = _performance().now() - p1;
    _console_log!("PERFORMANCE: {} in {} msecs", title, msec);
    msec
}

#[wasm_bindgen]
pub fn _performance_test_example(name: &str) {
    _perf_loop(name, 10000, &|| {
        let _ctx = ctx("canvas1");
    });
}
