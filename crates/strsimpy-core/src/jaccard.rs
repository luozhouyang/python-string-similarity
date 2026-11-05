//! Jaccard index over shingle sets.

use std::collections::HashSet;

use crate::shingle;

/// Jaccard similarity: `|V0 inter V1| / |V0 union V1|`.
pub fn similarity(s0: &str, s1: &str, k: usize) -> f64 {
    if s0 == s1 {
        return 1.0;
    }
    if s0.chars().count() < k || s1.chars().count() < k {
        return 0.0;
    }
    let profile0 = shingle::profile(s0, k);
    let profile1 = shingle::profile(s1, k);
    let union: HashSet<&String> = profile0.keys().chain(profile1.keys()).collect();
    let intersection = profile0.len() + profile1.len() - union.len();
    1.0 * intersection as f64 / union.len() as f64
}

/// `1.0 - similarity(s0, s1, k)`.
pub fn distance(s0: &str, s1: &str, k: usize) -> f64 {
    1.0 - similarity(s0, s1, k)
}

/// A [`Jaccard`] similarity calculator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Jaccard {
    k: usize,
}

impl Jaccard {
    pub fn new(k: usize) -> Self {
        Self { k }
    }

    pub fn k(&self) -> usize {
        self.k
    }

    pub fn similarity(&self, s0: &str, s1: &str) -> f64 {
        similarity(s0, s1, self.k)
    }

    pub fn distance(&self, s0: &str, s1: &str) -> f64 {
        distance(s0, s1, self.k)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_and_short_strings() {
        assert_eq!(similarity("Shanghai", "Shanghai", 1), 1.0);
        assert_eq!(similarity("", "", 1), 1.0);
        assert_eq!(similarity("a", "b", 3), 0.0);
    }

    #[test]
    fn known_values() {
        assert!((similarity("Shanghai", "Shanghai", 1) - 1.0).abs() < 1e-12);
        assert!((similarity("Shang  Hai", "Shang Hai", 2) - 1.0).abs() < 1e-12);
        assert!((similarity("night", "nacht", 2) - 1.0 / 7.0).abs() < 1e-12);
        assert!((distance("night", "nacht", 2) - 6.0 / 7.0).abs() < 1e-12);
    }

    #[test]
    fn whitespace_is_normalized() {
        assert_eq!(similarity("   ", "   ", 3), 1.0);
        assert_eq!(similarity("   ", "eating", 3), 0.0);
    }
}
