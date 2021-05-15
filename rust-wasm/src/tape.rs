use crate::grid::Grid;
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
const COL_COUNT: u32 = 3;
const ROW_HEIGHT: u32 = 30;
const MARGIN: u32 = 0;

#[derive(PartialEq, Copy, Clone, IntoEnumIterator)]
pub enum Field {
    Price = 0,
    Size = 1,
    Time = 2,
}

#[wasm_bindgen]
pub struct Tape {
    id: String,
    width: u32,
    height: u32,
}

#[wasm_bindgen]
impl Tape {
    pub fn new(id: String, width: u32, height: u32) -> Tape {
        Tape { id, width, height }
    }

    pub fn get_data_width() -> u32 {
        DATA_WIDTH
    }
}

impl Tape {
    pub fn paint(&self, trades: &[f64]) {
        let ctx = &ctx(&self.id);
        let grid = Grid::new(
            ctx,
            self.width,
            self.height,
            ROW_HEIGHT,
            COL_COUNT,
            DATA_WIDTH,
            MARGIN,
        );
        grid.assert_data_source(trades);
        grid.draw_grid();

        grid.clip_begin();
        self.draw_tape(&grid, trades);
        grid.clip_end();
    }

    fn draw_tape(&self, grid: &Grid, data: &[f64]) {
        let row_count = (data.len() / DATA_WIDTH as usize) as u32;
        let col_width = grid.cell_width();

        for r in 0.. {
            let y = grid.top() + (r * ROW_HEIGHT) as f64;
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

impl Tape {
    fn format_value(&self, value: f64, field: Field) -> String {
        match field {
            Field::Time => Local
                .timestamp(value as i64 / 1000, 0)
                .format("%r")
                .to_string(),
            _ => format_args!("{:.*}", self.cell_precision(field), value).to_string(),
        }
    }

    fn cell_x(&self, grid: &Grid, field: Field) -> f64 {
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
