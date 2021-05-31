#![allow(dead_code)]
use crate::grid::core::*;
use chrono::prelude::*;
use chrono::Local;
use serde::{Deserialize, Serialize};
use serde_repr::*;

#[derive(Debug, PartialEq, Serialize_repr, Deserialize_repr, Copy, Clone)]
#[repr(u32)]
pub enum ColumnType {
    None = 0,
    String,
    Number,
    Date,
    DateTime,
    Timestamp,
}

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

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Column {
    pub id: u32,
    pub name: String,
    pub col_type: ColumnType,
    pub data_offset: usize,
    #[serde(default = "num_size")]
    pub data_len: usize,
    #[serde(default)]
    pub precision: usize,
    #[serde(default)]
    pub hidden: bool,
}

impl Default for ColumnType {
    fn default() -> Self {
        ColumnType::Number
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

pub fn cell_precision(col: &Column) -> usize {
    match col.col_type {
        ColumnType::Number => col.precision,
        _ => 0,
    }
}

pub fn format_value(value: f64, col: &Column) -> String {
    match col.col_type {
        ColumnType::DateTime | ColumnType::Timestamp | ColumnType::Date => Local
            .timestamp(value as i64 / 1000, 0)
            .format("%r")
            .to_string(),
        ColumnType::Number => format_args!("{:.*}", cell_precision(col), value).to_string(),
        _ => value.to_string(),
    }
}
