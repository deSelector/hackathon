#![allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Schema {
    pub cols: Vec<Column>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Column {
    pub id: u32,
    pub name: String,
}
