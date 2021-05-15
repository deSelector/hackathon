use crate::ctx2d::clip_begin;
use crate::ctx2d::clip_end;
use crate::ctx2d::fill_rect;
use crate::ctx2d::fill_text_aligned;
use crate::ctx2d::horizontal_line;
use crate::ctx2d::set_fill_style;
use crate::ctx2d::set_text_baseline;
use crate::ctx2d::vertical_line;
use js_sys::Date;
use web_sys::CanvasRenderingContext2d;

pub struct Grid<'a> {
    width: u32,
    height: u32,
    row_height: u32,
    col_count: u32,
    margin: u32,
    ctx: Option<&'a CanvasRenderingContext2d>,
    data_width: u32,
}

impl<'a> Grid<'a> {
    pub fn new(
        ctx: &'a CanvasRenderingContext2d,
        width: u32,
        height: u32,
        row_height: u32,
        col_count: u32,
        data_width: u32,
        margin: u32,
    ) -> Grid {
        Grid {
            ctx: Some(ctx),
            width,
            height,
            row_height,
            col_count,
            data_width,
            margin,
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

    pub fn draw_grid(&self) {
        let ctx = self.get_ctx();
        self.clear();

        ctx.begin_path();
        ctx.set_stroke_style(&"#232832".into());

        let col_width = self.cell_width();

        // Vertical lines.
        {
            for i in 0..self.col_count + 1 {
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
                let y = self.top() + (j * self.row_height) as f64;
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

    pub fn draw_highlight(&self, x: f64, y: f64, width: f64, time: f64) {
        let ctx = self.get_ctx();
        let now = Date::new_0().get_time() as i64;
        if now - time as i64 <= 250 {
            ctx.save();
            fill_rect(ctx, x, y, width, self.row_height.into(), "#ffffff22");
            ctx.restore();
        }
    }

    pub fn fill_text_aligned(&self, text: &str, x: f64, y: f64, width: f64, align: &str) {
        fill_text_aligned(
            self.get_ctx(),
            text,
            x,
            y,
            width,
            self.row_height.into(),
            align,
        );
    }
}

impl<'a> Grid<'a> {
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
}
