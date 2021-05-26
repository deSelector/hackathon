use crate::grid::core::*;
use crate::grid::ctx2d::*;
use crate::grid::schema::Schema;

use crate::utils::*;
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

#[derive(PartialEq, Copy, Clone)]
enum Side {
    Bid = 0,
    Ask = 1,
}

#[wasm_bindgen]
#[derive(Default)]
pub struct DOB {
    id: String,
    bid_schema: Schema,
    ask_schema: Schema,
    pub width: u32,
    pub height: u32,
    pub data_width: u32,
}

#[wasm_bindgen]
impl DOB {
    pub fn new(id: String, width: u32, height: u32) -> DOB {
        set_panic_hook();
        DOB {
            id,
            width,
            height,
            data_width: 1,
            ..Default::default()
        }
    }

    pub fn render(&self, bids: &[f64], asks: &[f64]) {
        let ctx = &ctx(&self.id);

        let mut left = (
            GridCore::new(ctx, 0, 0, self.width / 2, self.height, self.data_width),
            bids,
            Side::Bid,
            &self.bid_schema,
        );
        let mut right = (
            GridCore::new(
                ctx,
                self.width / 2 + 1,
                0,
                self.width / 2,
                self.height,
                self.data_width,
            ),
            asks,
            Side::Ask,
            &self.ask_schema,
        );

        let ratio = self.calc_bid_side_ratio(&left.0, &left.1, &right.0, &right.1);

        for (grid, data, side, schema) in [&mut left, &mut right].iter_mut() {
            grid.col_count = 2; // schema.cols.len() as u32;
            grid.assert_data_source(data);
            grid.draw_gridlines();

            grid.clip_begin();
            self.render_book(grid, data, schema, *side);
            self.render_pyramid(grid, data, *side, ratio);
            grid.clip_end();
        }
    }

    pub fn set_schema(&mut self, bid: &JsValue, ask: &JsValue) {
        console_error_panic_hook::set_once();
        self.bid_schema = bid.into_serde::<Schema>().unwrap_or_default();
        self.ask_schema = ask.into_serde::<Schema>().unwrap_or_default();
    }
}

impl DOB {
    fn render_book(&self, grid: &GridCore, data: &[f64], schema: &Schema, side: Side) {
        let row_count = (data.len() / self.data_width as usize) as u32;
        let col_width = grid.cell_width();
        let dx = grid.left();

        let align = self.cell_align(side); // todo: add align to schema and remove

        for r in 0.. {
            let y = grid.top() + (r * grid.row_height) as f64;
            if y < grid.bottom() as f64 && r < row_count {
                let mut i = 0;
                for c in &schema.cols {
                    if !c.hidden {
                        let x = dx + grid.cell_x(i);
                        let v = grid
                            .cell_value(data, r as i32, c.data_offset)
                            .unwrap_or_default();

                        let hi = grid.is_highlight(
                            grid.cell_value(data, r as i32, 3 /*Field::Time as u32*/)
                                .unwrap_or_default(),
                        );
                        grid.fill_text_aligned(
                            &format_args!("{:.*}", c.precision as usize, v).to_string(),
                            x,
                            y,
                            col_width,
                            align,
                            hi,
                        );
                        i += 1;
                    }
                }
            } else {
                break;
            }
        }
    }

    fn calc_bid_side_ratio(
        &self,
        left_grid: &GridCore,
        left_data: &[f64],
        right_grid: &GridCore,
        right_data: &[f64],
    ) -> f64 {
        let max_cumulative_value = std::cmp::max(
            left_grid.last_row_value(left_data, 2 /*Field::CumSize*/) as u32,
            right_grid.last_row_value(right_data, 2 /*Field::CumSize*/) as u32,
        ) as f64;

        if max_cumulative_value > 0.0 {
            left_grid.client_width() / max_cumulative_value / 2.0
        } else {
            0.0
        }
    }

    fn render_pyramid(&self, grid: &GridCore, data: &[f64], side: Side, ratio: f64) {
        let row_count = (data.len() / self.data_width as usize) as u32;
        if row_count <= 0 {
            return;
        }

        let ctx = grid.get_ctx();
        ctx.save();

        for r in 0.. {
            let y = grid.top() + (r * grid.row_height) as f64;
            if y < grid.bottom() as f64 && r < row_count {
                let len = grid
                    .cell_value(data, r as i32, 2 /*Field::CumSize as u32*/)
                    .unwrap_or_default()
                    * ratio;
                let x = match side {
                    Side::Bid => grid.right() - len,
                    Side::Ask => grid.left(),
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
    fn cell_align(&self, side: Side) -> &str {
        match side {
            Side::Bid => "right",
            Side::Ask => "left",
        }
    }
}
