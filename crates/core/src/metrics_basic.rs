//! Pure basic source metrics (LOC/blank/comment-ish) without full AST analysis.
//! Residual pure deepen for tooling/metrics arithmetic fragment (FLEET-SITES-WAVE2).

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineKind {
    Blank,
    Comment,
    Code,
}

#[must_use]
pub fn classify_line(trimmed: &str) -> LineKind {
    if trimmed.is_empty() {
        return LineKind::Blank;
    }
    // Parity with TS isCommentLine (+ `*/` continuation already kept; HTML `<!--` added).
    if trimmed.starts_with("//")
        || trimmed.starts_with('#')
        || trimmed.starts_with("/*")
        || trimmed.starts_with('*')
        || trimmed.starts_with("*/")
        || trimmed.starts_with("<!--")
    {
        return LineKind::Comment;
    }
    LineKind::Code
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BasicLocMetrics {
    pub loc: usize,
    pub blank: usize,
    pub cloc: usize,
    pub sloc: usize,
}

#[must_use]
pub fn analyze_basic_loc(source: &str) -> BasicLocMetrics {
    let lines: Vec<&str> = if source.is_empty() {
        Vec::new()
    } else {
        source.split('\n').collect()
    };
    let loc = if source.is_empty() { 0 } else { lines.len() };
    let mut blank = 0usize;
    let mut cloc = 0usize;
    for line in &lines {
        match classify_line(line.trim()) {
            LineKind::Blank => blank += 1,
            LineKind::Comment => cloc += 1,
            LineKind::Code => {}
        }
    }
    let sloc = loc.saturating_sub(blank).saturating_sub(cloc);
    BasicLocMetrics { loc, blank, cloc, sloc }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loc_blank_comment_code() {
        let src = "fn main() {}\n\n// comment\nlet x = 1;\n";
        let m = analyze_basic_loc(src);
        assert!(m.loc >= 4);
        assert!(m.blank >= 1);
        assert!(m.cloc >= 1);
        assert!(m.sloc >= 2);
    }

    #[test]
    fn empty_source() {
        let m = analyze_basic_loc("");
        assert_eq!(m, BasicLocMetrics { loc: 0, blank: 0, cloc: 0, sloc: 0 });
    }

    #[test]
    fn fleet_web_media_wave4_classify_line_kinds() {
        // classify_line expects already-trimmed input (see analyze_basic_loc).
        assert_eq!(classify_line(""), LineKind::Blank);
        assert_eq!(classify_line("// comment"), LineKind::Comment);
        assert_eq!(classify_line("# python"), LineKind::Comment);
        assert_eq!(classify_line("/* block"), LineKind::Comment);
        assert_eq!(classify_line("* cont"), LineKind::Comment);
        assert_eq!(classify_line("<!-- html"), LineKind::Comment);
        assert_eq!(classify_line("let x = 1;"), LineKind::Code);
        // untrimmed spaces are NOT blank at this layer
        assert_eq!(classify_line("   "), LineKind::Code);
    }

    #[test]
    fn fleet_web_media_wave4_analyze_mixed_source() {
        let src = "\n// c\nfn main() {}\n\n";
        let m = analyze_basic_loc(src);
        assert!(m.loc >= 4);
        assert!(m.blank >= 1);
        assert!(m.cloc >= 1);
        assert!(m.sloc >= 1);
        assert_eq!(m.loc, m.blank + m.cloc + m.sloc);
    }
}
