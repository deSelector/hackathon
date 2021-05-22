use crate::ctx2d::*;
use crate::grid::Grid;
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
const SIDE_COL_COUNT: u32 = 2;
const TOTAL_COL_COUNT: u32 = SIDE_COL_COUNT * 2;
const ROW_HEIGHT: u32 = 30;
const MARGIN: u32 = 0;

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

#[wasm_bindgen]
pub struct DOB {
    id: String,
    width: u32,
    height: u32,
}

#[wasm_bindgen]
impl DOB {
    pub fn new(id: String, width: u32, height: u32) -> DOB {
        DOB { id, width, height }
    }

    pub fn get_data_width() -> u32 {
        DATA_WIDTH
    }
}

impl DOB {
    pub fn paint(&self, bids: &[f64], asks: &[f64]) {
        let ctx = &ctx(&self.id);
        let grid = Grid::new(
            ctx,
            self.width,
            self.height,
            ROW_HEIGHT,
            TOTAL_COL_COUNT,
            DATA_WIDTH,
            MARGIN,
        );

        grid.assert_data_source(bids);
        grid.assert_data_source(asks);
        grid.draw_gridlines();

        grid.clip_begin();
        self.draw_book_side(&grid, bids, Side::Bid);
        self.draw_book_side(&grid, asks, Side::Ask);
        self.draw_cumulative(&grid, bids, asks);
        grid.clip_end();
    }

    fn draw_book_side(&self, grid: &Grid, data: &[f64], side: Side) {
        let row_count = (data.len() / DATA_WIDTH as usize) as u32;
        let col_width = grid.cell_width();
        let dx = self.start_x(grid, side);
        let align = self.cell_align(side);

        for r in 0.. {
            let y = grid.top() + (r * ROW_HEIGHT) as f64;
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

    fn draw_cumulative(&self, grid: &Grid, bids: &[f64], asks: &[f64]) {
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

    fn draw_cumulative_side(&self, grid: &Grid, data: &[f64], side: Side, ratio: f64) {
        let row_count = (data.len() / DATA_WIDTH as usize) as u32;
        if row_count <= 0 {
            return;
        }

        let dim = self.side_dim(grid, side);
        let ctx = grid.get_ctx();
        ctx.save();

        for r in 0.. {
            let y = grid.top() + (r * ROW_HEIGHT) as f64;
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

                fill_rect(ctx, x, y, len, ROW_HEIGHT as f64, color);
            } else {
                break;
            }
        }

        ctx.restore();
    }
}

impl DOB {
    fn side_dim(&self, grid: &Grid, side: Side) -> (f64, f64) {
        match side {
            Side::Bid => (grid.left(), grid.mid()),
            Side::Ask => (grid.mid(), grid.right()),
        }
    }

    fn last_row_value(&self, grid: &Grid, data: &[f64], field: Field) -> f64 {
        match data.len() {
            len if len > 0 => grid
                .cell_value(data, (len as i32 / DATA_WIDTH as i32) - 1, field as u32)
                .unwrap_or_default(),
            _ => 0.0,
        }
    }
    fn start_x(&self, grid: &Grid, side: Side) -> f64 {
        grid.left()
            + match side {
                Side::Bid => 0.0,
                Side::Ask => grid.cell_width() * SIDE_COL_COUNT as f64,
            }
    }

    fn cell_x(&self, grid: &Grid, side: Side, field: Field) -> f64 {
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
