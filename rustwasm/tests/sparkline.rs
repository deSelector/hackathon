#![allow(dead_code)]

#[cfg(test)]
mod tests {
    //use super::*;
    use rustwasm::grid::sparkline::*;
    static DATA: &[f64] = &[100.0, 50.0, 75.0, 0.0, 25.0, 200.0];

    #[test]
    fn test_sparks_min() {
        let x = sparks_min(DATA);
        assert_eq!(x, 0.0);
    }

    #[test]
    fn test_sparks_max() {
        let x = sparks_max(DATA);
        assert_eq!(x, 200.0);
    }

    #[test]
    fn test_sparkline_dimensions() {
        let mut sp = Sparkline::new();
        let margin = 5.0;
        sp.init(10.0, 20.0, 300.0, 400.0, margin, DATA);
        assert_eq!(sp.x, 15.0);
        assert_eq!(sp.y, 25.0);
        assert_eq!(sp.width, 300.0 - 2.0 * margin);
        assert_eq!(sp.height, 400.0 - 2.0 * margin);
    }

    #[test]
    fn test_sparkline_empty_data() {
        let mut sp = Sparkline::new();
        let margin = 5.0;
        sp.init(10.0, 20.0, 300.0, 400.0, margin, &[]);
        assert_eq!(sp.min, 0.0);
        assert_eq!(sp.max, 0.0);
        assert_eq!(sp.range_y, 0.0);
        assert_eq!(sp.step_x, 0.0);
        assert_eq!(sp.x, 15.0);
        assert_eq!(sp.y, 20.0 + 200.0);
    }

    #[test]
    fn test_sparkline_with_data() {
        let mut sp = Sparkline::new();
        let margin = 5.0;
        sp.init(10.0, 20.0, 300.0, 400.0, margin, DATA);
        assert_eq!(DATA.len(), 6);
        assert_eq!(sp.range_y, 200.0);
        assert_eq!(sp.step_x, (300.0 - 2.0 * margin) / 5.0);
    }

    #[test]
    fn test_sparkline_y_values() {
        let mut sp = Sparkline::new();
        let margin = 0.0;
        sp.init(10.0, 20.0, 300.0, 400.0, margin, DATA);
        assert_eq!(DATA.len(), 6);
        let xxx: Vec<f64> = DATA.iter().map(|&x| sp.pos_y(x)).collect();
        assert_eq!(xxx, [220.0, 320.0, 270.0, 420.0, 370.0, 20.0]);
    }
}
