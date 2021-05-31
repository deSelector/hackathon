#![allow(dead_code)]
use super::ctx2d::*;
use super::ds::*;
use crate::grid::column::*;
use crate::grid::schema::*;

use js_sys::Date;
use std::f64;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

pub const HEADER_LINES: usize = 1;
const HIGHLIGHT_DURATION: i64 = 100;
const ROW_HEIGHT: usize = 30;
const MARGIN: u32 = 0;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! _console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

pub type SZ = u8;

pub const fn num_size() -> usize {
    std::mem::size_of::<f64>()
}

#[derive(Default)]
pub struct GridRenderer<'a> {
    ctx: Option<&'a CanvasRenderingContext2d>,
    pub schema: Option<&'a Schema>,
    pub row_height: usize,
    pub margin: u32,
    left: u32,
    top: u32,
    width: u32,
    height: u32,
    col_width: f64, // todo: use col widths later
}

impl<'a> GridRenderer<'a> {
    pub fn new(
        ctx: &'a CanvasRenderingContext2d,
        schema: &'a Schema,
        left: u32,
        top: u32,
        width: u32,
        height: u32,
    ) -> GridRenderer<'a> {
        GridRenderer {
            ctx: Some(ctx),
            schema: Some(schema),
            left,
            top,
            width,
            height,
            row_height: ROW_HEIGHT,
            margin: MARGIN,
            ..Default::default()
        }
    }

    pub fn clear(&self) {
        let ctx = self.get_ctx();
        fill_rect(
            ctx,
            self.left as f64,
            self.top as f64,
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

        // red: #ff3b69
        set_fill_style(ctx, "#03c67a");
        set_text_baseline(ctx, "middle");
    }

    pub fn render(&mut self, ds: &DataSource) {
        self.calc_col_width();
        self.render_gridlines(ds);
        self.clip_begin();
        self.render_data(ds);
        self.render_header();
        self.clip_end();
    }

    pub fn render_gridlines(&self, ds: &DataSource) {
        let ctx = self.get_ctx();
        self.clear();

        ctx.begin_path();
        ctx.set_stroke_style(&"#232832".into());

        // Vertical lines.
        let mut index = 0;
        for _col in self.schema.unwrap().get_visible_cols() {
            let x = self.left() + (index as f64 * self.col_width()).floor();
            ctx.move_to(x, self.top());
            ctx.line_to(x, self.bottom());
            vertical_line(ctx, self.top(), self.bottom(), x);
            index += 1;
        }

        // Horizontal lines.
        let mut j = 0;
        loop {
            let y = self.top() + (j * self.row_height) as f64;
            if y < self.bottom() && j <= ds.row_count + HEADER_LINES {
                horizontal_line(ctx, self.left(), self.right(), y);
                j += 1;
            } else {
                break;
            }
        }

        // final bottom horizontal line
        horizontal_line(ctx, self.left(), self.right(), self.bottom());

        ctx.stroke();
    }

    pub fn render_header(&self) {
        let mut index = 0;
        for col in self.schema.unwrap().get_visible_cols() {
            let x = self.left() + self.cell_x(index);
            self.render_text_aligned(
                col.name.as_str(),
                x,
                0_f64,
                self.col_width(),
                "center",
                false,
            );
            index += 1;
        }
    }

    pub fn render_data(&self, ds: &DataSource) {
        let ts_col = self.schema.unwrap().get_col_by_type(ColumnType::Timestamp);

        for row in 0_usize.. {
            let y = self.top() + ((row + HEADER_LINES) * self.row_height) as f64;

            if y < self.bottom() as f64 && row < ds.row_count {
                let highlight = if ts_col.is_some() {
                    self.is_highlight(ds.get_value_f64(row, ts_col.unwrap()).unwrap_or_default())
                } else {
                    false
                };

                let mut index = 0;
                for col in self.schema.unwrap().get_visible_cols() {
                    let x = self.left() + self.cell_x(index);
                    self.render_text_formatted(ds, row, x, y, col, highlight);
                    index += 1;
                }
            } else {
                break;
            }
        }
    }

    fn render_text_formatted(
        &self,
        ds: &DataSource,
        row: usize,
        x: f64,
        y: f64,
        col: &Column,
        highlight: bool,
    ) {
        let mut align = "right";

        let v = match col.col_type {
            ColumnType::String => {
                align = "left";
                ds.get_value_str(row, col).unwrap_or("?".to_string())
            }
            _ => {
                let val = ds.get_value_f64(row, col).unwrap_or_default();
                col.format_value(val)
            }
        };

        self.render_text_aligned(&v, x, y, self.col_width(), align, highlight);
    }

    pub fn is_highlight(&self, time: f64) -> bool {
        let now = Date::new_0().get_time() as i64;
        now - time as i64 <= HIGHLIGHT_DURATION
    }

    pub fn render_highlight(&self, x: f64, y: f64, width: f64, time: f64) {
        let ctx = self.get_ctx();
        let now = Date::new_0().get_time() as i64;
        if now - time as i64 <= HIGHLIGHT_DURATION {
            ctx.save();
            fill_rect(ctx, x, y, width, self.row_height as f64, "#ffffff22");
            ctx.restore();
        }
    }

    pub fn render_text_aligned(
        &self,
        text: &str,
        x: f64,
        y: f64,
        width: f64,
        align: &str,
        highlight: bool,
    ) {
        fill_text_aligned(
            self.get_ctx(),
            text,
            x,
            y,
            width,
            self.row_height as f64,
            align,
            highlight,
        );
    }

    pub fn clip_begin(&self) {
        clip_begin(
            self.get_ctx(),
            self.left(),
            self.top(),
            self.client_width(),
            self.client_height(),
        );
    }

    pub fn clip_end(&self) {
        clip_end(self.get_ctx());
    }
}

impl<'a> GridRenderer<'a> {
    pub fn cell_x(&self, index: usize) -> f64 {
        index as f64 * self.col_width()
    }
}

impl<'a> GridRenderer<'a> {
    pub fn get_ctx(&self) -> &CanvasRenderingContext2d {
        self.ctx.unwrap()
    }
    pub fn calc_col_width(&mut self) {
        let visible_count = self.schema.unwrap().get_visible_row_count();
        assert!(visible_count > 0);
        self.col_width = self.client_width() / visible_count as f64;
    }
    pub fn col_width(&self) -> f64 {
        self.col_width
    }
    pub fn client_width(&self) -> f64 {
        (self.width - 2 * self.margin) as f64
    }
    pub fn client_height(&self) -> f64 {
        (self.height - 2 * self.margin) as f64
    }
    pub fn left(&self) -> f64 {
        (self.left + self.margin) as f64 + 0.5
    }
    pub fn top(&self) -> f64 {
        (self.top + self.margin) as f64 + 0.5
    }
    pub fn right(&self) -> f64 {
        (self.left + self.width - self.margin) as f64 - 0.5
    }
    pub fn bottom(&self) -> f64 {
        (self.top + self.height - self.margin) as f64 - 0.5
    }
    pub fn mid(&self) -> f64 {
        self.left() + ((self.client_width() / 2.0).round())
    }
}
