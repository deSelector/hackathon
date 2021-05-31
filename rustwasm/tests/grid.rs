#![allow(dead_code)]

#[cfg(test)]
mod tests {
    use rustwasm::grid::renderer::*;
    //use super::*;
    use byteorder::{BigEndian, ByteOrder};

    #[test]
    fn test_u8_slice_to_f64() {
        let buf = [64, 143, 64, 0, 0, 0, 0, 0];
        let x = BigEndian::read_f64(&buf);
        assert_eq!(x, 1000.0);
    }

    #[test]
    fn test_f64_to_u8_slice() {
        let mut buf = [0u8; num_size() as usize];
        BigEndian::write_f64(&mut buf, 1000.0);
        assert_eq!(buf, [64, 143, 64, 0, 0, 0, 0, 0]);
    }
}
