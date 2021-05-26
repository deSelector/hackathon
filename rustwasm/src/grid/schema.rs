#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use serde_repr::*;

#[derive(Debug, PartialEq, Serialize_repr, Deserialize_repr, Copy, Clone)]
#[repr(u32)]
pub enum ColumnType {
    Default = 0,
    String,
    Number,
    Date,
    DateTime,
    Timestamp,
}

impl Default for ColumnType {
    fn default() -> Self {
        ColumnType::Default
    }
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
    #[serde(default)]
    pub precision: u32,
    #[serde(default)]
    pub hidden: bool,
}
