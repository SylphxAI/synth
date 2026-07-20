//! Pure language detection from URI extension — mirrors
//! `packages/synth/src/incremental-parser-manager.ts#detectLanguage`.
//! PORTFOLIO-PRODUCTS-WAVE7 pure residual. NO authority_rust / ts_deleted.

/// Detect language id from a file URI / path extension.
#[must_use]
pub fn detect_language(uri: &str) -> Option<&'static str> {
    let ext = uri
        .rsplit('.')
        .next()
        .map(|e| e.to_ascii_lowercase())
        .unwrap_or_default();
    // no extension (no dot) → None; treat whole string as "ext" only if had a dot
    if !uri.contains('.') {
        return None;
    }
    match ext.as_str() {
        "md" | "markdown" => Some("markdown"),
        "js" | "mjs" => Some("javascript"),
        "ts" | "mts" => Some("typescript"),
        "html" | "htm" => Some("html"),
        "json" => Some("json"),
        "css" => Some("css"),
        "yaml" | "yml" => Some("yaml"),
        "toml" => Some("toml"),
        "xml" => Some("xml"),
        _ => None,
    }
}

/// Common prefix length between two texts (detectEdit helper pure fragment).
#[must_use]
pub fn common_prefix_len(old: &str, new: &str) -> usize {
    old.bytes()
        .zip(new.bytes())
        .take_while(|(a, b)| a == b)
        .count()
}

/// Common suffix length after a known prefix start (detectEdit pure fragment).
#[must_use]
pub fn common_suffix_len(old: &str, new: &str, start_index: usize) -> usize {
    let old_b = old.as_bytes();
    let new_b = new.as_bytes();
    let mut old_end = old_b.len();
    let mut new_end = new_b.len();
    let mut n = 0usize;
    while old_end > start_index && new_end > start_index {
        if old_b[old_end - 1] != new_b[new_end - 1] {
            break;
        }
        old_end -= 1;
        new_end -= 1;
        n += 1;
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn language_from_ext() {
        assert_eq!(detect_language("a/b/c.md"), Some("markdown"));
        assert_eq!(detect_language("x.MARKDOWN"), Some("markdown"));
        assert_eq!(detect_language("app.mjs"), Some("javascript"));
        assert_eq!(detect_language("app.ts"), Some("typescript"));
        assert_eq!(detect_language("style.css"), Some("css"));
        assert_eq!(detect_language("cfg.yaml"), Some("yaml"));
        assert_eq!(detect_language("cfg.yml"), Some("yaml"));
        assert_eq!(detect_language("noext"), None);
        assert_eq!(detect_language("file.unknown"), None);
    }

    #[test]
    fn prefix_suffix() {
        assert_eq!(common_prefix_len("hello world", "hello there"), 6);
        assert_eq!(common_suffix_len("abXYZ", "cdXYZ", 0), 3);
        assert_eq!(common_suffix_len("abc", "abc", 0), 3);
        // when start cuts into suffix
        assert_eq!(common_suffix_len("aaabbb", "aaaxbb", 3), 2);
    }
}
