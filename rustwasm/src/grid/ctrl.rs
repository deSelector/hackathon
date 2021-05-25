use crate::grid::core::*;
use crate::grid::schema::*;
use crate::utils::*;
use chrono::prelude::*;
use chrono::Local;
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

#[wasm_bindgen]
#[derive(Default)]
pub struct Grid {
    id: String,
    schema: Schema,
    col_count: u32,
    pub left: u32,
    pub top: u32,
    pub width: u32,
    pub height: u32,
    pub data_width: u32,
}

#[wasm_bindgen]
impl Grid {
    pub fn new(id: String, width: u32, height: u32) -> Grid {
        set_panic_hook();
        Grid {
            id,
            width,
            height,
            data_width: 1,
            ..Default::default()
        }
    }

    pub fn render(&self, data: &[f64]) {
        let ctx = &ctx(&self.id);
        let mut grid = GridCore::new(
            ctx,
            self.left,
            self.top,
            self.width,
            self.height,
            self.data_width,
        );
        grid.col_count = self.col_count;
        grid.row_count = grid.calc_row_count(data);
        grid.draw_gridlines();

        grid.clip_begin();
        self.render_data(&grid, data);
        self.render_header(&grid);
        grid.clip_end();
    }

    pub fn set_schema(&mut self, obj: &JsValue) {
        console_error_panic_hook::set_once();
        self.schema = obj.into_serde::<Schema>().unwrap_or_default();
        _console_log!("SCHEMA: {:?}, el={:?}", obj, self.schema);
        self.col_count = self.schema.cols.len() as u32;
    }
}

impl Grid {
    fn render_data(&self, grid: &GridCore, data: &[f64]) {
        let row_count = (data.len() / self.data_width as usize) as u32;
        let col_width = grid.cell_width();
        let ts_offset = self.ts_col_offset();

        for r in 0.. {
            let y = grid.top() + ((r + HEADER_LINES) * grid.row_height) as f64;

            if y < grid.bottom() as f64 && r < row_count {
                let highlight = if ts_offset.is_some() {
                    grid.is_highlight(
                        grid.cell_value(data, r as i32, ts_offset.unwrap())
                            .unwrap_or_default(),
                    )
                } else {
                    false
                };

                for i in 0..self.schema.cols.len() {
                    let c = &self.schema.cols[i];
                    let x = grid.left() + grid.cell_x(i);
                    let v = grid
                        .cell_value(data, r as i32, c.data_offset)
                        .unwrap_or_default();

                    grid.fill_text_aligned(
                        &self.format_value(v, c),
                        x,
                        y,
                        col_width,
                        "right",
                        highlight,
                    );
                }
            } else {
                break;
            }
        }
    }

    fn render_header(&self, grid: &GridCore) {
        let col_width = grid.cell_width();
        for i in 0..self.schema.cols.len() {
            let x = grid.left() + grid.cell_x(i);
            grid.fill_text_aligned(
                self.schema.cols[i].name.as_str(),
                x,
                0_f64,
                col_width,
                "center",
                false,
            );
        }
    }
}

impl Grid {
    fn format_value(&self, value: f64, col: &Column) -> String {
        match col.col_type {
            ColumnType::DateTime | ColumnType::Timestamp | ColumnType::Date => Local
                .timestamp(value as i64 / 1000, 0)
                .format("%r")
                .to_string(),
            ColumnType::Number => {
                format_args!("{:.*}", self.cell_precision(col), value).to_string()
            }
            _ => value.to_string(),
        }
    }

    fn cell_precision(&self, col: &Column) -> usize {
        match col.col_type {
            ColumnType::Number => col.precision as usize,
            _ => 0,
        }
    }

    fn ts_col_offset(&self) -> Option<u32> {
        let col = self
            .schema
            .cols
            .iter()
            .find(|o| o.col_type == ColumnType::Timestamp);
        if col.is_some() {
            return Some(col.unwrap().data_offset);
        }
        None
    }
}
