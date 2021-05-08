use crate::ctx2d::*;
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

const COL_COUNT: u32 = 10;
const ROW_HEIGHT: u32 = 26;
const MARGIN: u32 = 20;

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
        self.client_width() / COL_COUNT as f64
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
            for i in 0..COL_COUNT + 1 {
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

    pub fn paint(&self) {
        let ctx = ctx(&self.id);
        let col_width = self.cell_width();
        self.draw_grid();

        // red: #ff3b69
        set_fill_style(&ctx, "#03c67a");
        set_text_align(&ctx, "right");
        set_text_baseline(&ctx, "middle");

        let values = &get_random_255().unwrap();

        clip_begin(
            &ctx,
            self.left(),
            self.top(),
            self.client_width(),
            self.client_height(),
        );

        let mut i = 0;
        for r in 0.. {
            let y = self.left() + (r * ROW_HEIGHT) as f64;
            if y < self.bottom() as f64 {
                for c in 0..COL_COUNT {
                    let x = self.left() + (c as f64 * col_width).floor();
                    let r = values[(i % 255) as usize];
                    fill_text_aligned(&ctx, &r.to_string(), x, y, col_width, ROW_HEIGHT as f64);
                    i += 1;
                }
            } else {
                break;
            }
        }
        clip_end(&ctx);
    }
}
