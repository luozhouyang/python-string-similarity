//! k-shingle (n-gram) profile construction.
//!
//! Profiles follow the Python implementation: whitespace runs are first
//! collapsed to a single ASCII space, then every window of `k` characters is
//! counted. Shingles are keyed by strings so profiles can be passed back to
//! Python as ordinary dictionaries.

use std::collections::HashMap;

/// Returns true for characters matched by Python's `\s` in a `str` pattern.
pub(crate) fn is_python_whitespace(c: char) -> bool {
    c.is_whitespace() || ('\u{1c}'..='\u{1f}').contains(&c)
}

/// Python's `re.sub(r"\s+", " ", string)`.
pub fn normalize_whitespace(string: &str) -> String {
    let mut normalized = String::with_capacity(string.len());
    let mut in_whitespace = false;
    for c in string.chars() {
        if is_python_whitespace(c) {
            if !in_whitespace {
                normalized.push(' ');
                in_whitespace = true;
            }
        } else {
            normalized.push(c);
            in_whitespace = false;
        }
    }
    normalized
}

/// Python's `str.split()` (whitespace runs, no empty tokens).
pub fn split_whitespace(string: &str) -> Vec<&str> {
    let mut words = Vec::new();
    let mut start = None;
    for (idx, c) in string.char_indices() {
        if is_python_whitespace(c) {
            if let Some(s) = start.take() {
                words.push(&string[s..idx]);
            }
        } else if start.is_none() {
            start = Some(idx);
        }
    }
    if let Some(s) = start {
        words.push(&string[s..]);
    }
    words
}

/// Builds the shingle profile of `string` for shingle length `k`.
pub fn profile(string: &str, k: usize) -> HashMap<String, usize> {
    let normalized = normalize_whitespace(string);
    let chars: Vec<char> = normalized.chars().collect();
    let mut profile: HashMap<String, usize> = HashMap::new();

    if k == 0 {
        profile.insert(String::new(), chars.len() + 1);
        return profile;
    }
    if chars.len() < k {
        return profile;
    }

    for window in chars.windows(k) {
        let shingle: String = window.iter().collect();
        *profile.entry(shingle).or_insert(0) += 1;
    }
    profile
}

/// A k-shingle profile builder.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShingleBased {
    k: usize,
}

impl ShingleBased {
    pub fn new(k: usize) -> Self {
        Self { k }
    }

    pub fn k(&self) -> usize {
        self.k
    }

    pub fn profile(&self, string: &str) -> HashMap<String, usize> {
        profile(string, self.k)
    }
}

impl Default for ShingleBased {
    fn default() -> Self {
        Self::new(3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn whitespace_is_collapsed() {
        assert_eq!(normalize_whitespace("a  b\t\nc"), "a b c");
        assert_eq!(normalize_whitespace("   "), " ");
        assert_eq!(normalize_whitespace("\u{1c}\u{1d}"), " ");
    }

    #[test]
    fn profile_counts_shingles() {
        let p = profile("aaaa", 2);
        assert_eq!(p.get("aa"), Some(&3));
        assert_eq!(p.len(), 1);

        let p = profile("abc", 3);
        assert_eq!(p.get("abc"), Some(&1));

        let p = profile("ab", 3);
        assert!(p.is_empty());
    }

    #[test]
    fn profile_handles_unicode() {
        let p = profile("上海市", 2);
        assert_eq!(p.get("上海"), Some(&1));
        assert_eq!(p.get("海市"), Some(&1));
    }

    #[test]
    fn whitespace_normalization_happens_before_shingling() {
        let p = profile("a   b", 3);
        assert_eq!(p.get("a b"), Some(&1));
    }

    #[test]
    fn split_whitespace_matches_python() {
        assert_eq!(split_whitespace("  hello\tworld\n"), vec!["hello", "world"]);
        assert!(split_whitespace(" \t\n").is_empty());
    }
}
