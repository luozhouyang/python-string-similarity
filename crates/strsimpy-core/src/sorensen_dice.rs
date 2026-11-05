//! Sorensen-Dice coefficient over shingle sets.

use std::collections::HashSet;

use crate::shingle;

/// Sorensen-Dice similarity: `2 * |V0 inter V1| / (|V0| + |V1|)`.
///
/// Two empty profiles (for example, strings shorter than `k`) yield a
/// similarity of 0.0 rather than a division by zero.
pub fn similarity(s0: &str, s1: &str, k: usize) -> f64 {
    if s0 == s1 {
        return 1.0;
    }
    let profile0 = shingle::profile(s0, k);
    let profile1 = shingle::profile(s1, k);
    let denominator = profile0.len() + profile1.len();
    if denominator == 0 {
        return 0.0;
    }
    let union: HashSet<&String> = profile0.keys().chain(profile1.keys()).collect();
    let intersection = profile0.len() + profile1.len() - union.len();
    2.0 * intersection as f64 / denominator as f64
}

/// `1.0 - similarity(s0, s1, k)`.
pub fn distance(s0: &str, s1: &str, k: usize) -> f64 {
    1.0 - similarity(s0, s1, k)
}

/// A [`SorensenDice`] similarity calculator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SorensenDice {
    k: usize,
}

impl SorensenDice {
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

impl Default for SorensenDice {
    fn default() -> Self {
        Self::new(3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_values() {
        assert!((similarity("上海", "上海市", 2) - 2.0 / 3.0).abs() < 1e-12);
        assert!((distance("上海", "上海市", 2) - 1.0 / 3.0).abs() < 1e-12);
    }

    #[test]
    fn empty_profiles_do_not_divide_by_zero() {
        assert_eq!(similarity("", "a", 3), 0.0);
        assert_eq!(distance("", "a", 3), 1.0);
        assert_eq!(similarity("a", "b", 3), 0.0);
    }

    #[test]
    fn identical_strings() {
        assert_eq!(similarity("", "", 3), 1.0);
        assert_eq!(similarity("Shanghai", "Shanghai", 3), 1.0);
    }
}
