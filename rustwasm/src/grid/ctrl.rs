use super::ds::*;
use super::renderer::*;
use super::schema::*;
use crate::utils::*;
use std::collections::HashMap;
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
    sparks: Sparks,
    top_index: usize,
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
        GridRenderer::new(&ctx(&self.id), &self.schema, left, top, width, height).render(
            &DataSource::new(data, data_width, Some(&self.sparks)),
            self.top_index,
        );
    }

    fn set_schema(obj: &JsValue) -> Schema {
        console_error_panic_hook::set_once();
        let mut schema = obj.into_serde::<Schema>().unwrap_or_default();
        normalize_schema(&mut schema);
        schema
    }

    pub fn set_sparks(&mut self, obj: &JsValue) {
        console_error_panic_hook::set_once();
        let temp = obj
            .into_serde::<HashMap<String, Vec<f64>>>()
            .unwrap_or_default();
        self.sparks = Sparks::new();
        for (key, val) in temp.iter() {
            self.sparks.insert(hash_code(key), val.to_vec());
        }
    }

    pub fn has_sparks(&self) -> bool {
        self.sparks.len() > 0
    }

    pub fn set_top_index(&mut self, top_index: usize) {
        self.top_index = top_index;
    }
}
