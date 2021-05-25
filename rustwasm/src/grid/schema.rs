#![allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Schema {
    pub cols: Vec<Column>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Column {
    pub id: u32,
    pub name: String,
}
