#![allow(dead_code)]
use crate::grid::core::*;
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
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Column {
    pub id: u32,
    pub name: String,
    pub col_type: ColumnType,
    pub data_offset: u32,
    #[serde(default = "num_size")]
    pub data_len: usize,
    #[serde(default)]
    pub precision: u32,
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
