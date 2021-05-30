use crate::grid::core::*;

#[derive(Default)]
pub struct DataSource<'a> {
    pub data: &'a [SZ],
    pub data_width: u32,
    pub row_count: u32,
}

impl<'a> DataSource<'a> {
    pub fn new(data: &'a [SZ], data_width: u32) -> DataSource {
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
            row_count: (data.len() / data_width as usize) as u32,
            ..Default::default()
        }
    }
}
