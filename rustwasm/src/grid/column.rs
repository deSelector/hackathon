#![allow(dead_code)]
use crate::grid::renderer::*;
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
    Sparkline,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Column {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub col_type: ColumnType,
    #[serde(default)]
    pub data_offset: usize,
    #[serde(default = "num_size")]
    pub size: usize,
    #[serde(default)]
    pub precision: usize,
    #[serde(default)]
    pub hidden: bool,
    #[serde(default)]
    pub align: String, // "left", "center", "right"
    #[serde(default)]
    pub highlight: bool,
    #[serde(default)]
    pub suppress_zero: bool, // switch to format later
}

impl Default for ColumnType {
    fn default() -> Self {
        ColumnType::Number
    }
}

impl Column {
    pub fn precision(&self) -> usize {
        match self.col_type {
            ColumnType::Number => self.precision,
            _ => 0,
        }
    }

    pub fn format_value(&self, value: f64) -> String {
        match self.col_type {
            ColumnType::DateTime | ColumnType::Timestamp | ColumnType::Date => Local
                .timestamp(value as i64 / 1000, 0)
                .format("%r")
                .to_string(),
            ColumnType::Number => {
                if value == 0.0 && self.suppress_zero {
                    String::from("")
                } else {
                    format_args!("{:.*}", self.precision(), value).to_string()
                }
            }
            ColumnType::Sparkline => String::from(""),
            _ => value.to_string(),
        }
    }
}
