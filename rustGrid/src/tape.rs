use crate::ctx2d::*;
use crate::utils::*;
use chrono::NaiveDateTime;
use enum_iterator::IntoEnumIterator;
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

const DATA_WIDTH: u32 = 3; // price, size, time
const COL_COUNT: u32 = 3;
const ROW_HEIGHT: u32 = 30;
const MARGIN: u32 = 20;

#[derive(PartialEq, Copy, Clone, IntoEnumIterator)]
pub enum Field {
    Price = 0,
    Size = 1,
    Time = 2,
}

#[wasm_bindgen]
pub struct Tape {
    id: String,
    width: u32,
    height: u32,
}

#[wasm_bindgen]
impl Tape {
    pub fn new(id: String, width: u32, height: u32) -> Tape {
        Tape { id, width, height }
    }

    pub fn get_data_width() -> u32 {
        DATA_WIDTH
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
}

impl Tape {
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

impl Tape {
    fn draw_grid(&self) {
        let ctx = ctx(&self.id);

        fill_rect(
            &ctx,
            0.0,
            0.0,
            self.width as f64,
            self.height as f64,
            &"#0b0e17",
        );

        fill_rect(
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

    pub fn paint(&self, trades: &[f64]) {
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

        self.draw_tape(&ctx, trades);

        clip_end(&ctx);
    }

    fn draw_tape(&self, ctx: &CanvasRenderingContext2d, data: &[f64]) {
        let row_count = (data.len() / DATA_WIDTH as usize) as u32;
        let col_width = self.cell_width();

        assert_eq!(
            data.len() as f64 % DATA_WIDTH as f64,
            0.0,
            "buffer size {} not divisible by {}",
            data.len(),
            DATA_WIDTH
        );

        for r in 0.. {
            let y = self.top() + (r * ROW_HEIGHT) as f64;
            if y < self.bottom() as f64 && r < row_count {
                for &field in [Field::Price, Field::Size, Field::Time].iter() {
                    let x = self.left() + self.cell_x(field);
                    let v = self.cell_value(data, r as i32, field).unwrap_or_default();
                    fill_text_aligned(
                        &ctx,
                        &self.format_value(v, field),
                        x,
                        y,
                        col_width,
                        ROW_HEIGHT as f64,
                        "right",
                    );
                }
            } else {
                break;
            }
        }
    }
}

impl Tape {
    fn format_value(&self, value: f64, field: Field) -> String {
        match field {
            Field::Time => NaiveDateTime::from_timestamp(value as i64 / 1000, 0)
                .format("%r")
                .to_string(),
            _ => format_args!("{:.*}", self.cell_precision(field), value).to_string(),
        }
    }

    fn cell_value(&self, data: &[f64], row: i32, field: Field) -> Option<f64> {
        match row {
            row if row >= 0 => {
                let index = row * DATA_WIDTH as i32 + field as i32;
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

    fn cell_x(&self, field: Field) -> f64 {
        match field {
            Field::Price => 0.0,
            Field::Size => self.cell_width(),
            Field::Time => self.cell_width() * 2.0,
        }
    }

    fn cell_precision(&self, field: Field) -> usize {
        match field {
            Field::Price => 3,
            Field::Size => 5,
            _ => 0,
        }
    }
}
