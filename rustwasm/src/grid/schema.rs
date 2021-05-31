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
    pub fn get_col_by_id(&self, id: &str) -> Option<&Column> {
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

pub fn normalize_schema(schema: &mut Schema) {
    let mut offset = 0_usize;
    for col in &mut schema.cols {
        assert!(col.id != "");
        assert!(col.col_type == ColumnType::String || col.size == num_size());
        col.data_offset = offset;
        if col.col_type == ColumnType::String {
            offset += col.size;
        } else {
            offset += num_size();
        }
    }
}
