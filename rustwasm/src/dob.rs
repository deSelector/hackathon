use super::grid::core::*;
use super::grid::ctx2d::*;
use super::grid::ds::*;
use super::grid::schema::*;

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

enum Field {
    CumSize = 3, // note: keep it in synch with column id schema config
}

#[wasm_bindgen]
#[derive(Default)]
pub struct DOB {
    id: String,
    bid_schema: Schema,
    ask_schema: Schema,
}

#[wasm_bindgen]
impl DOB {
    pub fn new(id: String, left_schema: &JsValue, right_schema: &JsValue) -> DOB {
        set_panic_hook();
        let (bid_schema, ask_schema) = DOB::set_schema(left_schema, right_schema);
        DOB {
            id,
            bid_schema,
            ask_schema,
            ..Default::default()
        }
    }

    pub fn render(
        &mut self,
        bids: &[SZ],
        asks: &[SZ],
        data_width: usize,
        left: u32,
        top: u32,
        width: u32,
        height: u32,
    ) {
        let ctx = &ctx(&self.id);

        let mut left_panel = (
            GridCore::new(ctx, &self.bid_schema, left, top, width / 2, height),
            DataSource::new(bids, data_width),
            Side::Bid,
        );
        let mut right_panel = (
            GridCore::new(
                ctx,
                &self.ask_schema,
                left + width / 2 + 1,
                top,
                width / 2,
                height,
            ),
            DataSource::new(asks, data_width),
            Side::Ask,
        );

        let ratio = self.calc_bid_ask_ratio(
            &left_panel.1,
            &right_panel.1,
            left_panel.0.client_width(),
            left_panel.0.get_col_by_id(Field::CumSize as u32).unwrap(),
        );

        for (grid, ds, side) in [&mut left_panel, &mut right_panel].iter_mut() {
            grid.calc_col_width();
            grid.render_gridlines(&ds);

            grid.clip_begin();
            self.render_book(grid, ds, *side);
            self.render_pyramid(grid, ds, *side, ratio);
            grid.render_header();
            grid.clip_end();
        }
    }

    fn set_schema(bid: &JsValue, ask: &JsValue) -> (Schema, Schema) {
        console_error_panic_hook::set_once();
        let bid_schema = bid.into_serde::<Schema>().unwrap();
        assert_schema(&bid_schema);
        let ask_schema = ask.into_serde::<Schema>().unwrap();
        assert_schema(&ask_schema);
        (bid_schema, ask_schema)
    }
}

impl DOB {
    fn render_book(&self, grid: &GridCore, ds: &DataSource, side: Side) {
        let dx = grid.left();
        let ts_col = grid.get_col_by_type(ColumnType::Timestamp).unwrap();
        let align = self.cell_align(side); // todo: add align to schema and remove

        for r in 0_usize.. {
            let y = grid.top() + ((r + HEADER_LINES) * grid.row_height) as f64;
            if y < grid.bottom() && r < ds.row_count {
                let mut i = 0;
                for col in &grid.schema.unwrap().cols {
                    if !col.hidden {
                        let x = dx + grid.cell_x(i);
                        let v = GridCore::get_value_f64(ds, r, col).unwrap_or_default();

                        let hi = grid.is_highlight(
                            GridCore::get_value_f64(ds, r, ts_col).unwrap_or_default(),
                        );
                        grid.fill_text_aligned(
                            &format_args!("{:.*}", col.precision, v).to_string(),
                            x,
                            y,
                            grid.col_width(),
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

    fn calc_bid_ask_ratio(
        &self,
        left_ds: &DataSource,
        right_ds: &DataSource,
        client_width: f64,
        cum_col: &Column,
    ) -> f64 {
        let max_cumulative_value = std::cmp::max(
            self.get_max_cum_size(left_ds, cum_col) as u32,
            self.get_max_cum_size(right_ds, cum_col) as u32,
        ) as f64;

        return if max_cumulative_value > 0.0 {
            client_width / max_cumulative_value
        } else {
            0.0
        };
    }

    fn render_pyramid(&self, grid: &GridCore, ds: &DataSource, side: Side, ratio: f64) {
        if ds.row_count == 0 {
            return;
        }
        let cum_col = grid.get_col_by_id(Field::CumSize as u32).unwrap();
        let ctx = grid.get_ctx();
        ctx.save();

        for r in 0_usize.. {
            let y = grid.top() + ((r + HEADER_LINES) * grid.row_height) as f64;
            if y < grid.bottom() && r < ds.row_count {
                let len = GridCore::get_value_f64(ds, r, cum_col).unwrap_or_default() * ratio;
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

    fn get_max_cum_size(&self, ds: &DataSource, col: &Column) -> f64 {
        GridCore::get_value_f64(ds, ds.row_count - 1, col).unwrap_or_default()
    }
}
