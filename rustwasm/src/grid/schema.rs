#![allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Schema {
    pub cols: Vec<Column>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Column {
    pub id: u32,
    pub name: String,
}
