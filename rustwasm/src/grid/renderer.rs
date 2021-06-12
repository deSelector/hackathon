#![allow(dead_code)]
use super::ctx2d::*;
use super::ds::*;
use crate::grid::column::*;
use crate::grid::schema::*;
use crate::grid::sparkline::*;

use js_sys::Date;
use std::f64;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

pub const HEADER_LINES: usize = 1;
const HIGHLIGHT_DURATION: i64 = 100;
const ROW_HEIGHT: usize = 40;
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

pub fn is_highlighted(time: f64) -> bool {
    let now = Date::new_0().get_time() as i64;
    now - time as i64 <= HIGHLIGHT_DURATION
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
    top_index: usize,
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

    pub fn render(&mut self, ds: &DataSource, top_index: usize) {
        self.set_top_index(top_index);
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
        let mut col_index = 0;
        let last_y = self.data_bottom(ds.row_count - self.top_index);
        for _col in self.schema.unwrap().get_visible_cols() {
            let x = self.get_x(col_index);
            if x < self.right() {
                vertical_line(ctx, self.top(), last_y, x);
                col_index += 1;
            } else {
                break;
            }
        }

        // Horizontal lines.
        let mut row_index = 0;
        loop {
            let y = self.get_y(row_index);
            if y < self.bottom() && row_index <= ds.row_count - self.top_index + HEADER_LINES {
                horizontal_line(ctx, self.left(), self.right(), y);
                row_index += 1;
            } else {
                break;
            }
        }

        // final bottom/right lines
        horizontal_line(ctx, self.left(), self.right(), self.bottom());
        vertical_line(ctx, self.top(), self.bottom(), self.right());

        ctx.stroke();
    }

    pub fn render_header(&self) {
        let mut index = 0;
        for col in self.schema.unwrap().get_visible_cols() {
            let x = self.get_x(index);
            self.render_text(col.name.as_str(), x, 0_f64, "center", false);
            index += 1;
        }
    }

    pub fn render_data(&self, ds: &DataSource) {
        let ts_col = self.schema.unwrap().get_col_by_type(ColumnType::Timestamp);

        for row_index in 0_usize.. {
            let y = self.get_y(row_index + HEADER_LINES);
            let row = self.top_index + row_index;

            if y < self.bottom() && row < ds.row_count {
                let highlight = if ts_col.is_some() {
                    is_highlighted(ds.get_value_f64(row, ts_col.unwrap()).unwrap_or_default())
                } else {
                    false
                };

                let mut col_index = 0;
                for col in self.schema.unwrap().get_visible_cols() {
                    let x = self.get_x(col_index);
                    self.render_cell(ds, row, x, y, col, highlight && col.highlight);
                    col_index += 1;
                }
            } else {
                break;
            }
        }
    }

    fn render_cell(
        &self,
        ds: &DataSource,
        row: usize,
        x: f64,
        y: f64,
        col: &Column,
        highlight: bool,
    ) {
        if col.col_type == ColumnType::Sparkline {
            self.render_sparkline(ds, row, x, y, col);
            return;
        }

        let v = match col.col_type {
            ColumnType::String => ds.get_value_str(row, col),
            _ => col.format_value(ds.get_value_f64(row, col)),
        };

        if v.is_some() {
            let align = match col.align.as_str() {
                "" => match col.col_type {
                    ColumnType::String => "left",
                    ColumnType::Date | ColumnType::DateTime | ColumnType::Timestamp => "center",
                    _ => "right",
                },
                _ => col.align.as_str(),
            };
            self.render_text(&v.unwrap(), x, y, align, highlight);
        }
    }

    fn render_text(&self, text: &str, x: f64, y: f64, align: &str, highlight: bool) {
        fill_text_aligned(
            self.get_ctx(),
            text,
            x,
            y,
            self.col_width(),
            self.row_height as f64,
            align,
            highlight,
        );
    }

    fn render_sparkline(&self, ds: &DataSource, row: usize, x: f64, y: f64, col: &Column) {
        let data = ds.get_sparkline(row, col);
        if data.is_some() {
            let mut ss = Sparkline::new();
            ss.render(
                self.get_ctx(),
                x,
                y,
                self.col_width(),
                self.row_height as f64,
                &data.unwrap()[..],
            );
        }
    }

    pub fn _render_highlight(&self, x: f64, y: f64, width: f64, time: f64) {
        let ctx = self.get_ctx();
        let now = Date::new_0().get_time() as i64;
        if now - time as i64 <= HIGHLIGHT_DURATION {
            ctx.save();
            fill_rect(ctx, x, y, width, self.row_height as f64, "#ffffff22");
            ctx.restore();
        }
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
    pub fn get_ctx(&self) -> &CanvasRenderingContext2d {
        self.ctx.unwrap()
    }

    pub fn get_visible_col_count(&self) -> usize {
        self.schema.unwrap().visible_col_count
    }
    pub fn set_top_index(&mut self, top_index: usize) {
        self.top_index = top_index;
    }
    pub fn calc_col_width(&mut self) {
        let visible_count = self.schema.unwrap().get_visible_row_count();
        assert!(visible_count > 0);
        // laziness: enforce min column width until we support horizontal scroll
        self.col_width = std::cmp::max(100, self.client_width() as usize / visible_count) as f64;
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
    pub fn data_bottom(&self, row_count: usize) -> f64 {
        std::cmp::min(
            self.bottom() as usize,
            self.top as usize + (row_count + HEADER_LINES) * self.row_height,
        ) as f64
            - 0.5
    }
    pub fn mid(&self) -> f64 {
        self.left() + ((self.client_width() / 2.0).round())
    }

    pub fn get_x(&self, col_index: usize) -> f64 {
        self.left() + (col_index as f64 * self.col_width()).floor()
    }

    pub fn get_y(&self, row_index: usize) -> f64 {
        self.top() + (row_index * self.row_height) as f64
    }
}
