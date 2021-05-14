use crate::ctx2d::fill_rect;
use js_sys::Date;
use web_sys::CanvasRenderingContext2d;

pub struct Grid {}

impl Grid {
    pub fn draw_highlight(
        ctx: &CanvasRenderingContext2d,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        time: f64,
    ) {
        let now = Date::new_0().get_time() as i64;
        if now - time as i64 <= 250 {
            ctx.save();
            fill_rect(ctx, x, y, width, height, "#ffff0088");
            ctx.restore();
        }
    }
}
