#![allow(dead_code)]

use super::ctx2d::*;
use js_sys::Date;
use web_sys::CanvasRenderingContext2d;

const HIGHLIGHT_DURATION: i64 = 100;
const ROW_HEIGHT: u32 = 30;
const MARGIN: u32 = 0;

#[derive(Default)]
pub struct GridCore<'a> {
    ctx: Option<&'a CanvasRenderingContext2d>,
    width: u32,
    height: u32,
    pub row_height: u32,
    pub col_count: u32,
    pub row_count: u32,
    pub margin: u32,
    data_width: u32,
}

impl<'a> GridCore<'a> {
    pub fn new(
        ctx: &'a CanvasRenderingContext2d,
        width: u32,
        height: u32,
        data_width: u32,
    ) -> GridCore {
        GridCore {
            ctx: Some(ctx),
            width,
            height,
            row_height: ROW_HEIGHT,
            data_width,
            margin: MARGIN,
            ..Default::default()
        }
    }

    pub fn cell_value(&self, data: &[f64], row: i32, col: u32) -> Option<f64> {
        match row {
            row if row >= 0 => {
                let index = row * self.data_width as i32 + col as i32;
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

    pub fn clear(&self) {
        let ctx = self.get_ctx();
        fill_rect(
            ctx,
            0.0,
            0.0,
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

    pub fn draw_gridlines(&self) {
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
            if y < self.bottom() && j <= self.row_count {
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
        highlight: Option<bool>,
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
        self.margin as f64 + 0.5
    }
    pub fn top(&self) -> f64 {
        self.margin as f64 + 0.5
    }
    pub fn right(&self) -> f64 {
        (self.width - self.margin) as f64 - 0.5
    }
    pub fn bottom(&self) -> f64 {
        (self.height - self.margin) as f64 - 0.5
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

    pub fn assert_data_source(&self, data: &[f64]) {
        assert_eq!(
            data.len() as f64 % self.data_width as f64,
            0.0,
            "buffer size {} not divisible by {}",
            data.len(),
            self.data_width
        );
    }

    pub fn calc_row_count(&self, data: &[f64]) -> u32 {
        data.len() as u32 / self.data_width
    }
}
