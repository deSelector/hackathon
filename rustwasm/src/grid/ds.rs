use crate::grid::core::*;
use crate::grid::schema::Column;
use byteorder::{BigEndian, ByteOrder};

#[derive(Default)]
pub struct DataSource<'a> {
    pub data: &'a [SZ],
    pub data_width: usize,
    pub row_count: usize,
}

impl<'a> DataSource<'a> {
    pub fn new(data: &'a [SZ], data_width: usize) -> DataSource {
        assert!(data_width > 0);
        assert_eq!(
            data.len() as f64 % data_width as f64,
            0.0,
            "buffer size {} not divisible by {}",
            data.len(),
            data_width
        );

        DataSource {
            data,
            data_width,
            row_count: (data.len() / data_width),
            ..Default::default()
        }
    }

    pub fn get_value_f64(&self, row: usize, col: &Column) -> Option<f64> {
        match row {
            row if row < self.row_count && self.data.len() > 0 => {
                let index = self.get_cell_index(row, col);
                // note: potential performance impact - verify.
                let v = BigEndian::read_f64(&self.data[index..index + num_size()]);
                Some(v)
            }
            _ => None,
        }
    }

    pub fn get_value_str(&self, row: usize, col: &Column) -> Option<String> {
        match row {
            row if row < self.row_count => {
                let index = self.get_cell_index(row, col);
                let str_slice = &self.data[index..index + col.data_len];
                let s = String::from_utf8_lossy(str_slice);
                return Some(s.to_string());
            }
            _ => None,
        }
    }

    pub fn get_cell_index(&self, row: usize, col: &Column) -> usize {
        let index = row * self.data_width + col.data_offset;
        assert_lt!(
            index,
            self.data.len(),
            "buffer index {} out of bounds {}",
            index,
            self.data.len()
        );
        index
    }
}
