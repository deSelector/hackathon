use crate::grid::core::*;
use crate::grid::schema::*;
use crate::utils::*;
use chrono::prelude::*;
use chrono::Local;
use enum_iterator::IntoEnumIterator;
use std::f64;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! _console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

const DATA_WIDTH: u32 = 3; // price, size, time

#[derive(PartialEq, Copy, Clone, IntoEnumIterator)]
pub enum Field {
    Price = 0,
    Size = 1,
    Time = 2,
}

#[wasm_bindgen]
#[derive(Default)]
pub struct Grid {
    id: String,
    schema: Schema,
    col_count: u32,
    pub width: u32,
    pub height: u32,
}

#[wasm_bindgen]
impl Grid {
    pub fn new(id: String, width: u32, height: u32) -> Grid {
        set_panic_hook();
        Grid {
            id,
            width,
            height,
            ..Default::default()
        }
    }

    pub fn get_data_width() -> u32 {
        DATA_WIDTH
    }

    pub fn render(&self, trades: &[f64]) {
        let ctx = &ctx(&self.id);
        let mut grid = GridCore::new(ctx, self.width, self.height, DATA_WIDTH);
        grid.col_count = self.col_count;
        grid.assert_data_source(trades);
        grid.draw_gridlines();

        grid.clip_begin();
        self.do_render(&grid, trades);
        grid.clip_end();
    }

    pub fn set_schema(&mut self, obj: &JsValue) {
        console_error_panic_hook::set_once();
        self.schema = obj.into_serde::<Schema>().unwrap();
        _console_log!("SCHEMA: {:?}, el={:?}", obj, self.schema);
        self.col_count = self.schema.cols.len() as u32;
    }
}

impl Grid {
    fn do_render(&self, grid: &GridCore, data: &[f64]) {
        let row_count = (data.len() / DATA_WIDTH as usize) as u32;
        let col_width = grid.cell_width();

        for r in 0.. {
            let y = grid.top() + (r * grid.row_height) as f64;
            if y < grid.bottom() as f64 && r < row_count {
                for &field in [Field::Price, Field::Size, Field::Time].iter() {
                    let x = grid.left() + self.cell_x(grid, field);
                    let v = grid
                        .cell_value(data, r as i32, field as u32)
                        .unwrap_or_default();
                    grid.draw_highlight(
                        x,
                        y,
                        col_width,
                        grid.cell_value(data, r as i32, Field::Time as u32)
                            .unwrap_or_default(),
                    );
                    grid.fill_text_aligned(&self.format_value(v, field), x, y, col_width, "right");
                }
            } else {
                break;
            }
        }
    }
}

impl Grid {
    fn format_value(&self, value: f64, field: Field) -> String {
        match field {
            Field::Time => Local
                .timestamp(value as i64 / 1000, 0)
                .format("%r")
                .to_string(),
            _ => format_args!("{:.*}", self.cell_precision(field), value).to_string(),
        }
    }

    fn cell_x(&self, grid: &GridCore, field: Field) -> f64 {
        match field {
            Field::Price => 0.0,
            Field::Size => grid.cell_width(),
            Field::Time => grid.cell_width() * 2.0,
        }
    }

    fn cell_precision(&self, field: Field) -> usize {
        match field {
            Field::Price => 3,
            Field::Size => 5,
            _ => 0,
        }
    }
}
