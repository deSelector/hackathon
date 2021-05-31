use super::ds::*;
use super::renderer::*;
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

    pub fn render(
        &self,
        data: &[SZ],
        data_width: usize,
        top: u32,
        left: u32,
        width: u32,
        height: u32,
    ) {
        GridRenderer::new(&ctx(&self.id), &self.schema, left, top, width, height)
            .render(&DataSource::new(data, data_width));
    }

    fn set_schema(obj: &JsValue) -> Schema {
        console_error_panic_hook::set_once();
        let mut schema = obj.into_serde::<Schema>().unwrap_or_default();
        _console_log!("SCHEMA: {:?}, el={:?}", obj, schema);
        normalize_schema(&mut schema);
        schema
    }
}
