use crate::ctx2d::*;
use crate::utils::*;
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

const COL_SIZE_COUNT: u32 = 2;
const ROW_HEIGHT: u32 = 30;
const MARGIN: u32 = 20;

#[derive(PartialEq, Copy, Clone)]
enum Side {
    Bid = 0,
    Ask = 1,
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
    fn cell_width(&self) -> f64 {
        self.client_width() / (COL_SIZE_COUNT as f64 * 2.0)
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
}

impl DOB {
    fn draw_grid(&self) {
        let ctx = ctx(&self.id);

        clear_rect(
            &ctx,
            0.0,
            0.0,
            self.width as f64,
            self.height as f64,
            &"#0b0e17",
        );
        clear_rect(
            &ctx,
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
            for i in 0..(COL_SIZE_COUNT * 2) + 1 {
                let x = self.left() + (i as f64 * col_width).floor();
                ctx.move_to(x, self.top());
                ctx.line_to(x, self.bottom());
                vertical_line(&ctx, self.top(), self.bottom(), x);
            }
        }

        {
            // Horizontal lines.
            let mut j = 0;
            loop {
                let y = self.top() + (j * ROW_HEIGHT) as f64;
                if y < self.bottom() {
                    horizontal_line(&ctx, self.left(), self.right(), y);
                    j += 1;
                } else {
                    break;
                }
            }
            horizontal_line(&ctx, self.left(), self.right(), self.bottom());
        }

        ctx.stroke();
    }

    pub fn paint(&self, bids: &[f64], asks: &[f64]) {
        let ctx = ctx(&self.id);

        self.draw_grid();

        // red: #ff3b69
        set_fill_style(&ctx, "#03c67a");
        set_text_baseline(&ctx, "middle");

        clip_begin(
            &ctx,
            self.left(),
            self.top(),
            self.client_width(),
            self.client_height(),
        );

        self.paint_side(&ctx, bids, Side::Bid);
        self.paint_side(&ctx, asks, Side::Ask);

        clip_end(&ctx);
    }

    fn paint_side(&self, ctx: &CanvasRenderingContext2d, data: &[f64], side: Side) {
        let row_count = (data.len() / COL_SIZE_COUNT as usize) as u32;
        let col_width = self.cell_width();
        let dx = self.start_x(side);
        let align = self.align(side);

        assert_eq!(
            data.len() as f64 % COL_SIZE_COUNT as f64,
            0.0,
            "buffer size {} not divisible by {}",
            data.len(),
            COL_SIZE_COUNT
        );

        for r in 0.. {
            let y = self.top() + (r * ROW_HEIGHT) as f64;
            if y < self.bottom() as f64 && r < row_count {
                for c in 0..COL_SIZE_COUNT {
                    let x = dx + (c as f64 * col_width).floor();
                    let v = self.value(data, r, c, side);
                    fill_text_aligned(
                        &ctx,
                        &format_args!("{0:.3}", v).to_string(),
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

    fn value(&self, data: &[f64], row: u32, col: u32, side: Side) -> f64 {
        let index = row * COL_SIZE_COUNT
            + if side == Side::Bid {
                COL_SIZE_COUNT - col - 1
            } else {
                col
            };

        assert_lt!(
            index as usize,
            data.len(),
            "buffer index {} out of bounds {}",
            index,
            data.len()
        );
        data[index as usize]
    }

    fn start_x(&self, side: Side) -> f64 {
        self.left()
            + if side == Side::Bid {
                0.0
            } else {
                self.cell_width() * COL_SIZE_COUNT as f64
            }
    }

    fn align(&self, side: Side) -> &str {
        if side == Side::Bid {
            "right"
        } else {
            "left"
        }
    }
}
