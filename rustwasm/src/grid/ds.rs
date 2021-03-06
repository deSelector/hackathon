use crate::grid::column::*;
use crate::grid::renderer::*;
use crate::utils::hash_code;
use byteorder::{BigEndian, ByteOrder};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! _console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}
pub type Sparks = HashMap<u64, Vec<f64>>;

#[derive(Default)]
pub struct DataSource<'a> {
    pub data: &'a [SZ],
    pub data_width: usize,
    pub row_count: usize,
    sparks: Option<&'a Sparks>,
}

impl<'a> DataSource<'a> {
    pub fn new(data: &'a [SZ], data_width: usize, sparks: Option<&'a Sparks>) -> DataSource<'a> {
        assert!(data_width > 0);
        assert_eq!(
            data.len() as f64 % data_width as f64,
            0.0,
            "buffer size {} not divisible by {}",
            data.len(),
            data_width
        );

        DataSource {
            data,
            data_width,
            row_count: (data.len() / data_width),
            sparks,
            ..Default::default()
        }
    }
}

impl<'a> DataSource<'a> {
    pub fn get_value_f64(&self, row: usize, col: &Column) -> Option<f64> {
        let index = self.get_cell_index(row, col);
        match index {
            Some(i) => {
                // note: potential performance impact - verify.
                let v = BigEndian::read_f64(&self.data[i..i + num_size()]);
                Some(v)
            }
            _ => None,
        }
    }

    pub fn get_value_str(&self, row: usize, col: &Column) -> Option<String> {
        let index = self.get_cell_index(row, col);
        match index {
            Some(i) => {
                let str_slice = &self.data[i..i + col.size];
                let s = String::from_utf8_lossy(str_slice);
                Some(String::from(s.trim_end_matches(char::from(0))))
            }
            None => None, // String::from(""),
        }
    }
    pub fn get_sparkline(&self, row: usize, col: &Column) -> Option<&Vec<f64>> {
        if self.sparks.unwrap().len() > 0 {
            let id = self.get_value_str(row, col);
            if id.is_some() {
                return self.sparks.unwrap().get(&hash_code(&id.unwrap()));
            }
        }
        None
    }

    fn get_cell_index(&self, row: usize, col: &Column) -> Option<usize> {
        if self.row_count > 0 && row < self.row_count {
            let index = row * self.data_width + col.data_offset;
            if index < self.data.len() {
                return Some(index);
            }
            assert_lt!(
                index,
                self.data.len(),
                "buffer index {} out of bounds {}",
                index,
                self.data.len()
            );
        }
        None
    }
}
