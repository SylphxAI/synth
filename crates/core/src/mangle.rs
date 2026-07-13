//! Pure identifier mangling (parity with synth-js-minify Compressor.generateMangledName /
//! mangleName arithmetic). Residual pure deepen for tooling/format-minify-lint fragment.
//! No AST transform engine, no authority claims.

/// Generate short mangled names: a, b, ..., z, aa, ab, ...
/// Parity: `generateMangledName` with counter starting at 0.
#[must_use]
pub fn generate_mangled_name(counter: usize) -> String {
    let alphabet = b"abcdefghijklmnopqrstuvwxyz";
    let mut num = counter as isize;
    let mut name = String::new();
    loop {
        let idx = (num % 26) as usize;
        name.insert(0, alphabet[idx] as char);
        num = num / 26 - 1;
        if num < 0 {
            break;
        }
    }
    name
}

/// Stateful mangler mapping original → short names, respecting reserved set.
#[derive(Debug, Default, Clone)]
pub struct NameMangler {
    counter: usize,
    map: std::collections::HashMap<String, String>,
    reserved: std::collections::HashSet<String>,
}

impl NameMangler {
    #[must_use]
    pub fn new(reserved: impl IntoIterator<Item = String>) -> Self {
        Self {
            counter: 0,
            map: std::collections::HashMap::new(),
            reserved: reserved.into_iter().collect(),
        }
    }

    /// Mangle a name (parity: mangleName). If mangling disabled path: pass-through via
    /// `mangle(name, false)`.
    pub fn mangle(&mut self, name: &str, enabled: bool) -> String {
        if !enabled {
            return name.to_string();
        }
        if self.reserved.contains(name) {
            return name.to_string();
        }
        if let Some(existing) = self.map.get(name) {
            return existing.clone();
        }
        let mangled = generate_mangled_name(self.counter);
        self.counter += 1;
        self.map.insert(name.to_string(), mangled.clone());
        mangled
    }

    #[must_use]
    pub fn counter(&self) -> usize {
        self.counter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sequence_a_to_aa() {
        assert_eq!(generate_mangled_name(0), "a");
        assert_eq!(generate_mangled_name(1), "b");
        assert_eq!(generate_mangled_name(25), "z");
        assert_eq!(generate_mangled_name(26), "aa");
        assert_eq!(generate_mangled_name(27), "ab");
    }

    #[test]
    fn mangler_stable_and_reserved() {
        let mut m = NameMangler::new(["window".into(), "document".into()]);
        assert_eq!(m.mangle("foo", true), "a");
        assert_eq!(m.mangle("bar", true), "b");
        assert_eq!(m.mangle("foo", true), "a");
        assert_eq!(m.mangle("window", true), "window");
        assert_eq!(m.mangle("baz", false), "baz");
        assert_eq!(m.counter(), 2);
    }
}
