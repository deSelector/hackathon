use super::core::*;
use super::ds::*;
use super::schema::*;
use crate::utils::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! _console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
#[derive(Default)]
pub struct Grid {
    id: String,
    schema: Schema,
    pub left: u32,
    pub top: u32,
}

#[wasm_bindgen]
impl Grid {
    pub fn new(id: String, schema: &JsValue) -> Grid {
        set_panic_hook();
        Grid {
            id,
            schema: Grid::set_schema(schema),
            ..Default::default()
        }
    }

    pub fn render(&self, data: &[SZ], data_width: usize, width: u32, height: u32) {
        let ctx = &ctx(&self.id);
        let ds = &DataSource::new(data, data_width);

        let mut grid = GridCore::new(ctx, &self.schema, self.left, self.top, width, height);
        grid.calc_col_width();
        grid.draw_gridlines(ds);

        grid.clip_begin();
        grid.render_data(ds);
        grid.render_header();
        grid.clip_end();
    }

    fn set_schema(obj: &JsValue) -> Schema {
        console_error_panic_hook::set_once();
        let schema = obj.into_serde::<Schema>().unwrap_or_default();
        _console_log!("SCHEMA: {:?}, el={:?}", obj, schema);
        assert_schema(&schema);
        schema
    }
}
