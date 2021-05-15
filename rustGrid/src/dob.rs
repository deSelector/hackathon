use crate::ctx2d::*;
use crate::grid::Grid;
use crate::utils::*;
use enum_iterator::IntoEnumIterator;
use std::f64;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

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
const MARGIN: u32 = 5;

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
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
    }
}

impl DOB {
    fn cell_width(&self) -> f64 {
        self.client_width() / TOTAL_COL_COUNT as f64
    }
    fn client_width(&self) -> f64 {
        (self.width - 2 * MARGIN) as f64
    }
    fn client_height(&self) -> f64 {
        (self.height - 2 * MARGIN) as f64
    }
    fn left(&self) -> f64 {
        MARGIN as f64 + 0.5
    }
    fn top(&self) -> f64 {
        MARGIN as f64 + 0.5
    }
    fn right(&self) -> f64 {
        (self.width - MARGIN) as f64 - 0.5
    }
    fn bottom(&self) -> f64 {
        (self.height - MARGIN) as f64 - 0.5
    }

    fn mid(&self) -> f64 {
        self.left() + ((self.client_width() / 2.0) as i32) as f64
    }

    fn side_dim(&self, side: Side) -> (f64, f64) {
        match side {
            Side::Bid => (self.left(), self.mid()),
            Side::Ask => (self.mid(), self.right()),
        }
    }
}

impl DOB {
    fn draw_grid(&self) {
        let ctx = &ctx(&self.id);

        fill_rect(
            ctx,
            0.0,
            0.0,
            self.width as f64,
            self.height as f64,
            &"#0b0e17",
        );

        fill_rect(
            ctx,
            self.left(),
            self.top(),
            self.client_width(),
            self.client_height(),
            &"#101722",
        );

        ctx.begin_path();
        ctx.set_stroke_style(&"#232832".into());

        let col_width = self.cell_width();

        // Vertical lines.
        {
            for i in 0..TOTAL_COL_COUNT + 1 {
                let x = self.left() + (i as f64 * col_width).floor();
                ctx.move_to(x, self.top());
                ctx.line_to(x, self.bottom());
                vertical_line(ctx, self.top(), self.bottom(), x);
            }
        }

        {
            // Horizontal lines.
            let mut j = 0;
            loop {
                let y = self.top() + (j * ROW_HEIGHT) as f64;
                if y < self.bottom() {
                    horizontal_line(ctx, self.left(), self.right(), y);
                    j += 1;
                } else {
                    break;
                }
            }
            horizontal_line(ctx, self.left(), self.right(), self.bottom());
        }

        ctx.stroke();
    }

    pub fn paint(&self, bids: &[f64], asks: &[f64]) {
        let ctx = &ctx(&self.id);

        self.draw_grid();

        // red: #ff3b69
        set_fill_style(ctx, "#03c67a");
        set_text_baseline(ctx, "middle");

        clip_begin(
            ctx,
            self.left(),
            self.top(),
            self.client_width(),
            self.client_height(),
        );

        self.draw_book_side(ctx, bids, Side::Bid);
        self.draw_book_side(ctx, asks, Side::Ask);
        self.draw_cumulative(ctx, bids, asks);

        clip_end(ctx);
    }

    fn draw_book_side(&self, ctx: &CanvasRenderingContext2d, data: &[f64], side: Side) {
        let row_count = (data.len() / DATA_WIDTH as usize) as u32;
        let col_width = self.cell_width();
        let dx = self.start_x(side);
        let align = self.cell_align(side);

        assert_eq!(
            data.len() as f64 % DATA_WIDTH as f64,
            0.0,
            "buffer size {} not divisible by {}",
            data.len(),
            DATA_WIDTH
        );

        for r in 0.. {
            let y = self.top() + (r * ROW_HEIGHT) as f64;
            if y < self.bottom() as f64 && r < row_count {
                for &field in [Field::Price, Field::Size].iter() {
                    let x = dx + self.cell_x(side, field);
                    let v = self.cell_value(data, r as i32, field).unwrap_or_default();

                    Grid::draw_highlight(
                        ctx,
                        x,
                        y,
                        col_width,
                        ROW_HEIGHT as f64,
                        self.cell_value(data, r as i32, Field::Time)
                            .unwrap_or_default(),
                    );
                    fill_text_aligned(
                        ctx,
                        &format_args!("{:.*}", self.cell_precision(field), v).to_string(),
                        x,
                        y,
                        col_width,
                        ROW_HEIGHT as f64,
                        align,
                    );
                }
            } else {
                break;
            }
        }
    }

    fn draw_cumulative(&self, ctx: &CanvasRenderingContext2d, bids: &[f64], asks: &[f64]) {
        let max_cumulative_value = std::cmp::max(
            self.last_row_value(bids, Field::CumSize) as u32,
            self.last_row_value(asks, Field::CumSize) as u32,
        ) as f64;

        if max_cumulative_value > 0.0 {
            let ratio = self.client_width() / max_cumulative_value / 2.0;
            self.draw_cumulative_side(ctx, bids, Side::Bid, ratio);
            self.draw_cumulative_side(ctx, asks, Side::Ask, ratio);
        }
    }

    fn draw_cumulative_side(
        &self,
        ctx: &CanvasRenderingContext2d,
        data: &[f64],
        side: Side,
        ratio: f64,
    ) {
        let row_count = (data.len() / DATA_WIDTH as usize) as u32;
        if row_count <= 0 {
            return;
        }

        let dim = self.side_dim(side);

        ctx.save();

        for r in 0.. {
            let y = self.top() + (r * ROW_HEIGHT) as f64;
            if y < self.bottom() as f64 && r < row_count {
                let len = self
                    .cell_value(data, r as i32, Field::CumSize)
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
    fn cell_value(&self, data: &[f64], row: i32, field: Field) -> Option<f64> {
        match row {
            row if row >= 0 => {
                let index = row * DATA_WIDTH as i32 + field as i32;
                assert_lt!(
                    index as usize,
                    data.len(),
                    "buffer index {} out of bounds {}",
                    index,
                    data.len()
                );
                return Some(data[index as usize]);
            }
            _ => None,
        }
    }

    fn last_row_value(&self, data: &[f64], field: Field) -> f64 {
        match data.len() {
            len if len > 0 => self
                .cell_value(data, (len as i32 / DATA_WIDTH as i32) - 1, field)
                .unwrap_or_default(),
            _ => 0.0,
        }
    }
    fn start_x(&self, side: Side) -> f64 {
        self.left()
            + match side {
                Side::Bid => 0.0,
                Side::Ask => self.cell_width() * SIDE_COL_COUNT as f64,
            }
    }

    fn cell_x(&self, side: Side, field: Field) -> f64 {
        match side {
            Side::Bid => match field {
                Field::Size => 0.0,
                Field::Price => self.cell_width(),
                _ => 0.0,
            },
            Side::Ask => match field {
                Field::Price => 0.0,
                Field::Size => self.cell_width(),
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
