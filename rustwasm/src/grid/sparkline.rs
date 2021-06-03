#![allow(dead_code)]

use crate::utils::set_panic_hook;
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

#[derive(PartialEq, Copy, Clone)]
#[repr(usize)]
enum Color {
    Red = 0,
    Green = 1,
    Gray = 2,
}

impl Default for Color {
    fn default() -> Self {
        Color::Gray
    }
}

const MARGIN: f64 = 5_f64;
static COLORS: [&str; 3] = ["#ff0000", "#008000", "#232832"];

#[derive(Default)]
pub struct Sparkline {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub margin: f64,
    pub min: f64,
    pub max: f64,
    pub range_y: f64,
    pub step_x: f64,
    last_c: Color,
}

impl Sparkline {
    pub fn new() -> Sparkline {
        set_panic_hook();
        Sparkline {
            ..Default::default()
        }
    }

    pub fn render(
        &mut self,
        ctx: &CanvasRenderingContext2d,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        sparks: &[f64],
    ) {
        if sparks.len() <= 1 {
            return;
        }

        self.init(x, y, width, height, MARGIN, sparks);

        if self.range_y == 0.0 {
            self.stroke(
                ctx,
                self.x,
                self.y,
                self.x + self.width,
                self.y,
                Color::Gray,
            );
            return;
        }

        let v_0 = sparks[0];
        let y_0 = self.pos_y(v_0);
        let mut last_y = y_0;
        let mut last_x = self.x;
        let mut next_x = self.x;

        for i in 1..sparks.len() {
            let v = sparks[i];
            let next_y = self.pos_y(v);
            let next_c = color_index(v_0, v);
            next_x += self.step_x;
            // split line to transition from below the y0 to above or vice-versa
            if next_y != last_y && next_c != self.last_c {
                let x_0 = last_x - ((last_x - next_x) * (y_0 - last_y)) / (next_y - last_y);
                self.stroke(ctx, last_x, last_y, x_0, y_0, self.last_c);
                self.stroke(ctx, x_0, y_0, next_x, next_y, next_c);
            } else {
                self.stroke(ctx, last_x, last_y, next_x, next_y, next_c);
            }

            last_x = next_x;
            last_y = next_y;
        }
    }

    pub fn init(&mut self, x: f64, y: f64, width: f64, height: f64, margin: f64, sparks: &[f64]) {
        self.margin = margin;
        self.width = width - 2.0 * margin;
        self.height = height - 2.0 * margin;
        self.x = x + margin;
        self.y = y + margin;

        if sparks.len() > 1 {
            self.min = sparks_min(sparks);
            self.max = sparks_max(sparks);
            self.range_y = self.max - self.min;
            self.step_x = self.width / (sparks.len() as f64 - 1.0);
        }
        // single line in the middle
        if self.range_y == 0.0 {
            self.y = y + height / 2.0;
        }
    }

    pub fn pos_y(&self, v: f64) -> f64 {
        self.y + self.height - ((v - self.min) / self.range_y) * self.height
    }

    fn stroke(
        &mut self,
        ctx: &CanvasRenderingContext2d,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        color_index: Color,
    ) {
        ctx.begin_path();
        if self.last_c != color_index {
            ctx.set_stroke_style(&COLORS[color_index as usize].into());
            self.last_c = color_index;
        }

        ctx.move_to(x1, y1);
        ctx.line_to(x2, y2);
        ctx.stroke();
    }
}

fn color_index(v0: f64, v: f64) -> Color {
    if v < v0 {
        Color::Red
    } else if v > v0 {
        Color::Green
    } else {
        Color::Gray
    }
}

pub fn sparks_min(data: &[f64]) -> f64 {
    *data
        .iter()
        .reduce(|a, b| if a <= b { a } else { b })
        .unwrap()
}
pub fn sparks_max(data: &[f64]) -> f64 {
    *data
        .iter()
        .reduce(|a, b| if a >= b { a } else { b })
        .unwrap()
}
