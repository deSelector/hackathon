#![allow(dead_code)]
use crate::grid::column::*;
use crate::grid::renderer::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Schema {
    pub cols: Vec<Column>,
    #[serde(default)]
    pub col_width: f64, // todo: switch to col-based widths
}

impl Schema {
    pub fn get_col_by_id(&self, id: u32) -> Option<&Column> {
        self.cols.iter().find(|o| o.id == id)
    }

    pub fn get_col_by_type(&self, col_type: ColumnType) -> Option<&Column> {
        self.cols.iter().find(|o| o.col_type == col_type)
    }

    pub fn get_visible_row_count(&self) -> usize {
        self.cols.iter().filter(|&o| !o.hidden).count()
    }

    pub fn get_visible_cols(&self) -> impl Iterator<Item = &Column> {
        self.cols.iter().filter(|&o| !o.hidden).into_iter()
    }
}

pub fn assert_schema(schema: &Schema) {
    for col in &schema.cols {
        assert!(col.id > 0);
        // ensure data size is defined for string columns
        assert!(col.col_type != ColumnType::String || col.data_len > 0);
        assert!(col.col_type == ColumnType::String || col.data_len == num_size())
    }
}
