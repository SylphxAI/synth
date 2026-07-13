//! Pure minify savings arithmetic (parity with packages/synth-js-minify savings()).

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Savings {
    pub original_size: usize,
    pub minified_size: usize,
    pub saved_bytes: isize,
    pub saved_percent: f64,
}

#[must_use]
pub fn savings(original: &str, minified: &str) -> Savings {
    let original_size = original.len();
    let minified_size = minified.len();
    let saved_bytes = original_size as isize - minified_size as isize;
    let saved_percent = if original_size == 0 {
        0.0
    } else {
        (saved_bytes as f64 / original_size as f64) * 100.0
    };
    Savings { original_size, minified_size, saved_bytes, saved_percent }
}

#[must_use]
pub fn compression_ratio(original_len: usize, minified_len: usize) -> f64 {
    if original_len == 0 {
        return 0.0;
    }
    1.0 - (minified_len as f64 / original_len as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn savings_basic() {
        let s = savings("aaaa", "aa");
        assert_eq!(s.original_size, 4);
        assert_eq!(s.minified_size, 2);
        assert_eq!(s.saved_bytes, 2);
        assert!((s.saved_percent - 50.0).abs() < 1e-9);
        assert!((compression_ratio(4, 2) - 0.5).abs() < 1e-9);
    }

    #[test]
    fn empty_original() {
        let s = savings("", "");
        assert_eq!(s.saved_bytes, 0);
        assert_eq!(s.saved_percent, 0.0);
    }
}
