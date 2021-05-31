use crate::grid::core::*;

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
}
