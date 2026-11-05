//! Normalized Levenshtein distance and similarity.

use crate::levenshtein;

/// Normalized Levenshtein distance: `levenshtein(s0, s1) / max(len(s0), len(s1))`.
///
/// The result lies in `[0, 1]`.
pub fn distance(s0: &str, s1: &str) -> f64 {
    if s0 == s1 {
        return 0.0;
    }
    let max_len = s0.chars().count().max(s1.chars().count());
    if max_len == 0 {
        return 0.0;
    }
    levenshtein::distance(s0, s1) / max_len as f64
}

/// `1.0 - distance(s0, s1)`.
pub fn similarity(s0: &str, s1: &str) -> f64 {
    1.0 - distance(s0, s1)
}

/// A [`NormalizedLevenshtein`] calculator.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct NormalizedLevenshtein;

impl NormalizedLevenshtein {
    pub fn new() -> Self {
        Self
    }

    pub fn distance(&self, s0: &str, s1: &str) -> f64 {
        distance(s0, s1)
    }

    pub fn similarity(&self, s0: &str, s1: &str) -> f64 {
        similarity(s0, s1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalized_values() {
        assert_eq!(distance("", ""), 0.0);
        assert_eq!(distance("", "上海"), 1.0);
        assert_eq!(distance("上海", "上海市"), 1.0 / 3.0);
        assert_eq!(distance("kitten", "sitting"), 3.0 / 7.0);
    }

    #[test]
    fn similarity_complements_distance() {
        for (a, b) in [("", ""), ("", "abc"), ("abc", "abd"), ("上海", "上海市")] {
            assert!((similarity(a, b) - (1.0 - distance(a, b))).abs() < f64::EPSILON);
        }
    }

    #[test]
    fn struct_delegates_to_functions() {
        let metric = NormalizedLevenshtein::new();
        assert_eq!(metric.distance("abc", "abd"), 1.0 / 3.0);
        assert!((metric.similarity("abc", "abd") - 2.0 / 3.0).abs() < 1e-12);
    }
}
