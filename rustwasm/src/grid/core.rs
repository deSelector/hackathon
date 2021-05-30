#![allow(dead_code)]
use super::ctx2d::*;
use super::ds::*;
use byteorder::{BigEndian, ByteOrder};
use js_sys::Date;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

pub const HEADER_LINES: u32 = 1;
const HIGHLIGHT_DURATION: i64 = 100;
const ROW_HEIGHT: u32 = 30;
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
pub struct GridCore<'a> {
    ctx: Option<&'a CanvasRenderingContext2d>,
    left: u32,
    top: u32,
    width: u32,
    height: u32,
    pub row_height: u32,
    pub col_count: u32,
    pub margin: u32,
}

impl<'a> GridCore<'a> {
    pub fn new(
        ctx: &'a CanvasRenderingContext2d,
        left: u32,
        top: u32,
        width: u32,
        height: u32,
    ) -> GridCore {
        GridCore {
            ctx: Some(ctx),
            left,
            top,
            width,
            height,
            row_height: ROW_HEIGHT,
            margin: MARGIN,
            ..Default::default()
        }
    }

    pub fn get_cell_index(ds: &DataSource, row: i32, col_data_offset: u32) -> i32 {
        row * ds.data_width as i32 + col_data_offset as i32
    }

    pub fn get_value_f64(ds: &DataSource, row: i32, col_data_offset: u32) -> Option<f64> {
        match row {
            row if row >= 0 && ds.data.len() > 0 => {
                let index = GridCore::get_cell_index(ds, row, col_data_offset);
                GridCore::assert_index(ds, index);
                // note: potential performance impact - verify.
                let v = BigEndian::read_f64(&ds.data[index as usize..index as usize + num_size()]);
                Some(v)
            }
            _ => None,
        }
    }

    pub fn cell_value_f64(&self, ds: &DataSource, row: i32, col_data_offset: u32) -> Option<f64> {
        GridCore::get_value_f64(ds, row, col_data_offset)
    }

    pub fn cell_value_str(
        &self,
        ds: &DataSource,
        row: i32,
        col_data_offset: u32,
        col_data_len: usize,
    ) -> Option<String> {
        match row {
            row if row >= 0 && row < ds.row_count as i32 => {
                let index = GridCore::get_cell_index(ds, row, col_data_offset);
                GridCore::assert_index(ds, index);
                let start = index as usize;
                let str_slice = &ds.data[start..start + col_data_len as usize]; // todo
                let s = String::from_utf8_lossy(str_slice);
                return Some(s.to_string());
            }
            _ => None,
        }
    }

    pub fn cell_x(&self, index: usize) -> f64 {
        index as f64 * self.cell_width()
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
            self.ctx.unwrap(),
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

    pub fn draw_gridlines(&self, ds: &DataSource) {
        let ctx = self.get_ctx();
        self.clear();

        ctx.begin_path();
        ctx.set_stroke_style(&"#232832".into());

        let col_width = self.cell_width();

        // Vertical lines.
        for i in 0..self.col_count + 1 {
            let x = self.left() + (i as f64 * col_width).floor();
            ctx.move_to(x, self.top());
            ctx.line_to(x, self.bottom());
            vertical_line(ctx, self.top(), self.bottom(), x);
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

    pub fn is_highlight(&self, time: f64) -> bool {
        let now = Date::new_0().get_time() as i64;
        now - time as i64 <= HIGHLIGHT_DURATION
    }

    pub fn draw_highlight(&self, x: f64, y: f64, width: f64, time: f64) {
        let ctx = self.get_ctx();
        let now = Date::new_0().get_time() as i64;
        if now - time as i64 <= HIGHLIGHT_DURATION {
            ctx.save();
            fill_rect(ctx, x, y, width, self.row_height.into(), "#ffffff22");
            ctx.restore();
        }
    }

    pub fn fill_text_aligned(
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
            self.row_height.into(),
            align,
            highlight,
        );
    }
}

impl<'a> GridCore<'a> {
    pub fn get_ctx(&self) -> &CanvasRenderingContext2d {
        self.ctx.unwrap()
    }
    pub fn cell_width(&self) -> f64 {
        self.client_width() / self.col_count as f64
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
        self.left() + ((self.client_width() / 2.0) as i32) as f64
    }
}

impl<'a> GridCore<'a> {
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

    fn assert_index(ds: &DataSource, index: i32) {
        assert_lt!(
            index as usize,
            ds.data.len(),
            "buffer index {} out of bounds {}",
            index,
            ds.data.len()
        );
    }
}
