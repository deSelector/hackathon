use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! _console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

pub fn _fill_text(ctx: &CanvasRenderingContext2d, v: &str, x: f64, y: f64) {
    ctx.fill_text(v, x, y).unwrap();
}

pub fn fill_text_aligned(
    ctx: &CanvasRenderingContext2d,
    v: &str,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    text_align: &str,
) {
    const PADDING_H: f64 = 10.0;
    let dy = y + height / 2.0;
    set_text_align(ctx, text_align);
    match text_align {
        "right" => ctx.fill_text(v, x + width - PADDING_H, dy).unwrap(),
        "center" => ctx.fill_text(v, x + width / 2.0, dy).unwrap(),
        _ => ctx.fill_text(v, x + PADDING_H, dy).unwrap(),
    };
}

pub fn horizontal_line(ctx: &CanvasRenderingContext2d, left: f64, right: f64, y: f64) {
    ctx.move_to(left, y);
    ctx.line_to(right, y);
}

pub fn vertical_line(ctx: &CanvasRenderingContext2d, top: f64, bottom: f64, x: f64) {
    ctx.move_to(x, top);
    ctx.line_to(x, bottom);
}

pub fn _set_stroke(ctx: &CanvasRenderingContext2d, color: &str) {
    ctx.set_stroke_style(&color.into());
}

pub fn set_fill_style(ctx: &CanvasRenderingContext2d, color: &str) {
    ctx.set_fill_style(&color.into());
}

pub fn clear_rect(
    ctx: &CanvasRenderingContext2d,
    left: f64,
    top: f64,
    width: f64,
    height: f64,
    color: &str,
) {
    set_fill_style(ctx, &color);
    //ctx.clear_rect(left, top, width, height);
    ctx.fill_rect(left, top, width, height);
}

pub fn clip_begin(ctx: &CanvasRenderingContext2d, left: f64, top: f64, width: f64, height: f64) {
    ctx.save();
    ctx.begin_path();
    ctx.rect(left, top, width, height);
    ctx.clip();
}

pub fn clip_end(ctx: &CanvasRenderingContext2d) {
    ctx.restore();
}

pub fn set_text_align(ctx: &CanvasRenderingContext2d, align: &str) {
    ctx.set_text_align(&align);
}

pub fn set_text_baseline(ctx: &CanvasRenderingContext2d, align: &str) {
    ctx.set_text_baseline(&align);
}

pub fn _set_font(ctx: &CanvasRenderingContext2d, font: &str) {
    ctx.set_font(font);
}

pub fn _measure_text(ctx: &CanvasRenderingContext2d, text: &str) -> (f64, f64) {
    let metrics: ExtendedTextMetrics = ctx.measure_text(&text).unwrap().unchecked_into();
    let width = metrics.width();
    let height = metrics.actual_bounding_box_ascent() - metrics.actual_bounding_box_descent();
    (width, height)
}

#[wasm_bindgen]
extern "C" {
    type ExtendedTextMetrics;

    #[wasm_bindgen(method, getter, js_name = actualBoundingBoxAscent)]
    fn actual_bounding_box_ascent(this: &ExtendedTextMetrics) -> f64;

    #[wasm_bindgen(method, getter, js_name = actualBoundingBoxDescent)]
    fn actual_bounding_box_descent(this: &ExtendedTextMetrics) -> f64;

    #[wasm_bindgen(method, getter, js_name = actualBoundingBoxLeft)]
    fn actual_bounding_box_left(this: &ExtendedTextMetrics) -> f64;

    #[wasm_bindgen(method, getter, js_name = actualBoundingBoxRight)]
    fn actual_bounding_box_right(this: &ExtendedTextMetrics) -> f64;

    #[wasm_bindgen(method, getter)]
    fn width(this: &ExtendedTextMetrics) -> f64;
}
