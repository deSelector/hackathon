use crate::grid::core::*;
use crate::grid::ctx2d::*;
use crate::grid::schema::Schema;

use crate::utils::*;
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

const DATA_WIDTH: u32 = 4; // price, size, cumSize, time

#[derive(PartialEq, Copy, Clone)]
pub enum Side {
    Bid = 0,
    Ask = 1,
}

#[derive(PartialEq, Copy, Clone, IntoEnumIterator)]
pub enum Field {
    Price = 0,
    Size = 1,
    CumSize = 2,
    Time = 3,
}

// todo: consolidate with generic Grid
#[wasm_bindgen]
#[derive(Default)]
pub struct DOB {
    id: String,
    schema: Schema,
    col_count: u32,
    pub width: u32,
    pub height: u32,
}

#[wasm_bindgen]
impl DOB {
    pub fn new(id: String, width: u32, height: u32) -> DOB {
        set_panic_hook();
        DOB {
            id,
            width,
            height,
            ..Default::default()
        }
    }

    pub fn get_data_width() -> u32 {
        DATA_WIDTH
    }

    pub fn paint(&self, bids: &[f64], asks: &[f64]) {
        let ctx = &ctx(&self.id);
        let mut grid = GridCore::new(ctx, self.width, self.height, DATA_WIDTH);

        grid.col_count = self.col_count * 2;
        grid.assert_data_source(bids);
        grid.assert_data_source(asks);
        grid.draw_gridlines();

        grid.clip_begin();
        self.draw_book_side(&grid, bids, Side::Bid);
        self.draw_book_side(&grid, asks, Side::Ask);
        self.draw_cumulative(&grid, bids, asks);
        grid.clip_end();
    }

    pub fn set_schema(&mut self, obj: &JsValue) {
        console_error_panic_hook::set_once();
        self.schema = obj.into_serde::<Schema>().unwrap();
        _console_log!("SCHEMA: {:?}, el={:?}", obj, self.schema);
        self.col_count = self.schema.cols.len() as u32;
    }
}

impl DOB {
    fn draw_book_side(&self, grid: &GridCore, data: &[f64], side: Side) {
        let row_count = (data.len() / DATA_WIDTH as usize) as u32;
        let col_width = grid.cell_width();
        let dx = self.start_x(grid, side);
        let align = self.cell_align(side);

        for r in 0.. {
            let y = grid.top() + (r * grid.row_height) as f64;
            if y < grid.bottom() as f64 && r < row_count {
                for &field in [Field::Price, Field::Size].iter() {
                    let x = dx + self.cell_x(grid, side, field);
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
                    grid.fill_text_aligned(
                        &format_args!("{:.*}", self.cell_precision(field), v).to_string(),
                        x,
                        y,
                        col_width,
                        align,
                    );
                }
            } else {
                break;
            }
        }
    }

    fn draw_cumulative(&self, grid: &GridCore, bids: &[f64], asks: &[f64]) {
        let max_cumulative_value = std::cmp::max(
            self.last_row_value(grid, bids, Field::CumSize) as u32,
            self.last_row_value(grid, asks, Field::CumSize) as u32,
        ) as f64;

        if max_cumulative_value > 0.0 {
            let ratio = grid.client_width() / max_cumulative_value / 2.0;
            self.draw_cumulative_side(grid, bids, Side::Bid, ratio);
            self.draw_cumulative_side(grid, asks, Side::Ask, ratio);
        }
    }

    fn draw_cumulative_side(&self, grid: &GridCore, data: &[f64], side: Side, ratio: f64) {
        let row_count = (data.len() / DATA_WIDTH as usize) as u32;
        if row_count <= 0 {
            return;
        }

        let dim = self.side_dim(grid, side);
        let ctx = grid.get_ctx();
        ctx.save();

        for r in 0.. {
            let y = grid.top() + (r * grid.row_height) as f64;
            if y < grid.bottom() as f64 && r < row_count {
                let len = grid
                    .cell_value(data, r as i32, Field::CumSize as u32)
                    .unwrap_or_default()
                    * ratio;
                let x = match side {
                    Side::Bid => dim.1 - len,
                    Side::Ask => dim.0,
                };

                let color = match side {
                    Side::Bid => "#0c433899",
                    Side::Ask => "#ff3b6960",
                };

                fill_rect(ctx, x, y, len, grid.row_height as f64, color);
            } else {
                break;
            }
        }

        ctx.restore();
    }
}

impl DOB {
    fn side_dim(&self, grid: &GridCore, side: Side) -> (f64, f64) {
        match side {
            Side::Bid => (grid.left(), grid.mid()),
            Side::Ask => (grid.mid(), grid.right()),
        }
    }

    fn last_row_value(&self, grid: &GridCore, data: &[f64], field: Field) -> f64 {
        match data.len() {
            len if len > 0 => grid
                .cell_value(data, (len as i32 / DATA_WIDTH as i32) - 1, field as u32)
                .unwrap_or_default(),
            _ => 0.0,
        }
    }
    fn start_x(&self, grid: &GridCore, side: Side) -> f64 {
        grid.left()
            + match side {
                Side::Bid => 0.0,
                Side::Ask => grid.cell_width() * self.col_count as f64,
            }
    }

    fn cell_x(&self, grid: &GridCore, side: Side, field: Field) -> f64 {
        match side {
            Side::Bid => match field {
                Field::Size => 0.0,
                Field::Price => grid.cell_width(),
                _ => 0.0,
            },
            Side::Ask => match field {
                Field::Price => 0.0,
                Field::Size => grid.cell_width(),
                _ => 0.0,
            },
        }
    }

    fn cell_align(&self, side: Side) -> &str {
        match side {
            Side::Bid => "right",
            Side::Ask => "left",
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
