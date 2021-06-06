use super::grid::ctx2d::*;
use super::grid::ds::*;
use super::grid::renderer::*;
use super::grid::schema::*;
use crate::grid::column::*;

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

const CUM_SIZE_COL_ID: &str = "cumSize"; // don't change it - used by the UI demo

#[wasm_bindgen]
#[derive(Default)]
pub struct DOB {
    id: String,
    bid_schema: Schema,
    ask_schema: Schema,
    top_index: usize,
}

#[wasm_bindgen]
impl DOB {
    pub fn new(id: String, schema_obj: &JsValue) -> DOB {
        set_panic_hook();
        let (bid_schema, ask_schema) = DOB::set_schema(schema_obj);
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

        let mut bid_panel = (
            GridRenderer::new(ctx, &self.bid_schema, left, top, width / 2, height),
            DataSource::new(bids, data_width, None),
            Side::Bid,
        );
        let mut ask_panel = (
            GridRenderer::new(
                ctx,
                &self.ask_schema,
                left + width / 2 + 1,
                top,
                width / 2,
                height,
            ),
            DataSource::new(asks, data_width, None),
            Side::Ask,
        );

        let ratio = self.calc_bid_ask_ratio(
            &bid_panel.1,
            &ask_panel.1,
            bid_panel.0.client_width(),
            self.bid_schema.get_col_by_id(CUM_SIZE_COL_ID).unwrap(),
        );

        for (grid, ds, side) in [&mut bid_panel, &mut ask_panel].iter_mut() {
            grid.set_top_index(self.top_index);
            grid.calc_col_width();
            grid.render_gridlines(ds);

            grid.clip_begin();
            grid.render_data(ds);
            self.render_pyramid(grid, ds, *side, ratio);
            grid.render_header();
            grid.clip_end();
        }
    }

    fn set_schema(obj: &JsValue) -> (Schema, Schema) {
        console_error_panic_hook::set_once();
        let mut bid_schema = obj.into_serde::<Schema>().unwrap();
        normalize_schema(&mut bid_schema);
        // flip the ask side columns and alignments
        let mut ask_schema = bid_schema.clone();
        ask_schema.cols.reverse();
        for mut col in &mut ask_schema.cols {
            col.align = "left".to_string();
        }
        (bid_schema, ask_schema)
    }
    pub fn set_top_index(&mut self, top_index: usize) {
        self.top_index = top_index;
    }
}

impl DOB {
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

    fn render_pyramid(&self, gr: &GridRenderer, ds: &DataSource, side: Side, ratio: f64) {
        if ds.row_count == 0 {
            return;
        }
        let cum_col = gr.schema.unwrap().get_col_by_id(CUM_SIZE_COL_ID).unwrap();
        let ctx = gr.get_ctx();
        ctx.save();

        for row in 0_usize.. {
            let y = gr.top() + ((row + HEADER_LINES) * gr.row_height) as f64;
            let index = self.top_index + row;
            if y < gr.bottom() && index < ds.row_count {
                let len = ds.get_value_f64(index, cum_col).unwrap_or_default() * ratio;
                let x = match side {
                    Side::Bid => gr.right() - len,
                    Side::Ask => gr.left(),
                };

                let color = match side {
                    Side::Bid => "#0c433899",
                    Side::Ask => "#ff3b6960",
                };

                fill_rect(ctx, x, y, len, gr.row_height as f64, color);
            } else {
                break;
            }
        }

        ctx.restore();
    }

    fn get_max_cum_size(&self, ds: &DataSource, col: &Column) -> f64 {
        ds.get_value_f64(ds.row_count - 1, col).unwrap_or_default()
    }
}
