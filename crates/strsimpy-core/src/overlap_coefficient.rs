//! Overlap coefficient (Szymkiewicz-Simpson) over shingle sets.

use std::collections::HashSet;

use crate::shingle;

/// Overlap coefficient: `|V0 inter V1| / min(|V0|, |V1|)`.
pub fn similarity(s0: &str, s1: &str, k: usize) -> f64 {
    if s0 == s1 {
        return 1.0;
    }
    let profile0 = shingle::profile(s0, k);
    let profile1 = shingle::profile(s1, k);
    if profile0.is_empty() || profile1.is_empty() {
        return 0.0;
    }
    let union: HashSet<&String> = profile0.keys().chain(profile1.keys()).collect();
    let intersection = profile0.len() + profile1.len() - union.len();
    intersection as f64 / profile0.len().min(profile1.len()) as f64
}

/// `1.0 - similarity(s0, s1, k)`.
pub fn distance(s0: &str, s1: &str, k: usize) -> f64 {
    1.0 - similarity(s0, s1, k)
}

/// An [`OverlapCoefficient`] similarity calculator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OverlapCoefficient {
    k: usize,
}

impl OverlapCoefficient {
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

impl Default for OverlapCoefficient {
    fn default() -> Self {
        Self::new(3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subset_has_similarity_one() {
        assert_eq!(similarity("eat", "eating", 3), 1.0);
        assert_eq!(distance("eat", "eating", 3), 0.0);
    }

    #[test]
    fn half_similar_example() {
        assert_eq!(similarity("car", "bar", 2), 0.5);
        assert_eq!(distance("car", "bar", 2), 0.5);
    }

    #[test]
    fn empty_profiles_after_normalization() {
        assert_eq!(similarity("   ", "eating", 3), 0.0);
        assert_eq!(distance("   ", "eating", 3), 1.0);
        assert_eq!(similarity("   ", "   ", 3), 1.0);
    }
}
