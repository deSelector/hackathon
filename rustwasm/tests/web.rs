//! Test suite for the Web and headless browsers.

//! wasm-pack test --chrome --headless
//! wasm-pack build  

#![cfg(target_arch = "wasm32")]

extern crate rustwasm;
use rustwasm::grid::ctrl::*;

extern crate wasm_bindgen_test;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[cfg(test)]
pub fn init_grid() -> Grid {
    let grid = Grid::new("canvas".to_string(), &JsValue::NULL);
    grid
}

#[wasm_bindgen_test]
pub fn grid_dimensions() {
    let grid = init_grid();
    assert_eq!(grid.get_id(), "canvas".to_string());
}
